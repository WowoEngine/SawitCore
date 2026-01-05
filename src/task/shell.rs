use crate::{print, println};
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use alloc::string::String;

pub async fn shell_task() {
    let mut scancodes = super::keyboard::ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
        HandleControl::Ignore);
    
    let mut line_buffer = String::new();
    let mut in_management_mode = false;
    
    // SawitDB State
    use crate::sawitdb::btree::BTreeIndex;
    use crate::sawitdb::types::Value;
    // We only support one table for now in this simple shell
    let mut active_table: Option<BTreeIndex> = None;

    print!("Sawit> ");

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        match character {
                            '\n' => {
                                println!();
                                let command_line = line_buffer.trim();
                                if command_line == "manage" {
                                    in_management_mode = true;
                                    print_management_menu();
                                } else if command_line == "exit" && in_management_mode {
                                    in_management_mode = false;
                                    println!("Exited management mode.");
                                } else if command_line == "help" {
                                    println!("Available commands:");
                                    println!("  manage  - Open SawitDB Management Menu");
                                    println!("  help    - Show this help");
                                    println!("  clear   - Clear screen");
                                } else if command_line == "clear" {
                                     // Poor man's clear
                                     for _ in 0..25 { println!(); }
                                } else if !command_line.is_empty() {
                                    if in_management_mode {
                                        // Management Commands
                                        let parts: alloc::vec::Vec<&str> = command_line.split_whitespace().collect();
                                        match parts[0] {
                                            "help" => print_management_menu(),
                                            "meminfo" => {
                                                use crate::allocator::{HEAP_SIZE, HEAP_START};
                                                println!("OS Memory Info:");
                                                println!("  Heap Start: {:#x}", HEAP_START);
                                                println!("  Heap Size:  {} KiB", HEAP_SIZE / 1024);
                                                // We can't easily get used/free from LockedHeap without modifying it
                                                println!("  Status:     Initialized");
                                            },
                                            "db_init" => {
                                                if parts.len() < 2 {
                                                    println!("Usage: db_init <table_name>");
                                                } else {
                                                    let name = String::from(parts[1]);
                                                    active_table = Some(BTreeIndex::new(4, name.clone(), String::from("id")));
                                                    println!("Table '{}' initialized.", name);
                                                }
                                            },
                                            "put" => {
                                                // put <key_int> <val_str>
                                                if let Some(ref mut table) = active_table {
                                                    if parts.len() < 3 {
                                                        println!("Usage: put <key_int> <val_string>");
                                                    } else {
                                                        if let Ok(k) = parts[1].parse::<i64>() {
                                                            let val_str = String::from(parts[2]); // Take first word as val
                                                            // Re-join rest if needed? For now simple single word
                                                            table.insert(Value::Int(k), Value::String(val_str));
                                                            println!("Inserted.");
                                                        } else {
                                                            println!("Error: Key must be integer");
                                                        }
                                                    }
                                                } else {
                                                    println!("Error: No table active. Run 'db_init <table>'");
                                                }
                                            },
                                            "get" => {
                                                if let Some(ref table) = active_table {
                                                    if parts.len() < 2 {
                                                        println!("Usage: get <key_int>");
                                                    } else {
                                                        if let Ok(k) = parts[1].parse::<i64>() {
                                                             let results = table.search(&Value::Int(k));
                                                             if results.is_empty() {
                                                                 println!("Not Found.");
                                                             } else {
                                                                 // Print first result
                                                                 println!("Found: {:?}", results[0]);
                                                             }
                                                        } else {
                                                            println!("Error: Key must be integer");
                                                        }
                                                    }
                                                } else {
                                                    println!("Error: No table active.");
                                                }
                                            },
                                            _ => println!("Unknown command '{}'. Type 'help' for menu.", parts[0]),
                                        }
                                        
                                    } else {
                                        println!("Unknown command: '{}'", command_line);
                                    }
                                }
                                
                                line_buffer.clear();
                                if !in_management_mode {
                                    print!("Sawit> ");
                                } else {
                                    print!("SawitDB> ");
                                }
                            }
                            '\x08' => { // Backspace
                                if !line_buffer.is_empty() {
                                    line_buffer.pop();
                                    print!("{}", '\x08'); 
                                }
                            }
                            _ => {
                                print!("{}", character);
                                line_buffer.push(character);
                            }
                        }
                    },
                    DecodedKey::RawKey(_key) => {},
                }
            }
        }
    }
}

fn print_management_menu() {
    println!("\n--- SawitDB Management ---");
    println!("meminfo           - Show Memory Stats");
    println!("db_init <table>   - Create new Table Index");
    println!("put <key> <val>   - Insert Data (Key=Int)");
    println!("get <key>         - Query Data");
    println!("exit              - Return to Shell");
}
