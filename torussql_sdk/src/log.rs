// TorusSQL - simple relational database management system.
// Copyright (C) 2025-2026 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Logging macros.

/// Custom log output.
///
/// # Parameters
/// - `title` - given custom log title.
#[macro_export]
macro_rules! custom {
    ($title:expr, $($arg:tt)*) => {{
        let local_time = chrono::Local::now();
        let timestamp  = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

        print!("[{}] [{}]: ", timestamp, $title);
        print!("{}\n", format_args!($($arg)*));
    }};
}

/// Informational log output.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        crate::log::custom!("INFO", $($arg)*)
    }};
}

/// Debug log output.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        crate::log::custom!("DEBUG", $($arg)*)
    }};
}

/// Error log output.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        crate::log::custom!("ERROR", $($arg)*)
    }};
}

// Re-export macro rules.
pub use custom;
pub use info;
pub use debug;
pub use error;