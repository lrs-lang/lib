use libc::{pid_t, getpid, getppid};

pub mod ids;

/// Returns the process id of this process.
pub fn this_process_id() -> pid_t {
    unsafe { getpid() }
}

/// Returns the process id of the process that created this process.
pub fn parent_process_id() -> pid_t {
    unsafe { getppid() }
}


