// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! TorusSQL meta-commands related declarations module.

use crate::log;

/// Builtin meta-command info struct.
struct MetaCommand {
    /// Command name.
    name: &'static str,
    /// Command purpose description.
    description: &'static str,
    /// Command function handler.
    handler: fn(&Vec<&str>) -> bool,
}

/// Array of builtin meta-commands.
static COMMANDS: [MetaCommand; 4] = [
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
///
/// # Returns
/// - `true`  - if client process should be terminated.
/// - `false` - otherwise.
pub fn handle_command(input: &String) -> bool {
    // Extract command & remove extra whitespaces.
    let input: Vec<_> = (&input[1..]).trim().split(" ").collect();
    let command_name = input[0];

    log::debug!("Handle command: {:?}", input);

    // Try to find command in commands array.
    for command in &COMMANDS {
        if command.name == command_name {
            return (command.handler)(&input);
        }
    }

    // Handle unknown command.
    // TODO: replace with Result<> or custom error enum.
    log::debug!("Unknown meta-command: '{command_name}'");
    false
}

/// Function to find the closest commands based on current input.
///
/// # Parameters
/// - `input` - given user input to handle.
///
/// # Returns
/// - Vector of closest commands suggestions.
pub fn find_closest_commands(input: &str) -> Vec<String> {
    let input = input.trim();

    COMMANDS
        .iter()
        .filter(|command| command.name.starts_with(&input[1..]))
        .map(|command| command.name.to_string())
        .collect()
}

/// Display list of available meta-commands.
pub fn help(_: &Vec<&str>) -> bool {
    for command in &COMMANDS {
        println!(":{:<10} {}", command.name, command.description);
    }

    false
}

/// Exit TorusSQL client.
///
/// # Returns
/// - `true`  - if client process should be terminated.
/// - `false` - otherwise.
pub fn exit(_: &Vec<&str>) -> bool {
    log::debug!("Exiting TorusSQL client");
    true
}

/// Display TorusSQL version and additional info.
pub fn version(_: &Vec<&str>) -> bool {
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    println!("TorusSQL v{version}\nAuthors: {authors}");
    false
}

/// Execute SQL from file specified file.
pub fn exec(args: &Vec<&str>) -> bool {
    if args.len() != 2 {
        log::error!("Incorrect number of arguments");
        // TODO: print error for user.
        // TODO: print usage example for user.
        // TODO: add usage example for MetaCommand.
        return false;
    }

    // TODO: check whether given path is correct.
    let _path = args[1];

    log::debug!("File: '{_path}'");
    false
}
