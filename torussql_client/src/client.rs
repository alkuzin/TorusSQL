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
    let mut buffer = [0; 1];        // Buffer to store symbol from keyboard.
    let mut input  = String::new(); // User input buffer.

    print!("{PROMPT}");
    stdout().flush().unwrap();

    loop {
        // Read symbol from keyboard.
        let _ = stdin().read_exact(&mut buffer);
        stdout().flush().unwrap();

        if buffer[0] == key::ESC {
            handle_arrow_keys(&mut buffer);
        }
        else if buffer[0] == key::BACKSPACE {
            handle_backspace(&mut input);
        }
        else if buffer[0] == key::TAB {
            handle_tab(&mut input);
        }
        else if buffer[0] == key::ENTER {
            handle_enter(&mut input);
        }
        else if is_ctrl(buffer[0] as i32) {
            let to_break = handle_ctrl(buffer[0]);

            if to_break {
                break;
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

/// Handle arrow keys.
///
/// # Parameters
/// - `buffer` - given keyboard key buffer.
fn handle_arrow_keys(buffer: &mut [u8]) {
    let _ = stdin().read_exact(buffer);

    // Check whether it is arrow keys.
    if buffer[0] == key::CSI {
        let _ = stdin().read_exact(buffer);

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

/// Handle backspace key.
///
/// # Parameters
/// - `input` - given user input buffer.
fn handle_backspace(input: &mut String) {
    // Handle clearing symbols.
    if !input.is_empty() {
        input.pop();
        print!("\r{PROMPT}{}", input);
        print!(" ");
        print!("\r{PROMPT}{}", input);
    }
    stdout().flush().unwrap();
}

/// Handle tab key.
///
/// # Parameters
/// - `input` - given user input buffer.
fn handle_tab(input: &mut String) {
    // Remove extra whitespaces.
    *input = input.trim().to_string();

    // Autocomplete meta-command.
    if meta::is_command(&input) {
        let suggestions = meta::find_closest_commands(&input);

        match suggestions.len() {
            0 => return,
            1 => {
                print!("\r{PROMPT}");

                for _ in 0..input.len() {
                    print!(" ");
                }

                input.clear();
                *input = format!(":{}", suggestions[0].clone());

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

/// Handle enter key.
///
/// # Parameters
/// - `input` - given user input buffer.
fn handle_enter(input: &mut String) {
    print!("\n");

    // Remove extra whitespaces.
    *input = input.trim().to_string();

    // Skip if input is empty.
    if input.is_empty() {
        print!("{PROMPT}");
        stdout().flush().unwrap();
        input.clear();
        return;
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

/// Handle CTRL+KEY/CTRL+SHIFT+KEY keys.
///
/// # Parameters
/// - `symbol` - given keyboard symbol.
///
/// # Returns
/// - `true`  - flag signaling to exit the program.
/// - `false` - flag signaling to not exit the program.
fn handle_ctrl(symbol: u8) -> bool {
    // Handle CTRL + <KEY> / CTRL + SHIFT + <KEY>.
    let symbol = (symbol + 'A' as u8 - 1) as char;

    match symbol {
        // Exit program.
        'C' => {
            print!("\n");
            true
        },
        _ => false,
    }
}