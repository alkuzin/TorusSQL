// Project name: TorusSQL.
// Description: Relational database management system.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! TorusSQL terminal related declarations.

use libc::{
    ECHO, ICANON, STDIN_FILENO, TCSANOW, tcgetattr, tcsetattr, termios,
};

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
    pub const ESC: u8 = 27;
    pub const CSI: u8 = 91;
    pub const UP_ARROW: u8 = 65;
    pub const DOWN_ARROW: u8 = 66;
    pub const BACKSPACE: u8 = 127;
    pub const ENTER: u8 = 13;
    pub const TAB: u8 = 9;
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
