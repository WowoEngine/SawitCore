#![no_std]
#![no_main]

use sawitcore_os::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use sawitcore_os::memory::BootInfoFrameAllocator;
use sawitcore_os::allocator;
use x86_64::{VirtAddr};

extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // println!("Hello World!");
    println!("SawitCore OS.");
    
    sawitcore_os::init();

    // println!("BootInfo: {:#?}", boot_info);
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let phys_mem_offset = VirtAddr::new(0);
    let mut mapper = unsafe { sawitcore_os::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let mut executor = sawitcore_os::task::simple_executor::SimpleExecutor::new();
    executor.spawn(sawitcore_os::task::Task::new(sawitcore_os::task::shell::shell_task()));
    executor.run();

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
