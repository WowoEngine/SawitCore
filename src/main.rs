#![no_std]
#![no_main]

use sawitcore_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // println!("Hello World!");
    println!("SawitCore OS.");
    
    sawitcore_os::init();

    // println!("It did not crash!");
    // println!("Keyboard interrupts are enabled. Type something:");

    #[allow(clippy::empty_loop)]
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
