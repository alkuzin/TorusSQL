// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! SQL tokens related declarations.

use std::{
    fmt::{Display, Formatter},
    convert::TryFrom
};

/// SQL token types enumeration.
#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    String(String),
    Semicolon,
    End,
}

/// SQL keywords enumeration.
#[derive(Debug, PartialEq)]
pub enum Keyword {
    Create,
    Database,
}

impl TryFrom<&str> for Keyword {
    // TODO: replace with TorusSQL error enum.
    type Error = &'static str;

    /// Try to convert string to SQL keyword.
    ///
    /// # Parameters
    /// - `value` - given string value to convert.
    ///
    /// # Returns
    /// - `SQL keyword` - in case of success.
    /// - `Err`         - otherwise.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Make string lowercase.
        let lowercase_value = value.to_lowercase();
        let value = lowercase_value.as_str();

        let result = match value {
            "create" => Self::Create,
            "database" => Self::Database,
            _ => return Err("Not a keyword"),
        };

        Ok(result)
    }
}

impl Display for Keyword {
    /// Display SQL keyword.
    ///
    /// # Parameters
    /// - `f` - given formatter.
    ///
    /// # Returns
    /// - `OK`  - in case of success.
    /// - `Err` - otherwise.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Keyword::Create => "CREATE",
            Keyword::Database => "DATABASE",
        };

        f.write_str(result)
    }
}
