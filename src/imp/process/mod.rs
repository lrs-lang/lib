use imp::cty::{pid_t};
use imp::syscall::{getpid, getppid};

pub mod ids;

/// Returns the process id of this process.
pub fn this_process_id() -> pid_t {
    getpid()
}

/// Returns the process id of the process that created this process.
pub fn parent_process_id() -> pid_t {
    getppid()
}


