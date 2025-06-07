// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! SQL parser related declarations.

use crate::compiler::lexer::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::compiler::lexer::Lexer;

    #[test]
    fn test_parser() {
        let lexer = Lexer::new("CREATE DATABASE \"MyDB\"");
    }
}