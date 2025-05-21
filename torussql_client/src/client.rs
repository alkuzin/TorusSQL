// TorusSQL - simple relational database management system.
// Copyright (C) 2025-2026 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! TorusSQL client related declarations.

use crate::{log, meta, terminal::{key, TAB_SIZE, is_ctrl, reset_terminal, set_raw_mode}};
use std::io::{stdin, stdout, Read, Write};

/// TorusSQL client shell prompt.
const PROMPT: &str = "torussql> ";

/// Run client.
pub fn run() {
    let old_terminal = set_raw_mode();

    println!("TorusSQL v{}", env!("CARGO_PKG_VERSION"));
    println!("Print ':help' to see list of available commands.");

    // TODO: declare commands here & pass it to read_input() as argument.

    let _ = std::panic::catch_unwind(|| {
        read_input();
    });

    // TODO: save commands history to file before exit.
    reset_terminal(old_terminal);
}

/// Read and handle user input.
fn read_input() {
    // TODO: fix issue with: readline: warning: turning off output flushing.
    // TODO: move key handlers to separate functions.
    let mut buffer = [0; 1];        // Buffer to store symbol from keyboard.
    let mut input  = String::new(); // User input buffer.

    print!("{PROMPT}");
    stdout().flush().unwrap();

    loop {
        // Read symbol from keyboard.
        let _ = stdin().read_exact(&mut buffer);
        stdout().flush().unwrap();

        if buffer[0] == key::ESC {
            let _ = stdin().read_exact(&mut buffer);

            // Check whether it is arrow keys.
            if buffer[0] == key::CSI {
                let _ = stdin().read_exact(&mut buffer);

                match buffer[0] {
                    key::UP_ARROW => {
                        // TODO: retrieve last command from history.
                        log::debug!("UP ARROW");
                    }
                    key::DOWN_ARROW => {
                        // TODO: retrieve next command from history.
                        log::debug!("DOWN ARROW");
                    }
                    _ => {}
                }
            }
        }
        else if buffer[0] == key::BACKSPACE {
            // Handle clearing symbols.
            if !input.is_empty() {
                input.pop();
                print!("\r{PROMPT}{}", input);
                print!(" ");
                print!("\r{PROMPT}{}", input);
            }
            stdout().flush().unwrap();
        }
        else if buffer[0] == key::TAB {
            // Remove extra whitespaces.
            input = input.trim().to_string();

            // Autocomplete meta-command.
            if meta::is_command(&input) {
                let suggestions = meta::find_closest_commands(&input);

                match suggestions.len() {
                    0 => continue,
                    1 => {
                        print!("\r{PROMPT}");

                        for _ in 0..input.len() {
                            print!(" ");
                        }

                        input.clear();
                        input = format!(":{}", suggestions[0].clone());

                        print!("\r{PROMPT}{input}");
                        stdout().flush().unwrap();
                    },
                    _ => {
                        print!("\n");

                        for i in suggestions {
                            print!("{i}\t");
                        }

                        print!("\n{PROMPT}{input}");
                        stdout().flush().unwrap();
                    },
                }
            }
            else {
                // Add tab after input.
                for _ in 0..TAB_SIZE {
                    input.push(' ');
                    print!(" ");
                }
            }
        }
        else if buffer[0] == key::ENTER {
            print!("\n");

            // Remove extra whitespaces.
            input = input.trim().to_string();

            // Skip if input is empty.
            if input.is_empty() {
                continue;
            }

            // Check whether input is meta-command or SQL query.
            if meta::is_command(&input) {
                // TODO: add meta-commands history.
                meta::handle_command(&input);
            }
            else {
                // TODO: check whether it is correct query or not.
                log::debug!("Entered: '{input}'");
            }

            print!("{PROMPT}");
            stdout().flush().unwrap();
            input.clear();
        }
        else if is_ctrl(buffer[0] as i32) {
            // Handle CTRL + <KEY> / CTRL + SHIFT + <KEY>.
            let symbol = (buffer[0] + 'A' as u8 - 1) as char;

            match symbol {
                // Exit program.
                'C' => {
                    print!("\n");
                    break;
                },
                _ => {},
            }
        }
        else {
            // Display symbol on the screen & add it to the input buffer.
            let symbol = buffer[0] as char;
            print!("{symbol}");
            input.push(symbol);

            stdout().flush().unwrap();
        }
    }
}