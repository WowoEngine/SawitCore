#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate rlibc;

pub mod drivers;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;
pub mod sawitdb;


pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { 
        let mut pics = interrupts::PICS.lock();
        pics.initialize();
        
        // Manually mask IRQ 10 and 11 on Slave PIC (Port 0xA1)
        // IRQ 8-15 are on Slave. 
        // IRQ 10 = Slave 2 (1<<2 = 4)
        // IRQ 11 = Slave 3 (1<<3 = 8)
        // Mask = 4 | 8 = 12 (0x0C)
        // ENABLE IRQ 11 for testing: Mask only 10 (0x04)
        use x86_64::instructions::port::Port;
        let mut start_mask: u8 = 0;
        let mut data_port = Port::<u8>::new(0xA1);
        start_mask = data_port.read();
        // Clear bit 3 (IRQ 11) -> 0xF7. Set bit 2 (IRQ 10) -> 0x04.
        // But better: Just Don't mask 11.
        // Let's preserve existing mask but ensure 11 is 0.
        // Actually, Pic8259 init sets all to masked? No, it usually masks all.
        // We need to UNMASK 11.
        // pics.write_masks() isn't public.
        // pics.notify_end_of_interrupt is.
        // pics.initialize() masks all? usually not.
        
        // Let's explicitly Unmask 11 using Port
        // Read current
        let current = data_port.read();
        // Unmask IRQ 11 (bit 3) and IRQ 2 (Cascade? No, Slave is IRQ 2 on Master).
        // Slave IRQ 3 = Pin 3 on Slave.
        // Unmask bit 3: AND !8.
        data_port.write(current & !(1<<3));
    };
    x86_64::instructions::interrupts::enable();
}


