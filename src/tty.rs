use std::{fs, io};
#[cfg(not(target_os = "windows"))]
use std::os::unix::io::AsRawFd;
#[cfg(target_os = "windows")]
use std::os::windows::io::AsRawHandle;

/// Is this stream an TTY?
#[cfg(not(any(target_os = "redox", target_os = "windows")))]
pub fn is_tty<T: AsRawFd>(stream: &T) -> bool {
    use libc;

    unsafe { libc::isatty(stream.as_raw_fd()) == 1}
}

/// This will panic.
#[cfg(target_os = "redox")]
pub fn is_tty<T: AsRawFd>(_stream: &T) -> bool {
    unimplemented!();
}

/// This will panic.
#[cfg(target_os = "windows")]
pub fn is_tty<T: AsRawHandle>(stream: &T) -> bool {
    use kernel32::GetConsoleMode;
    use winapi::minwindef::{DWORD, LPDWORD};

    let mut out: DWORD = 0;
    // If this function doesn't fail then fd is a TTY
    match unsafe { GetConsoleMode(stream.as_raw_handle(),
                                  &mut out as LPDWORD) } {
        0 => false,
        _ => true,
    }
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
#[cfg(not(any(target_os = "redox", target_os = "windows")))]
pub fn get_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(true).open("/dev/tty")
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
#[cfg(target_os = "redox")]
pub fn get_tty() -> io::Result<fs::File> {
    use std::env;
    let tty = try!(env::var("TTY").map_err(|x| io::Error::new(io::ErrorKind::NotFound, x)));
    fs::OpenOptions::new().read(true).write(true).open(tty)
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
#[cfg(target_os = "windows")]
pub fn get_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(true).open("/dev/tty")
}
