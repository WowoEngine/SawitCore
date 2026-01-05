use smoltcp::iface::{Interface, Config, SocketSet};
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, HardwareAddress};
use smoltcp::phy::{Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;
use alloc::vec;
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;
use virtio_drivers::device::net::{VirtIONet, TxBuffer, RxBuffer};
use crate::drivers::virtio_transport::LegacyPciTransport;
use crate::drivers::virtio_hal::VirtioHal;
use crate::println;

// Global Network Interface
lazy_static! {
    pub static ref NET_IFACE: Mutex<Option<Interface>> = Mutex::new(None);
    pub static ref NET_DEVICE: Mutex<Option<VirtioNetWrapper>> = Mutex::new(None);
    pub static ref SOCKETS: Mutex<SocketSet<'static>> = Mutex::new(SocketSet::new(vec![]));
}

pub static VIRTIO_IO_BASE: core::sync::atomic::AtomicU16 = core::sync::atomic::AtomicU16::new(0);

// Wrapper
pub struct VirtioNetWrapper {
    driver: VirtIONet<VirtioHal, LegacyPciTransport, 16>,
}

unsafe impl Send for VirtioNetWrapper {}

impl VirtioNetWrapper {
    pub fn new(transport: LegacyPciTransport) -> Self {
        let driver = VirtIONet::new(transport, 2048).expect("Failed to init VirtIONet");
        VirtioNetWrapper { driver }
    }
}

impl Device for VirtioNetWrapper {
    type RxToken<'a> = VirtioRxToken where Self: 'a;
    type TxToken<'a> = VirtioTxToken<'a> where Self: 'a;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        // Manually ACK/Clear ISR since we masked the IRQ in PIC
        // This ensures the device doesn't stall thinking an interrupt is pending
        let io_base = VIRTIO_IO_BASE.load(core::sync::atomic::Ordering::Relaxed);
        if io_base != 0 {
             unsafe { Port::<u8>::new(io_base + 19).read(); }
        }

        match self.driver.receive() {
            Ok(rx_buffer) => {
                crate::serial_println!("RX Pkt: Len {}", rx_buffer.packet_len());
                Some((VirtioRxToken(rx_buffer), VirtioTxToken(&mut self.driver)))
            },
            Err(_) => None,
        }
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        crate::serial_println!("TX Token Requested");
        Some(VirtioTxToken(&mut self.driver))
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = 1500;
        caps
    }
}

pub struct VirtioRxToken(RxBuffer);
impl smoltcp::phy::RxToken for VirtioRxToken {
    fn consume<R, F>(mut self, f: F) -> R
    where F: FnOnce(&mut [u8]) -> R {
        f(self.0.packet_mut())
    }
}

pub struct VirtioTxToken<'a>(&'a mut VirtIONet<VirtioHal, LegacyPciTransport, 16>);
impl<'a> smoltcp::phy::TxToken for VirtioTxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where F: FnOnce(&mut [u8]) -> R {
        let mut buffer = vec![0u8; len];
        let result = f(&mut buffer);
        self.0.send(TxBuffer::from(&buffer)).ok(); 
        result
    }
}


// PCI Scanning
const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;

unsafe fn pci_read(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let address = 0x80000000 | ((bus as u32) << 16) | ((slot as u32) << 11) | ((func as u32) << 8) | ((offset as u32) & 0xFC);
    let mut addr_port = Port::<u32>::new(PCI_CONFIG_ADDRESS);
    let mut data_port = Port::<u32>::new(PCI_CONFIG_DATA);
    addr_port.write(address);
    data_port.read()
}

unsafe fn pci_write(bus: u8, slot: u8, func: u8, offset: u8, value: u32) {
    let address = 0x80000000 | ((bus as u32) << 16) | ((slot as u32) << 11) | ((func as u32) << 8) | ((offset as u32) & 0xFC);
    let mut addr_port = Port::<u32>::new(PCI_CONFIG_ADDRESS);
    let mut data_port = Port::<u32>::new(PCI_CONFIG_DATA);
    addr_port.write(address);
    data_port.write(value);
}

pub fn init() {
    println!("Scanning PCI Bus...");
    for bus in 0..=0 {
        for slot in 0..32 {
            let vendor_id = unsafe { pci_read(bus, slot, 0, 0) } & 0xFFFF;
            if vendor_id == 0x1AF4 {
                let device_id = (unsafe { pci_read(bus, slot, 0, 0) } >> 16) & 0xFFFF;
                // 0x1000 is Legacy Net, 0x1041 is Modern Net?
                // QEMU -device virtio-net-pci usually provides 0x1000 with legacy/modern.
                if device_id == 0x1000 || (device_id >= 0x1040 && device_id <= 0x107F) {
                     println!("Found VirtIO Device ID {:#x} at {}:{}:0", device_id, bus, slot);


                     // 1. Find IO BAR first (don't enable device yet)
                     let mut io_base: u32 = 0;
                     for bar_off in [0x10, 0x14, 0x18, 0x1C, 0x20, 0x24] {
                         let bar = unsafe { pci_read(bus, slot, 0, bar_off) };
                         // Bit 0 = 1 -> IO
                         if bar & 1 == 1 && bar != 0 {
                             io_base = bar & 0xFFFFFFFC; // Mask last 2 bits
                             crate::serial_println!("Found IO BAR at {:#x}", io_base);
                             break;
                         }
                     }
                     
                     let irq = unsafe { pci_read(bus, slot, 0, 0x3C) } & 0xFF;
                     crate::serial_println!("Found Legacy VirtIO at {}:{}:{} (IRQ {})", bus, slot, 0, irq);

                     if io_base != 0 {
                         // 2. Set Atomic IO Base for Interrupt Handler
                         VIRTIO_IO_BASE.store(io_base as u16, core::sync::atomic::Ordering::SeqCst);
                         
                         // 3. Enable Bus Master (Bit 2), Memory (Bit 1), IO (Bit 0)
                         let command = unsafe { pci_read(bus, slot, 0, 0x04) };
                         unsafe { pci_write(bus, slot, 0, 0x04, command | 0x7) };

                         // 4. Init Transport
                         crate::serial_println!("Initializing Driver with LegacyPciTransport...");
                         
                         // Create transport
                         let transport = LegacyPciTransport::new(io_base as u16);
                         
                         let driver = VirtIONet::new(transport, 2048).expect("VirtIONet init failed");
                         let mut wrapper = VirtioNetWrapper { driver };

                         // Setup Interface
                         let mac = wrapper.driver.mac_address();
                         crate::serial_println!("[Net] MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]);
                         
                         let ethernet_addr = EthernetAddress::from_bytes(&mac);
                         let ip_addr = IpCidr::new(IpAddress::v4(10, 0, 2, 15), 24);
                         
                         let mut config = Config::new();
                         config.hardware_addr = Some(HardwareAddress::Ethernet(ethernet_addr));
                         config.random_seed = 0x1234; 
                         
                         let mut iface = Interface::new(config, &mut wrapper);
                         iface.update_ip_addrs(|ip_addrs| {
                             ip_addrs.push(ip_addr).ok();
                         });
                         
                         *NET_DEVICE.lock() = Some(wrapper);
                         *NET_IFACE.lock() = Some(iface);
                         
                         crate::serial_println!("[Net] Interface Ready: 10.0.2.15/24");
                         return; // Initialized finding one is enough
                     }
                }
            }
        }
    }
    println!("[Net] No VirtIO Network Device Found.");
}
