mod bindings;

use std::{ffi, io, mem, sync::Once};

static INIT: Once = Once::new();

fn init_ctrlc_internal(message: Option<&str>) -> io::Result<()> {
    let mut result = Ok(());
    INIT.call_once(|| unsafe {
        let message = if let Some(message) = message {
            let c_string = ffi::CString::new(message).unwrap();
            let ptr = c_string.as_ptr();
            mem::forget(c_string);
            ptr
        } else {
            std::ptr::null()
        };
        if bindings::init_sigint_handler(message) != 0 {
            result = Err(io::Error::last_os_error());
        }
    });
    result
}

/// Initializes the SIGINT (Ctrl-C) signal handler.
///
/// This function installs a minimal, signal-safe handler for `SIGINT`.
/// Once installed, any incoming Ctrl-C will set an internal flag,
/// which can later be queried via [`is_ctrlc_received()`].
///
/// This function may be called multiple times;
/// the signal handler will only be installed once.
/// Repeated calls are safe and have no additional effect.
///
/// # Note
///
/// Use either this function OR [`init_ctrlc_with_print()`], not both.
///
/// # Errors
///
/// Returns an `Err` if the underlying system call (`sigaction`)
/// fails during handler installation. This typically indicates a
/// low-level OS error or permission issue.
///
/// # Examples
///
/// ```rust,no_run
/// ctrlc_tiny::init_ctrlc()?;
/// while !ctrlc_tiny::is_ctrlc_received() {
///     // Do work here
/// }
/// # Ok::<_, std::io::Error>(())
/// ```
pub fn init_ctrlc() -> io::Result<()> {
    init_ctrlc_internal(None)
}

/// Initializes the SIGINT (Ctrl-C) signal handler with a custom message.
///
/// This function installs a minimal, signal-safe handler for `SIGINT`.
/// Once installed, any incoming Ctrl-C will set an internal flag and
/// print the specified message to stderr.
/// The flag can later be queried via [`is_ctrlc_received()`].
///
/// This function may be called multiple times;
/// the signal handler will only be installed once.
/// Repeated calls are safe and have no additional effect.
///
/// # Note
///
/// Use either this function OR [`init_ctrlc()`], not both.
///
/// # Arguments
///
/// * `message` - The message to print to stderr when Ctrl-C is pressed
///
/// # Errors
///
/// Returns an `Err` if the underlying system call (`sigaction`)
/// fails during handler installation. This typically indicates a
/// low-level OS error or permission issue.
///
/// # Examples
///
/// ```rust,no_run
/// ctrlc_tiny::init_ctrlc_with_print("Ctrl+C pressed\n")?;
/// while !ctrlc_tiny::is_ctrlc_received() {
///     // Do work here
/// }
/// # Ok::<_, std::io::Error>(())
/// ```
pub fn init_ctrlc_with_print(message: &str) -> io::Result<()> {
    init_ctrlc_internal(Some(message))
}

/// Checks whether Ctrl-C (SIGINT) has been received.
///
/// Returns `true` if a `SIGINT` signal (typically from Ctrl-C)
/// has been delivered since [`init_ctrlc()`] was called.
///
/// Once set, the flag remains `true` for the lifetime of the process.
///
/// This function is safe to call from any thread at any time
/// after initialization.
///
/// # Examples
///
/// ```rust,no_run
/// ctrlc_tiny::init_ctrlc_with_print("Ctrl+C pressed\n")?;
/// while !ctrlc_tiny::is_ctrlc_received() {
///     // Do work here
/// }
/// # Ok::<_, std::io::Error>(())
/// ```
pub fn is_ctrlc_received() -> bool {
    unsafe { bindings::get_is_sigint_received() != 0 }
}

/// Resets the internal Ctrl-C received flag to `false`.
///
/// This can be useful if you want to detect multiple Ctrl-C presses
/// independently (e.g. "exit on second Ctrl-C").
///
/// # Safety
///
/// Internally, this clears a `sig_atomic_t` flag that may be concurrently
/// modified by the signal handler. This is safe but may cause a signal
/// received during the reset to be missed.
///
/// # Examples
///
/// ```rust,no_run
/// ctrlc_tiny::init_ctrlc_with_print("Ctrl+C pressed\n")?;
/// let mut count = 0;
/// loop {
///     if ctrlc_tiny::is_ctrlc_received() {
///         ctrlc_tiny::reset_ctrlc_received();
///         count += 1;
///         if count == 2 {
///             break;
///         }
///     }
/// }
/// # Ok::<_, std::io::Error>(())
/// ```
pub fn reset_ctrlc_received() {
    unsafe {
        bindings::reset_is_sigint_received();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_ctrlc_should_succeed_and_be_idempotent() {
        assert!(init_ctrlc().is_ok());
        assert!(init_ctrlc().is_ok());
    }

    #[test]
    fn init_ctrlc_with_print_should_succeed() {
        assert!(init_ctrlc_with_print("Test message\n").is_ok());
    }

    #[test]
    fn is_ctrlc_received_initially_false() {
        assert!(!is_ctrlc_received());
        reset_ctrlc_received();
        assert!(!is_ctrlc_received());
    }
}
