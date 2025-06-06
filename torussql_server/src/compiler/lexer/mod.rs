// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! SQL lexer related declarations.

pub mod token;

use crate::log;
use std::{iter::Peekable, str::Chars};
use token::{Keyword, Token};

/// Struct that converts SQL code into tokens.
pub struct Lexer<'a> {
    /// SQL code set of chars.
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// Construct new `Lexer` object.
    ///
    /// # Parameters
    /// - `input` - given SQL code.
    ///
    /// # Returns
    /// - New `Lexer` object.
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    /// Get next token.
    ///
    /// # Returns
    /// - `SQL token`  - in case of success.
    /// - `Token::End` - in case of reaching end of SQL code.
    /// - `None`       - in case of failure.
    pub fn next_token(&mut self) -> Option<Token> {
        // Skip whitespaces.
        if let Some(current_char) = self.input.peek() {
            if current_char.is_whitespace() {
                self.skip_whitespace();
            }
        }

        // Handle characters.
        if let Some(c) = self.input.peek() {
            let token = match c {
                c if c.is_alphabetic() => self.consume_keyword_or_ident(),
                '"' => self.consume_string(),
                _ => self.consume_symbol(),
            };

            self.advance();
            return token;
        }

        // End of SQL code was reached.
        Some(Token::End)
    }

    /// Skip space characters.
    fn skip_whitespace(&mut self) {
        while self.input.peek().is_some()
            && self.input.peek().unwrap().is_whitespace()
        {
            self.advance()
        }
    }

    /// Advance current character position.
    #[inline(always)]
    fn advance(&mut self) {
        self.input.next();
    }

    /// Consume keyword or ident token.
    ///
    /// # Returns
    ///  - `SQL token` - in case of success.
    ///  - `None`      - otherwise.
    fn consume_keyword_or_ident(&mut self) -> Option<Token> {
        let mut value = String::new();

        // Extract keyword/ident from input.
        while let Some(&c) = self.input.peek() {
            if c.is_alphabetic() {
                value.push(c);
                self.advance()
            } else {
                break;
            }
        }

        // Handle empty string.
        if value.is_empty() {
            log::error!("Can't convert to token");
            return None;
        }

        log::debug!("Found value: \"{}\"", value);

        // Try to convert to SQL keyword.
        let result = Keyword::try_from(value.as_str());

        match result {
            Ok(keyword) => {
                log::debug!("Found keyword: {}", keyword);
                Some(Token::Keyword(keyword))
            }
            Err(_) => {
                // Convert to ident token.
                Some(Token::String(value))
            }
        }
    }

    /// Consume string literal.
    ///
    /// # Returns
    ///  - `SQL token` - in case of success.
    ///  - `None`      - otherwise.
    fn consume_string(&mut self) -> Option<Token> {
        // Skip '"' symbol.
        self.advance();
        let mut value = String::new();

        while let Some(c) = self.input.peek() {
            if *c == '"' {
                // Consume the closing quote.
                self.advance();
                break;
            }

            value.push(*c);
            self.advance();
        }

        if !value.is_empty() {
            log::debug!("Found literal string: \"{value}\"");
            return Some(Token::String(value));
        }

        None
    }

    /// Consume special symbol.
    ///
    /// # Returns
    ///  - `SQL token` - in case of success.
    ///  - `None`      - otherwise.
    fn consume_symbol(&mut self) -> Option<Token> {
        if let Some(c) = self.input.peek() {
            let token = match c {
                ';' => Token::Semicolon,
                _ => return None,
            };

            log::debug!("Found symbol: '{}'", c);
            return Some(token);
        }

        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::compiler::lexer::Keyword::*;

    #[test]
    fn test_next_token() {
        let input = "     CREATE     DATABASE    \"MyDB\"      ;     ";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Keyword(Create)));

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Keyword(Database)));

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::String("MyDB".to_string())));

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Semicolon));

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::End));

        // Check that end was reached again.
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::End));
    }

    #[test]
    fn test_next_token_different_case() {
        let input = "     CreAtE     DATAbase    \"  MyDB  \"      ;     ";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Keyword(Create)));

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Keyword(Database)));

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::String("  MyDB  ".to_string())));

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::Semicolon));

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::End));

        // Check that end was reached again.
        let token = lexer.next_token();
        assert_eq!(token, Some(Token::End));
    }

    #[test]
    fn test_parse_empty_string() {
        let input = "     ";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert_eq!(token, Some(Token::End));
    }
}
