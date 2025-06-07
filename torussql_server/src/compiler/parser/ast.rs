// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! SQL Abstract Syntax Tree (AST) related declarations.

/// Struct that describes the syntactic structure of a SQL statement.
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// Create a new database.
    CreateDatabase {
        /// Database name.
        name: String,
    },
}
