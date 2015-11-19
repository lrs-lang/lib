// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use base::{error};
use {r_syscall, cty};
use cty::alias::{ProcessId};
use syscall::{exit_group};

pub fn fork<F>(f: F) -> Result<ProcessId>
    where F: FnOnce()
{
    let rv = unsafe {
        r_syscall::clone(cty::SIGCHLD as cty::k_ulong, 0 as *mut _, 0 as *mut _,
                         0 as *mut _, 0 as *mut _)
    };
    match rv {
        e if e < 0 => Err(error::Errno(-e as cty::c_int)),
        0 => {
            f();
            exit_group(0);
        },
        n => Ok(n as ProcessId),
    }
}

pub fn fork_continue() -> Result<Option<ProcessId>> {
    let rv = unsafe {
        r_syscall::clone(cty::SIGCHLD as cty::k_ulong, 0 as *mut _, 0 as *mut _,
                         0 as *mut _, 0 as *mut _)
    };
    match rv {
        e if e < 0 => Err(error::Errno(-e as cty::c_int)),
        0 => Ok(None),
        n => Ok(Some(n as ProcessId)),
    }
}
