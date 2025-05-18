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

//! TorusSQL meta-commands related declarations module.

use std::process::{self, Command};
use crate::log;

/// Builtin meta-command info struct.
struct MetaCommand {
    /// Command name.
    name: &'static str,
    /// Command purpose description.
    description: &'static str,
    /// Command function handler.
    handler: fn() -> (),
}

/// Array of builtin meta-commands.
static COMMANDS: [MetaCommand;3] = [
    MetaCommand {
        name: "help",
        description: "Display list of available meta-commands",
        handler: help,
    },
    MetaCommand {
        name: "exit",
        description: "Exit TorusSQL client",
        handler: exit,
    },
    MetaCommand {
        name: "version",
        description: "Display TorusSQL version and additional info",
        handler: version,
    },
];

/// Check whether input is meta-command.
///
/// # Parameters
/// - `input` - given user input string.
///
/// # Returns
/// - `true`  - if input is meta-command.
/// - `false` - otherwise.
#[inline(always)]
pub fn is_command(input: &String) -> bool {
    input.starts_with(":")
}

/// Handle meta-command.
///
/// # Parameters
/// - `input` - given user command.
pub fn handle_command(input: &String) {
    // Extract command & remove extra whitespaces.
    let input = (&input[1..]).trim();

    // Try to find command in commands array.
    for command in &COMMANDS {
        if command.name == input {
            (command.handler)();
            return;
        }
    }

    // Handle unknown command.
    // TODO: replace with Result<>.
    log::debug!("Unknown meta-command: '{input}'");
}

/// Display list of available meta-commands.
pub fn help() {
    for command in &COMMANDS {
        println!(":{:<10} {}", command.name, command.description);
    }
}

/// Exit TorusSQL client.
pub fn exit() {
    log::debug!("Exiting TorusSQL client");
    process::exit(0);
}

/// Get rustc compiler version info.
///
/// # Returns
/// - String representation of rustc version.
fn get_rustc_version() -> String {
    // Execute the rustc --version command.
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc");

    // Convert the output to a String.
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Display TorusSQL version and additional info.
pub fn version() {
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    let rustc_version = get_rustc_version();

    println!("TorusSQL v{version}\n{rustc_version}\nAuthors: {authors}");
}