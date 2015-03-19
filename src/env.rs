use libc::{clearenv};

/// Removes all environment variables.
///
/// ### Return value
///
/// Return true on success, false otherwise.
pub fn clear_vars() -> bool {
    unsafe { clearenv() == 0 }
}
