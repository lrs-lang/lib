// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {libc};
use base::prelude::*;
use base::{error};
use cty::alias::{ProcessId};
use syscall::{exit_group};

pub fn fork<F>(f: F) -> Result<ProcessId>
    where F: FnOnce()
{
    match unsafe { libc::fork() as ProcessId } {
        -1 => {
            let error = unsafe { *libc::__errno_location() };
            Err(error::Errno(error))
        },
        0 => {
            f();
            exit_group(0);
        },
        n => Ok(n),
    }
}

pub fn fork_continue() -> Result<Option<ProcessId>> {
    match unsafe { libc::fork() as ProcessId } {
        -1 => {
            let error = unsafe { *libc::__errno_location() };
            Err(error::Errno(error))
        },
        0 => Ok(None),
        n => Ok(Some(n)),
    }
}
