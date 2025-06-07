// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! SQL Abstract Syntax Tree (AST) related declarations.

/// SQL language types enumeration.
#[derive(Debug, PartialEq)]
pub enum LanguageType {
    /// Data Definition Language - defines and manages database objects.
    DDL,
    /// Data Manipulation Language - manages data within database objects.
    DML,
    /// Data Control Language - controls access to data.
    DCL,
    /// Transaction Control Language - manages transactions.
    TCL,
    /// Data Query Language - queries data from the database.
    DQL,
    /// Vendor-specific language.
    Vendor,
}

/// Struct that describes the syntactic structure of a SQL statement.
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// Create a new database.
    CreateDatabase {
        /// Database name.
        name: String,
    },
}

impl Statement {
    /// Get SQL language type of this SQL statement.
    ///
    /// # Returns
    /// - SQL language type.
    pub fn language_type(&self) -> LanguageType {
        match self {
            Statement::CreateDatabase { .. } => LanguageType::DDL,
        }
    }
}
