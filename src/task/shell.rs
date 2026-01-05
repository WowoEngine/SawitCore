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

    print!("Sawit> ");

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        match character {
                            '\n' => {
                                println!();
                                if line_buffer.trim() == "manage" {
                                    in_management_mode = true;
                                    print_management_menu();
                                } else if line_buffer.trim() == "exit" && in_management_mode {
                                    in_management_mode = false;
                                    println!("Exited management mode.");
                                } else if line_buffer.trim() == "help" {
                                    println!("Available commands:");
                                    println!("  manage  - Open SawitDB Management Menu");
                                    println!("  help    - Show this help");
                                    println!("  clear   - Clear screen (simulated)");
                                } else if !line_buffer.trim().is_empty() {
                                    if in_management_mode {
                                        println!("Command '{}' not recognized in management mode.", line_buffer.trim());
                                        print_management_menu();
                                    } else {
                                        println!("Unknown command: '{}'", line_buffer.trim());
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
                                    // Hacky backspace handling for VGA buffer: move back, overwrite space, move back
                                    print!("{}", '\x08'); 
                                }
                            }
                            _ => {
                                print!("{}", character);
                                line_buffer.push(character);
                            }
                        }
                    },
                    DecodedKey::RawKey(_key) => {
                         // handle special keys if needed
                    },
                }
            }
        }
    }
}

fn print_management_menu() {
    println!("\n--- SawitDB Management ---");
    println!("1. [TODO] Create Database");
    println!("2. [TODO] View Tables");
    println!("3. [TODO] Query Data");
    println!("Type 'exit' to return to shell.");
}
