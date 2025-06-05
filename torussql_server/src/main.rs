// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! TorusSQL server entry point.

pub mod sql;

use torussql_sdk::log;

fn main() {
    log::info!("Running TorusSQL server");
}
