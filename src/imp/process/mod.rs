// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use imp::cty::{pid_t};
use imp::syscall::{getpid, getppid};

pub mod ids;

pub type ProcessId = pid_t;

/// Returns the process id of this process.
pub fn this_process_id() -> ProcessId {
    getpid()
}

/// Returns the process id of the process that created this process.
pub fn parent_process_id() -> ProcessId {
    getppid()
}
