// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! SQL DDL related commands code generation declarations.

use super::language_type_to_bytecode;
use crate::compiler::codegen::{Bytecode, statement_to_bytecode};
use crate::compiler::parser::ast::{LanguageType, Statement};

/// Generate bytecode for inner virtual machine.
///
/// # Parameters
/// - `bytecode`  - given bytecode to store.
/// - `statement` - given SQL statement.
pub fn generate_bytecode(bytecode: &mut Bytecode, statement: &Statement) {
    // Bytecode header.
    bytecode.push(language_type_to_bytecode(LanguageType::DDL));
    bytecode.push(statement_to_bytecode(statement));

    // Handle different types of SQL statements.
    match statement {
        Statement::CreateDatabase { name } => {
            generate_create_database(bytecode, name)
        }
    }
}

/// Generate bytecode CREATE DATABASE statement.
///
/// # Parameters
/// - `bytecode` - given bytecode to store.
/// - `name`     - given database name.
fn generate_create_database(bytecode: &mut Bytecode, name: &String) {
    // TODO: add bytecode for "IF NOT EXISTS".

    // Generate byte code for database name.
    bytecode.push(name.len() as u8);

    for b in name.bytes() {
        bytecode.push(b);
    }
}
