// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! SQL code generation related declarations.

pub mod ddl;

use crate::compiler::parser::Parser;
use crate::compiler::parser::ast::{LanguageType, Statement};
use crate::log;

// TODO: implement OpCode & BytecodeInstruction structs.
// TODO: implement trait that implements method to_bytecode().
// Use it for Statement.

/// Bytecode type alias.
pub type Bytecode = Vec<u8>;

/// Struct responsible for generation of bytecode for
/// custom TorusSQL virtual machine.
pub struct CodeGen<'a> {
    /// SQL statements parser.
    parser: Parser<'a>,
    /// SQL statement bytecode.
    bytecode: Bytecode,
}

impl<'a> CodeGen<'a> {
    /// Construct new `CodeGen` object.
    ///
    /// # Parameters
    /// - `parser` - given SQL statements parser.
    ///
    /// # Returns
    /// - New `CodeGen` object.
    pub fn new(parser: Parser<'a>) -> Self {
        Self {
            parser,
            bytecode: Bytecode::with_capacity(64),
        }
    }

    /// Generate bytecode for inner virtual machine.
    ///
    /// # Returns
    /// - `Bytecode` - in case of success.
    /// - `None`     - in case of failure.
    pub fn generate_bytecode(&mut self) -> Option<Bytecode> {
        if let Some(statement) = self.parser.parse() {
            let language_type = statement.language_type();

            log::debug!("Statement: {:?}", statement);
            log::debug!("Language type: {:?}", language_type);

            match language_type {
                LanguageType::DDL => {
                    ddl::generate_bytecode(&mut self.bytecode, &statement)
                }
                LanguageType::DML => return None,
                LanguageType::DCL => return None,
                LanguageType::TCL => return None,
                LanguageType::DQL => return None,
                LanguageType::Vendor => return None,
            };

            return Some(self.bytecode.clone());
        }

        None
    }
}

/// Convert SQL language type to bytecode unit.
///
/// # Parameters
/// - `lang_type` - given SQL language type to convert.
///
/// # Returns
/// - `Bytecode unit representation of SQL language type`.
pub const fn language_type_to_bytecode(lang_type: LanguageType) -> u8 {
    match lang_type {
        LanguageType::DDL => 0x01,
        LanguageType::DML => 0x02,
        LanguageType::DCL => 0x03,
        LanguageType::TCL => 0x04,
        LanguageType::DQL => 0x05,
        LanguageType::Vendor => 0x06,
    }
}

/// Convert bytecode unit to SQL language type.
///
/// # Parameters
/// - `byte` - given bytecode unit to convert.
///
/// # Returns
/// - `SQL language type representation of bytecode unit` - in case of success.
/// - `None` - in case of failure.
pub const fn bytecode_to_language_type(byte: u8) -> Option<LanguageType> {
    match byte {
        0x01 => Some(LanguageType::DDL),
        0x02 => Some(LanguageType::DML),
        0x03 => Some(LanguageType::DCL),
        0x04 => Some(LanguageType::TCL),
        0x05 => Some(LanguageType::DQL),
        0x06 => Some(LanguageType::Vendor),
        _ => None,
    }
}

/// Convert SQL statement to bytecode unit.
///
/// # Parameters
/// - `statement` - given SQL statement to convert.
///
/// # Returns
/// - `Bytecode unit representation of SQL statement`.
pub const fn statement_to_bytecode(statement: &Statement) -> u8 {
    match statement {
        Statement::CreateDatabase { .. } => 0x01,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::compiler::codegen::CodeGen;
    use crate::compiler::lexer::Lexer;
    use crate::compiler::parser::Parser;
    use torussql_sdk::log;

    fn create_codegen(input: &str) -> CodeGen {
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        CodeGen::new(parser)
    }

    #[test]
    fn test_codegen_create_database() {
        let mut codegen = create_codegen("CREATE DATABASE \"MyDB\";");
        let bytecode = codegen.generate_bytecode().unwrap();

        log::debug!("Bytecode: {:X?}", bytecode);
    }
}
