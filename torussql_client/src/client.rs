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

use libc::{tcgetattr, tcsetattr, termios, ECHO, ICANON, STDIN_FILENO, TCSANOW};
use std::io::{stdin, stdout, Read, Write};
use crate::{log, meta};

/// TorusSQL client shell prompt.
const PROMPT: &str = "torussql> ";

/// Run client.
pub fn run() {
    let old_terminal = terminal_set_raw_mode();

    println!("TorusSQL v{}", env!("CARGO_PKG_VERSION"));
    println!("Print ':help' to see list of available commands.");

    // TODO: declare commands here & pass it to read_input() as argument.

    let _ = std::panic::catch_unwind(|| {
        read_input();
    });

    // TODO: save commands history to file before exit.

    reset_terminal(old_terminal);
}

// TODO: move terminal-related code to other module.

const TAB_SIZE: usize = 4;

#[repr(u8)]
enum Key {
    Unknown   = 0,
    Esc       = 27,
    Csi       = 91,
    UpArrow   = 65,
    DownArrow = 66,
    Backspace = 127,
    Enter     = 13,
    Tab       = 9,
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value {
            27  => Key::Esc,
            91  => Key::Csi,
            65  => Key::UpArrow,
            66  => Key::DownArrow,
            127 => Key::Backspace,
            9   => Key::Tab,
            13  => Key::Enter,
            _   => Key::Unknown,
        }
    }
}

/// Read and handle user input.
fn read_input() {
    // TODO: fix issue with: readline: warning: turning off output flushing.
    // TODO: description comments.
    // TODO: move key handlers to separate functions.
    let mut buffer = [0; 1];
    let mut input  = String::new();

    print!("{PROMPT}");
    stdout().flush().unwrap();

    const ESC: u8 = Key::Esc as u8;
    const CSI: u8 = Key::Csi as u8;
    const UP_ARROW: u8 = Key::UpArrow as u8;
    const DOWN_ARROW: u8 = Key::DownArrow as u8;
    const BACKSPACE: u8 = Key::Backspace as u8;
    const ENTER: u8 = Key::Enter as u8;
    const TAB: u8 = Key::Tab as u8;

    loop {
        // Read symbol from keyboard.
        let _ = stdin().read_exact(&mut buffer);
        stdout().flush().unwrap();

        if buffer[0] == ESC {
            let _ = stdin().read_exact(&mut buffer);

            // Check whether it is arrow keys.
            if buffer[0] == CSI {
                let _ = stdin().read_exact(&mut buffer);

                match buffer[0] {
                    UP_ARROW => {
                        // TODO: retrieve last command from history.
                        log::debug!("UP ARROW");
                    }
                    DOWN_ARROW => {
                        // TODO: retrieve next command from history.
                        log::debug!("DOWN ARROW");
                    }
                    _ => {}
                }
            }
        }
        else if buffer[0] == BACKSPACE {
            // Handle clearing symbols.
            if !input.is_empty() {
                input.pop();
                print!("\r{PROMPT}{}", input);
                print!(" ");
                print!("\r{PROMPT}{}", input);
            }
            stdout().flush().unwrap();
        }
        else if buffer[0] == TAB {
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
        else if buffer[0] == ENTER {
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

/// Check whether given symbol is CTRL/CTRL+SHIFT.
///
/// # Parameters
/// - `symbol` - given symbol code to check.
///
/// # Returns
/// - `true`  - if given symbol is CTRL/CTRL+SHIFT.
/// - `false` - otherwise.
#[inline(always)]
fn is_ctrl(symbol: i32) -> bool {
    symbol >= 1 && symbol <= 26
}

/// Set raw mode of terminal.
///
/// # Returns
/// - Terminal attributes before setting raw mode.
fn terminal_set_raw_mode() -> termios {
    unsafe {
        // Get current terminal attributes.
        let mut terminal: termios = std::mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut terminal);

        // Save old terminal attributes in order to restore it later.
        let old_terminal = terminal.clone();

        // Disable echo and canonical mode.
        terminal.c_lflag = !(ECHO | ICANON);
        tcsetattr(STDIN_FILENO, TCSANOW, &terminal);

        old_terminal
    }
}

/// Set terminal to canonical mode.
///
/// # Parameter
/// - `terminal` - given canonical mode terminal attributes.
fn reset_terminal(terminal: termios) {
    unsafe { tcsetattr(0, TCSANOW, &terminal) };
}