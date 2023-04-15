use std::io;
use std::ptr;
use windows_sys::Win32::Foundation::{CloseHandle, BOOL, HANDLE, WAIT_FAILED, WAIT_OBJECT_0};
use windows_sys::Win32::System::Console::SetConsoleCtrlHandler;
use windows_sys::Win32::System::Threading::{
    CreateSemaphoreA, ReleaseSemaphore, WaitForSingleObject,
};
use windows_sys::Win32::System::WindowsProgramming::INFINITE;

/// Platform specific error type
pub type Error = io::Error;

/// Platform specific signal type
pub type Signal = u32;

const MAX_SEM_COUNT: i32 = 255;
static mut SEMAPHORE: HANDLE = 0 as HANDLE;
const TRUE: BOOL = 1;
const FALSE: BOOL = 0;

unsafe extern "system" fn os_handler(_: u32) -> BOOL {
    // Assuming this always succeeds. Can't really handle errors in any meaningful way.
    ReleaseSemaphore(SEMAPHORE, 1, ptr::null_mut());
    TRUE
}

/// Register os signal handler.
#[inline]
pub unsafe fn init_os_handler() -> Result<(), Error> {
    SEMAPHORE = CreateSemaphoreA(ptr::null_mut(), 0, MAX_SEM_COUNT, ptr::null());
    if SEMAPHORE == 0 {
        return Err(io::Error::last_os_error());
    }

    if SetConsoleCtrlHandler(Some(os_handler), TRUE) == FALSE {
        let e = io::Error::last_os_error();
        CloseHandle(SEMAPHORE);
        SEMAPHORE = 0 as HANDLE;
        return Err(e);
    }

    Ok(())
}

/// Blocks until a Ctrl-C signal is received.
#[inline]
pub unsafe fn block_ctrl_c() -> Result<(), Error> {
    match WaitForSingleObject(SEMAPHORE, INFINITE) {
        WAIT_OBJECT_0 => Ok(()),
        WAIT_FAILED => Err(io::Error::last_os_error()),
        ret => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("WaitForSingleObject(), unexpected return value \"{:x}\"", ret),
        )),
    }
}
