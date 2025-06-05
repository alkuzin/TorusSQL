// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! TorusSQL client entry point.

mod client;
mod meta;
mod terminal;

use torussql_sdk::log;

fn main() {
    // TODO: setup special files & directories.
    // In case of errors stop program.
    client::run();
}
