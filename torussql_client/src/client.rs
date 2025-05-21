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
use std::{io::{self, stdin, stdout, BufRead, BufReader, Read, Write}, fs::OpenOptions};

// TODO: move consts into config module.
// TODO: add meta-command to change history limit & store all configs in config file.

/// TorusSQL client shell prompt.
const PROMPT: &str = "torussql> ";

/// User input size limit.
const INPUT_LIMIT: usize = 64;

/// User input history size limit.
const HISTORY_LIMIT: usize = 64;

/// Max number of characters per line.
const LINE_SIZE: usize = INPUT_LIMIT + PROMPT.len();

/// Special files directory path.
const DIRECTORY_PATH: &'static str = ".torussql";

/// User input history file path.
const HISTORY_PATH: &'static str = ".torussql/history";

/// Struct for handling client CLI.
pub struct Client {
    /// Buffer to store symbol from keyboard.
    buffer: [u8; 1],
    /// User input buffer.
    input: String,
    /// User input history.
    history: Vec<String>,
    /// Current user input history position.
    history_pos: usize,
}

impl Client {
    /// Construct new `Client` object.
    ///
    /// # Returns
    /// - New `Client` object.
    pub fn new() -> Self {
        let buffer      = [0; 1];
        let input       = String::with_capacity(INPUT_LIMIT);
        let history     = Vec::with_capacity(HISTORY_LIMIT);
        let history_pos = 0;

        Self { buffer, input, history, history_pos }
    }

    /// Read and handle user input.
    pub fn read_input(&mut self) {
        // TODO: fix issue with: readline: warning: turning off output flushing.
        // Read commands history from file.
        if let Err(e) = self.load_history() {
            log::error!("Error: {e}");
            return;
        }

        print!("{PROMPT}");
        stdout().flush().unwrap();

        loop {
            // Read symbol from keyboard.
            let _ = stdin().read_exact(&mut self.buffer);
            stdout().flush().unwrap();

            if self.buffer[0] == key::ESC {
                self.handle_arrow_keys();
            }
            else if self.buffer[0] == key::BACKSPACE {
               self.handle_backspace();
            }
            else if self.buffer[0] == key::TAB {
                self.handle_tab();
            }
            else if self.buffer[0] == key::ENTER {
                self.handle_enter();
            }
            else if is_ctrl(self.buffer[0] as i32) {
                let to_break = self.handle_ctrl(self.buffer[0]);

                if to_break {
                    break;
                }
            }
            else {
                // Display symbol on the screen & add it to the input buffer.
                let symbol = self.buffer[0] as char;
                print!("{symbol}");
                self.input.push(symbol);

                stdout().flush().unwrap();
            }
        }

        // Save commands history to file.
        if let Err(e) = self.save_history() {
            log::error!("Error: {e}");
        }
    }

    /// Load commands history from file.
    ///
    /// # Returns
    /// - `Ok`  - in case of success.
    /// - `Err` - otherwise.
    fn load_history(&mut self) -> io::Result<()> {
        // Create directory if it not exists.
        std::fs::create_dir_all(DIRECTORY_PATH)?;

        // Create the file if it does not exist.
        let file = OpenOptions::new()
            .create(true)           // Create the file if it doesn't exist.
            .write(true)            // Allow writing to the file.
            .read(true)             // Allow reading from the file.
            .open(HISTORY_PATH)?;

        // Check if the file is empty.
        let metadata = file.metadata()?;

        if metadata.len() == 0 {
            return Ok(());
        }

        // If the file exists and is not empty, read all commands.
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let command = line?;
            self.history.push(command);
        }

