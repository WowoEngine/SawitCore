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

    // --- SAWITI DB VERIFICATION ---
    use alloc::boxed::Box;
    use alloc::string::String;
    use alloc::vec;
    use sawitcore_os::drivers::block::RamDisk;
    use sawitcore_os::sawitdb::pager::Pager;
    use sawitcore_os::sawitdb::btree::BTreeIndex;
    use sawitcore_os::sawitdb::types::Value;

    println!("");
    println!("[SawitDB] Starting Verification Tests...");

    // 1. Storage & Pager Test
    // Use 64KB RamDisk to fit in heap
    let ramdisk = Box::new(RamDisk::new(64 * 1024));
    println!("[SawitDB] RamDisk Created (64KB)");

    match Pager::new(ramdisk) {
        Ok(mut pager) => {
             println!("[SawitDB] Pager Initialized");
             if let Ok(p0) = pager.read_page(0) {
                 if &p0[0..4] == b"WOWO" {
                     println!("[SawitDB] Pager Magic Verified: WOWO");
                 } else {
                     println!("[SawitDB] Error: Invalid Magic");
                 }
             }
             match pager.alloc_page() {
                 Ok(pid) => println!("[SawitDB] Page Allocated: ID {}", pid),
                 Err(_) => println!("[SawitDB] Error Allocating Page"),
             }
        },
        Err(_) => println!("[SawitDB] Failed to init Pager"),
    }
    
    // 2. BTree Index Test
    println!("[SawitDB] Testing BTree Index...");
    let mut btree = BTreeIndex::new(4, String::from("users"), String::from("id"));
    
    println!("[SawitDB] Inserting Keys...");
    btree.insert(Value::Int(10), Value::String(String::from("Alice")));
    btree.insert(Value::Int(5), Value::String(String::from("Bob")));
    btree.insert(Value::Int(20), Value::String(String::from("Charlie")));
    btree.insert(Value::Int(15), Value::String(String::from("Dave")));
    btree.insert(Value::Int(2), Value::String(String::from("Eve")));
    
    println!("[SawitDB] Searching Keys...");
    let search_keys = vec![5, 15, 99];
    for k in search_keys {
        let key_val = Value::Int(k);
        let results = btree.search(&key_val);
        if !results.is_empty() {
             println!("[SawitDB] Found Key {}: {}", k, results[0]);
        } else {
             println!("[SawitDB] Key {} Not Found", k);
        }
    }
    println!("[SawitDB] Tests Completed.");
    println!(""); 
    // --- END TESTS ---

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
