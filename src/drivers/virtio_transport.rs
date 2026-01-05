use virtio_drivers::{Error, PhysAddr};
use virtio_drivers::transport::{Transport, DeviceType, DeviceStatus};
use x86_64::instructions::port::Port;
use core::ptr::NonNull;
use core::result::Result;

pub struct LegacyPciTransport {
    io_addr: u16,
    config_shadow: [u8; 64], // Enough for VirtioNetConfig
}

impl LegacyPciTransport {
    pub fn new(io_addr: u16) -> Self {
        let mut t = Self { 
            io_addr,
            config_shadow: [0; 64],
        };
        // Read config space (offset 20) into shadow
        // Config space size depends on device, Net is small.
        for i in 0..64 {
             let mut port = Port::<u8>::new(io_addr + 20 + i);
             t.config_shadow[i as usize] = unsafe { port.read() };
        }
        t
    }
}

// Legacy Layout Offsets
const OFFSET_DEVICE_FEATURES: u16 = 0;
const OFFSET_GUEST_FEATURES: u16 = 4;
const OFFSET_QUEUE_ADDR: u16 = 8;
const OFFSET_QUEUE_SIZE: u16 = 12;
const OFFSET_QUEUE_SELECT: u16 = 14;
const OFFSET_QUEUE_NOTIFY: u16 = 16;
const OFFSET_STATUS: u16 = 18;
const OFFSET_ISR: u16 = 19;

impl Transport for LegacyPciTransport {
    // ... implementations ...
    
    // Copy-paste previous methods...
    fn device_type(&self) -> DeviceType { DeviceType::Network }
    fn read_device_features(&mut self) -> u64 {
        let mut port = Port::<u32>::new(self.io_addr + OFFSET_DEVICE_FEATURES);
        let f = unsafe { port.read() as u64 };
        // crate::serial_println!("Relay Features Read: {:#x}", f);
        f
    }
    fn write_driver_features(&mut self, driver_features: u64) {
        let mut port = Port::<u32>::new(self.io_addr + OFFSET_GUEST_FEATURES);
        crate::serial_println!("VirtIO Features Negotiated: {:#x}", driver_features);
        unsafe { port.write(driver_features as u32) }
    }
    fn max_queue_size(&self) -> u32 {
        let mut port = Port::<u16>::new(self.io_addr + OFFSET_QUEUE_SIZE);
        unsafe { port.read() as u32 }
    }
    fn notify(&mut self, queue: u16) {
        let mut port = Port::<u16>::new(self.io_addr + OFFSET_QUEUE_NOTIFY);
        unsafe { port.write(queue) }
    }
    fn get_status(&self) -> DeviceStatus {
        let mut port = Port::<u8>::new(self.io_addr + OFFSET_STATUS);
        let status = unsafe { port.read() };
        DeviceStatus::from_bits_truncate(status as u32)
    }
    fn set_status(&mut self, status: DeviceStatus) {
        let mut port = Port::<u8>::new(self.io_addr + OFFSET_STATUS);
        unsafe { port.write(status.bits() as u8) }
    }
    fn set_guest_page_size(&mut self, _guest_page_size: u32) { }
    fn requires_legacy_layout(&self) -> bool { true }

    fn queue_set(&mut self, queue: u16, _size: u32, descriptors: PhysAddr, _driver_area: PhysAddr, _device_area: PhysAddr) {
        let mut port_sel = Port::<u16>::new(self.io_addr + OFFSET_QUEUE_SELECT);
        unsafe { port_sel.write(queue) };
        let pfn = (descriptors / 4096) as u32;
        crate::serial_println!("VirtIO Queue {}: PFN {:#x} (Addr {:#x})", queue, pfn, descriptors);
        let mut port_pfn = Port::<u32>::new(self.io_addr + OFFSET_QUEUE_ADDR);
        unsafe { port_pfn.write(pfn) };
    }
    fn queue_unset(&mut self, _queue: u16) {
        let mut port_addr = Port::<u32>::new(self.io_addr + OFFSET_QUEUE_ADDR);
        unsafe { port_addr.write(0) };
    }
    fn queue_used(&mut self, queue: u16) -> bool {
        let mut port_sel = Port::<u16>::new(self.io_addr + OFFSET_QUEUE_SELECT);
        unsafe { port_sel.write(queue) };
        let mut port_addr = Port::<u32>::new(self.io_addr + OFFSET_QUEUE_ADDR);
        unsafe { port_addr.read() != 0 }
    }
    fn ack_interrupt(&mut self) -> bool {
        let mut port = Port::<u8>::new(self.io_addr + OFFSET_ISR);
        let isr = unsafe { port.read() };
        isr & 1 != 0
    }

    fn config_space<T: 'static>(&self) -> Result<NonNull<T>, Error> {
        // Return pointer to shadow
        let ptr = self.config_shadow.as_ptr() as *mut T;
        NonNull::new(ptr).ok_or(Error::ConfigSpaceMissing)
    }
}