        self.history_pos = self.history.len();
        Ok(())
    }

    /// Save commands history to file.
    ///
    /// # Returns
    /// - `Ok`  - in case of success.
    /// - `Err` - otherwise.
    fn save_history(&mut self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(HISTORY_PATH)?;

        // Trim the history if it exceeds the limit.
        while self.history.len() > HISTORY_LIMIT {
            self.history.remove(0);
        }

        // Write commands to the file.
        for command in &self.history {
            writeln!(file, "{}", command)?;
        }

        Ok(())
    }


    /// Handle arrow keys.
    fn handle_arrow_keys(&mut self) {
        let _ = stdin().read_exact(&mut self.buffer);

        // Check whether it is arrow keys.
        if self.buffer[0] == key::CSI {
            let _ = stdin().read_exact(&mut self.buffer);

            // TODO: handle left and right arrow keys.
            match self.buffer[0] {
                key::UP_ARROW   => self.handle_up_arrow(),
                key::DOWN_ARROW => self.handle_down_arrow(),
                _ => {}
            }
        }
    }

    /// Handle up arrow key.
    fn handle_up_arrow(&mut self) {
        // Retrieve last command from history.
        if self.history.len() > 0 {
            if self.history_pos == 0 {
                // Do not change the position, if already at the top.
            }
            else {
                // Move up in history.
                self.history_pos -= 1;
            }

            // Update input with the command at the current history position.
            self.input = self.history[self.history_pos].clone();
        }

        // Clear line before updating input.
        print!("\r{PROMPT}{}", self.input);
        stdout().flush().unwrap();

        for _ in 0..LINE_SIZE / 4 {
            print!(" ");
        }

        print!("\r{PROMPT}{}", self.input);
        stdout().flush().unwrap();
    }

    /// Handle down arrow key.
    fn handle_down_arrow(&mut self) {
        // Retrieve next command from history.
        let len = self.history.len();

        if len > 0 {
            if self.history_pos >= len - 1 {
                // Do not change the position, if already at the bottom.
                self.input.clear();
                self.history_pos = len;
            }
            else {
                // Move down in history.
                self.history_pos += 1;
            }

            // Update input with the command at the current history position.
            if self.history_pos < len {
                self.input = self.history[self.history_pos].clone();
            }
        }

        // Clear line before updating input.
        print!("\r{PROMPT}{}", self.input);
        stdout().flush().unwrap();

        for _ in 0..LINE_SIZE / 4 {
            print!(" ");
        }

        print!("\r{PROMPT}{}", self.input);
        stdout().flush().unwrap();
    }

    /// Handle backspace key.
    fn handle_backspace(&mut self) {
        let input = &mut self.input;

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
    fn handle_tab(&mut self) {
        // Remove extra whitespaces.
        self.input = self.input.trim().to_string();

        // Autocomplete meta-command.
        if meta::is_command(&self.input) {
            let suggestions = meta::find_closest_commands(&self.input);

            match suggestions.len() {
                0 => return,
                1 => {
                    // Fill input with suggestion.
                    print!("\r{PROMPT}");

                    for _ in 0..self.input.len() {
                        print!(" ");
                    }

                    self.input.clear();
                    self.input = format!(":{}", suggestions[0].clone());

                    print!("\r{PROMPT}{}", self.input);
                    stdout().flush().unwrap();
                },
                _ => {
                    // Print list of suitable suggestions.
                    print!("\n");

                    for i in suggestions {
                        print!("{i}\t");
                    }

                    print!("\n{PROMPT}{}", self.input);
                    stdout().flush().unwrap();
                },
            }
        }
        else {
            // Add tab after input.
            for _ in 0..TAB_SIZE {
                self.input.push(' ');
                print!(" ");
            }
        }
    }

    /// Handle enter key.
    fn handle_enter(&mut self) {
        print!("\n");

        // Remove extra whitespaces.
        self.input = self.input.trim().to_string();
        let input  = &mut self.input;

        // Skip if input is empty.
        if input.is_empty() {
            print!("{PROMPT}");
            stdout().flush().unwrap();
            input.clear();
            return;
        }

        // Check whether input is meta-command or SQL query.
        if meta::is_command(&input) {
            self.history.push(input.to_string());
            meta::handle_command(&input);
        }
        else {
            self.history.push(input.to_string());
            // TODO: check whether it is correct query or not.
        }

        self.history_pos = self.history.len();

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
    fn handle_ctrl(&self, symbol: u8) -> bool {
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
}

/// Run client.
pub fn run() {
    let old_terminal = set_raw_mode();

    println!("TorusSQL v{}", env!("CARGO_PKG_VERSION"));
    println!("Print ':help' to see list of available commands.");

    let _ = std::panic::catch_unwind(|| {
        let mut client = Client::new();
        client.read_input();
    });

    reset_terminal(old_terminal);
}