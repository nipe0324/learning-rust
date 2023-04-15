#[macro_use]

mod error;
mod platform;
pub use platform::Signal;
mod signal;
pub use signal::*;

pub use error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static INIT: AtomicBool = AtomicBool::new(false);

/// Register signal handler for Ctrl-C.
///
/// Starts a new dedicated signal handler thread. Should only be called once,
/// typically at the start of your program.
pub fn set_handler<F>(mut user_handler: F) -> Result<(), Error>
where
    F: FnMut() -> () + 'static + Send,
{
    if INIT
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .map_or_else(|e| e, |a| a)
    {
        return Err(Error::MultipleHandlers);
    }

    unsafe {
        match platform::init_os_handler() {
            Ok(_) => {}
            Err(e) => {
                INIT.store(false, Ordering::SeqCst);
                return Err(e.info());
            }
        }
    }

    thread::Builder
        .new()
        .name("ctrl-c".into())
        .spawn(move || loop {
            unsafe {
                platform::block_ctrl_c().expect("Critical system error while waiting for Ctrl-C");
            }
            user_handler();
        })
        .expect("failed to spawn thread");

    Ok(())
}
