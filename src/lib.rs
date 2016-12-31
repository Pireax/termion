//! Termion is a pure Rust, bindless library for low-level handling, manipulating
//! and reading information about terminals. This provides a full-featured
//! alternative to Termbox.
//!
//! Termion aims to be simple and yet expressive. It is bindless, meaning that it
//! is not a front-end to some other library (e.g., ncurses or termbox), but a
//! standalone library directly talking to the TTY.
//!
//! Supports Redox, Mac OS X, and Linux (or, in general, ANSI terminals).
//!
//! For more information refer to the [README](https://github.com/ticki/termion).
#![warn(missing_docs)]

#[cfg(not(target_os = "redox"))]
extern crate libc;

#[cfg(target_os = "windows")]
extern crate winapi;
extern crate kernel32;

#[cfg(not(any(target_os = "redox", target_os = "windows")))]
mod termios;

mod async;
pub use async::{AsyncReader, async_stdin};

mod size;
pub use size::terminal_size;

mod tty;
pub use tty::{is_tty, get_tty};

#[macro_use]
mod macros;
pub mod clear;
pub mod color;
pub mod cursor;
pub mod event;
pub mod input;
pub mod raw;
pub mod scroll;
pub mod style;

/// Does nothing on non windows targets.
#[cfg(not(target_os = "windows"))]
pub fn init() { }

/// Sets console to use windows 10's vterm processing.
#[cfg(target_os = "windows")]
pub fn init() {
    use kernel32::{GetStdHandle, GetConsoleMode, SetConsoleMode};
    use winapi::winbase::{STD_OUTPUT_HANDLE, STD_ERROR_HANDLE, STD_INPUT_HANDLE};
    use winapi::minwindef::{DWORD, LPDWORD};

    let mut mode: DWORD = 0;
    unsafe {
        let h_out = GetStdHandle(STD_OUTPUT_HANDLE);
        GetConsoleMode(h_out, &mut mode as LPDWORD);
        mode |= 0x004; // ENABLE_VIRTUAL_TERMINAL_PROCESSING
        mode |= 0x008; // DISABLE_NEWLINE_AUTO_RETURN
        SetConsoleMode(h_out, mode);

        let h_in = GetStdHandle(STD_INPUT_HANDLE);
        GetConsoleMode(h_in, &mut mode as LPDWORD);
        mode |= 0x0200 // ENABLE_VIRTUAL_TERMINAL_INPUT
        SetConsoleMode(h_in, mode);
    }
}