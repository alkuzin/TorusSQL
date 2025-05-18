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

use std::io::{stdout, Write};
use crate::{log, meta};

/// TorusSQL client shell prompt.
const PROMPT: &str = "torussql> ";

/// Run client.
pub fn run() {
    println!("TorusSQL v{}", env!("CARGO_PKG_VERSION"));
    println!("Print ':help' to see list of available commands.");

    let mut input = String::new();

    loop {
        print!("{}", PROMPT);

        if let Err(e) = stdout().flush() {
            log::error!("Error to flush stdout: {e}");
            break;
        }

        // Read and handle user input.
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Remove extra whitespaces.
                input = input.trim().to_string();

                // Skip if input is empty.
                if input.is_empty() {
                    continue;
                }

                // Check whether input is meta-command or SQL query.
                if meta::is_command(&input) {
                    // TODO: add autocomplete for meta-commands.
                    // TODO: add meta-commands history.
                    meta::handle_command(&input);
                }
                else {
                    // TODO: check whether it is correct query or not.
                    log::debug!("Entered: '{input}'");
                }

                input.clear();
            }
            Err(e) => {
                log::error!("Error to read input: {e}");
            }
        }
    }
}