#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod use self::unix::*;

#[cfg(windows)]
mod use self::windows::*;
