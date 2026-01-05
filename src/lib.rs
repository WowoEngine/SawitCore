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
        use x86_64::instructions::port::Port;
        let mut start_mask: u8 = 0;
        let mut data_port = Port::<u8>::new(0xA1);
        start_mask = data_port.read();
        data_port.write(start_mask | 0x0C);
    };
    x86_64::instructions::interrupts::enable();
}


