// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[allow(unused_imports)] #[prelude_import] use base::prelude::*;
use core::{mem};
use core::ops::{BitAnd, BitOr, Not};
use cty::{
    c_int, WEXITED, WSTOPPED, WCONTINUED, WNOHANG, WNOWAIT, P_ALL, CLD_EXITED,
    CLD_KILLED, CLD_DUMPED, CLD_STOPPED, CLD_TRAPPED, CLD_CONTINUED, P_PID,
};
use cty::alias::{ProcessId};
use syscall::{waitid};
use rv::{retry};
use fmt::{Debug, Write};

#[derive(Copy, Eq)]
pub enum ChildStatus {
    None,
    Exited(c_int),
    Killed(c_int),
    Dumped(c_int),
    Stopped(c_int),
    Trapped(c_int),
    Continued(c_int),
}

impl Debug for ChildStatus {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let id = match *self {
            ChildStatus::None => return w.write(b"None").ignore_ok(),
            ChildStatus::Exited(i)    => { try!(w.write(b"Exited"));    i },
            ChildStatus::Killed(i)    => { try!(w.write(b"Killed"));    i },
            ChildStatus::Dumped(i)    => { try!(w.write(b"Dumped"));    i },
            ChildStatus::Stopped(i)   => { try!(w.write(b"Stopped"));   i },
            ChildStatus::Trapped(i)   => { try!(w.write(b"Trapped"));   i },
            ChildStatus::Continued(i) => { try!(w.write(b"Continued")); i },
        };
        write!(w, "({})", id)
    }
}

#[derive(Pod, Eq)]
pub struct WaitFlags(c_int);

impl BitAnd for WaitFlags {
    type Output = WaitFlags;
    fn bitand(self, other: WaitFlags) -> WaitFlags { WaitFlags(self.0 & other.0) }
}

impl BitOr for WaitFlags {
    type Output = WaitFlags;
    fn bitor(self, other: WaitFlags) -> WaitFlags { WaitFlags(self.0 | other.0) }
}

impl Not for WaitFlags {
    type Output = WaitFlags;
    fn not(self) -> WaitFlags { WaitFlags(!self.0) }
}

pub const WAIT_EXITED:       WaitFlags = WaitFlags( WEXITED    );
pub const WAIT_STOPPED:      WaitFlags = WaitFlags( WSTOPPED   );
pub const WAIT_CONTINUED:    WaitFlags = WaitFlags( WCONTINUED );
pub const WAIT_NON_BLOCKING: WaitFlags = WaitFlags( WNOHANG    );
pub const WAIT_DONT_REAP:    WaitFlags = WaitFlags( WNOWAIT    );

pub fn wait_all(flags: WaitFlags) -> Result<(ProcessId, ChildStatus)> {
    wait_inner(P_ALL, 0, flags)
}

pub fn wait_id(id: ProcessId, flags: WaitFlags) -> Result<ChildStatus> {
    wait_inner(P_PID, id, flags).map(|o| o.1)
}

fn wait_inner(kind: c_int, id: ProcessId,
              flags: WaitFlags) -> Result<(ProcessId, ChildStatus)> {
    let mut info = mem::zeroed();
    try!(retry(|| waitid(kind, id, &mut info, flags.0, None)));
    let sigchld = info._sigchld();
    // The kernel zeros everything if we use WAIT_NON_BLOCKING and there is nothing
    // to wait for. This is a non-standard linux extension.
    let status = match code_to_status(info.si_code()) {
        Some(f) => f(sigchld._status),
        _ => ChildStatus::None,
    };
    Ok((sigchld._pid, status))
}

fn code_to_status(code: c_int) -> Option<fn(c_int) -> ChildStatus> {
    match code {
        CLD_EXITED    => Some(ChildStatus::Exited),
        CLD_KILLED    => Some(ChildStatus::Killed),
        CLD_DUMPED    => Some(ChildStatus::Dumped),
        CLD_STOPPED   => Some(ChildStatus::Stopped),
        CLD_TRAPPED   => Some(ChildStatus::Trapped),
        CLD_CONTINUED => Some(ChildStatus::Continued),
        _ => None,
    }
}
