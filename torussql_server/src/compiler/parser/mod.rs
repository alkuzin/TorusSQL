// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! SQL parser related declarations.

pub mod ast;

use crate::compiler::{
    lexer::{
        Lexer,
        token::{Keyword, Token},
    },
    parser::ast::Statement,
};
use torussql_sdk::log;

/// SQL statements parser struct.
pub struct Parser<'a> {
    /// SQL lexer.
    lexer: Lexer<'a>,
    /// Current token to handle.
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    /// Construct new `Parser` object.
    ///
    /// # Parameters
    /// - `lexer` - given SQL lexer.
    ///
    /// # Returns
    /// - New `Parser` object.
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
        };

        parser.next_token();
        parser
    }

    /// Get next token.
    ///
    /// # Returns
    /// - `SQL token`  - in case of success.
    /// - `Token::End` - in case of reaching end of SQL code.
    /// - `None`       - in case of failure.
    #[inline(always)]
    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    /// Parse SQL statement.
    ///
    /// # Returns
    /// - `SQL statement` - in case of success.
    /// - `None`          - in case of failure.
    pub fn parse(&mut self) -> Option<Statement> {
        if let Some(token) = &self.current_token {
            log::debug!("Token: {:?}", token);

            return match token {
                Token::Keyword(keyword) => match keyword {
                    // Handle CREATE statement.
                    Keyword::Create => self.parse_create(),
                    _ => None,
                },
                _ => None,
            };
        };

        None
    }

    /// Parse create statement.
    ///
    /// # Returns
    /// - `SQL statement` - in case of success.
    /// - `None`          - in case of failure.
    fn parse_create(&mut self) -> Option<Statement> {
        // Handle next token.
        self.next_token();

        if let Some(Token::Keyword(keyword)) = &self.current_token {
            return match keyword {
                // Handle CREATE DATABASE statement.
                Keyword::Database => self.parse_create_database(),
                _ => return None,
            };
        }

        None
    }

    /// Parse create database statement.
    ///
    /// # Returns
    /// - `SQL statement` - in case of success.
    /// - `None`          - in case of failure.
    fn parse_create_database(&mut self) -> Option<Statement> {
        // Get database name.
        self.next_token();

        if let Some(Token::String(name)) = &self.current_token {
            return Some(Statement::CreateDatabase {
                name: name.to_string(),
            });
        }

        None
    }
}

#[cfg(test)]
pub mod tests {
    use crate::compiler::{
        lexer::Lexer,
        parser::{Parser, ast::Statement},
    };
    use torussql_sdk::log;

    fn create_parser(input: &str) -> Parser {
        let lexer = Lexer::new(input);
        Parser::new(lexer)
    }

    // TODO: add TorusSQL errors.
    // TODO: add more tests for CREATE DATABASE statement.
    #[test]
    fn test_create_database() {
        let mut parser = create_parser("CREATE DATABASE \"MyDB\";");
        let statement = parser.parse().unwrap();

        let correct_statement = Statement::CreateDatabase {
            name: "MyDB".to_string(),
        };

        log::debug!("Statement: {:?}", statement);
        assert_eq!(statement, correct_statement);
    }
}
