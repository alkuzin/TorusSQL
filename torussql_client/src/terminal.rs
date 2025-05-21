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

//! TorusSQL terminal related declarations.

use libc::{tcgetattr, tcsetattr, termios, ECHO, ICANON, STDIN_FILENO, TCSANOW};

/// Set raw mode of terminal.
///
/// # Returns
/// - Terminal attributes before setting raw mode.
pub fn set_raw_mode() -> termios {
    unsafe {
        // Get current terminal attributes.
        let mut terminal: termios = std::mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut terminal);

        // Save old terminal attributes in order to restore it later.
        let old_terminal = terminal.clone();

        // Disable echo and canonical mode.
        terminal.c_lflag = !(ECHO | ICANON);
        tcsetattr(STDIN_FILENO, TCSANOW, &terminal);

        old_terminal
    }
}

/// Set terminal to canonical mode.
///
/// # Parameter
/// - `terminal` - given canonical mode terminal attributes.
pub fn reset_terminal(terminal: termios) {
    unsafe { tcsetattr(0, TCSANOW, &terminal) };
}

/// Number of spaces per tab.
pub const TAB_SIZE: usize = 4;

/// Keyboard keys module.
pub mod key {
    pub const ESC: u8        = 27;
    pub const CSI: u8        = 91;
    pub const UP_ARROW: u8   = 65;
    pub const DOWN_ARROW: u8 = 66;
    pub const BACKSPACE: u8  = 127;
    pub const ENTER: u8      = 13;
    pub const TAB: u8        = 9;
}

/// Check whether given symbol is CTRL/CTRL+SHIFT.
///
/// # Parameters
/// - `symbol` - given symbol code to check.
///
/// # Returns
/// - `true`  - if given symbol is CTRL/CTRL+SHIFT.
/// - `false` - otherwise.
#[inline(always)]
pub fn is_ctrl(symbol: i32) -> bool {
    symbol >= 1 && symbol <= 26
}
