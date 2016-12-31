use std::io;

#[cfg(not(target_os = "redox"))]
#[cfg(not(target_os = "windows"))]
use libc::c_ushort;

#[cfg(not(target_os = "redox"))]
#[cfg(not(target_os = "windows"))]
#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    _x: c_ushort,
    _y: c_ushort,
}

// Since attributes on non-item statements is not stable yet, we use a function.
#[cfg(not(target_os = "android"))]
#[cfg(not(target_os = "redox"))]
#[cfg(not(target_os = "windows"))]
#[cfg(target_pointer_width = "64")]
#[cfg(not(target_env = "musl"))]
fn tiocgwinsz() -> u64 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as u64
}
#[cfg(not(target_os = "android"))]
#[cfg(not(target_os = "redox"))]
#[cfg(not(target_os = "windows"))]
#[cfg(target_pointer_width = "32")]
#[cfg(not(target_env = "musl"))]
fn tiocgwinsz() -> u32 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as u32
}

#[cfg(any(target_env = "musl", target_os = "android"))]
#[cfg(target_pointer_width = "32")]
fn tiocgwinsz() -> i32 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as i32
}

#[cfg(target_os = "android")]
#[cfg(target_pointer_width = "64")]
fn tiocgwinsz() -> i64 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as i64
}

/// Get the size of the terminal.
#[cfg(not(any(target_os = "redox", target_os = "windows")))]
pub fn terminal_size() -> io::Result<(u16, u16)> {
    use libc::ioctl;
    use libc::STDOUT_FILENO;

    use std::mem;
    unsafe {
        let mut size: TermSize = mem::zeroed();

        if ioctl(STDOUT_FILENO, tiocgwinsz(), &mut size as *mut _) == 0 {
            Ok((size.col as u16, size.row as u16))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Unable to get the terminal size."))
        }
    }
}

/// Get the size of the terminal.
#[cfg(target_os = "redox")]
pub fn terminal_size() -> io::Result<(u16, u16)> {
    use std::env;

    let width = try!(env::var("COLUMNS").map_err(|x| io::Error::new(io::ErrorKind::NotFound, x)))
        .parse()
        .unwrap_or(0);
    let height = try!(env::var("LINES").map_err(|x| io::Error::new(io::ErrorKind::NotFound, x)))
        .parse()
        .unwrap_or(0);

    Ok((width, height))
}

/// Get the size of the terminal.
#[cfg(target_os = "windows")]
pub fn terminal_size() -> io::Result<(u16, u16)> {
    use kernel32::{GetStdHandle, GetConsoleScreenBufferInfo};
    use winapi::winbase::STD_OUTPUT_HANDLE;
    use winapi::wincon::{PCONSOLE_SCREEN_BUFFER_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT};

    let mut buffer_info: CONSOLE_SCREEN_BUFFER_INFO = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0,
        srWindow: SMALL_RECT { Left: 0, Top: 0, Right: 0, Bottom: 0 },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 }
    };
    unsafe {
        let h_out = GetStdHandle(STD_OUTPUT_HANDLE);
        GetConsoleScreenBufferInfo(h_out, &mut buffer_info as PCONSOLE_SCREEN_BUFFER_INFO);
    }
    let width = buffer_info.srWindow.Right - buffer_info.srWindow.Left + 1;
    let height = buffer_info.srWindow.Bottom - buffer_info.srWindow.Top + 1;

    Ok((width as u16, height as u16))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size() {
        assert!(terminal_size().is_ok());
    }
}
