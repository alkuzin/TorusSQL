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

use crate::log;

/// Builtin meta-command info struct.
struct MetaCommand {
    /// Command name.
    name: &'static str,
    /// Command purpose description.
    description: &'static str,
    /// Command function handler.
    handler: fn(&Vec<&str>) -> (),
}

/// Array of builtin meta-commands.
static COMMANDS: [MetaCommand;4] = [
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
    MetaCommand {
        name: "exec",
        description: "Execute SQL from file specified file",
        handler: exec,
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
    let input: Vec<_> = (&input[1..]).trim().split(" ").collect();
    let command_name = input[0];

    log::debug!("Handle command: {:?}", input);

    // Try to find command in commands array.
    for command in &COMMANDS {
        if command.name == command_name {
            (command.handler)(&input);
            return;
        }
    }

    // Handle unknown command.
    // TODO: replace with Result<>.
    log::debug!("Unknown meta-command: '{command_name}'");
}

/// Display list of available meta-commands.
pub fn help(_: &Vec<&str>) {
    for command in &COMMANDS {
        println!(":{:<10} {}", command.name, command.description);
    }
}

/// Exit TorusSQL client.
pub fn exit(_: &Vec<&str>) {
    log::debug!("Exiting TorusSQL client");
    std::process::exit(0);
}

/// Display TorusSQL version and additional info.
pub fn version(_: &Vec<&str>) {
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    println!("TorusSQL v{version}\nAuthors: {authors}");
}

/// Execute SQL from file specified file.
pub fn exec(args: &Vec<&str>) {
    if args.len() != 2 {
        log::error!("Incorrect number of arguments");
        // TODO: print error for user.
        // TODO: print usage example for user.
        // TODO: add usage example for MetaCommand.
        return;
    }

    // TODO: check whether given path is correct.
    let path = args[1];

    log::debug!("File: '{path}'");
}
