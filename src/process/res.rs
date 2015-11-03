// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals, non_camel_case_types)]

use base::prelude::*;
use fmt::{Debug, Write};
use cty::{
    self, c_int,
};

#[derive(Pod, Eq)]
pub struct Resource(pub c_int);

macro_rules! create {
    ($($(#[$meta:meta])* res $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: Resource = Resource(cty::$val as c_int);)*

        impl Debug for Resource {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let s = match *self {
                    $($name => stringify!($name),)*
                    _ => return write!(w, "Unknown({})", self.0),
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    #[doc = "The maximum size of the process's virtual memory.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_AS therein"]
    res VirtualMemory = RLIMIT_AS;

    #[doc = "The maximum size of a core dump.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_CORE therein"]
    res CoreDumpSize = RLIMIT_CORE;

    #[doc = "The maximum amount of CPU time a process can use.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_CPU therein"]
    res CpuTime = RLIMIT_CPU;

    #[doc = "The maximum amount of CPU time a process can use before making a blocking
             system call.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_RTTIME therein"]
    res ContiguousCpuTime = RLIMIT_RTTIME;

    #[doc = "The maximum size of the process's data segment.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_DATA therein"]
    res DataSegment = RLIMIT_DATA;

    #[doc = "The maximum size of files this process can create.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_FSIZE therein"]
    res FileSize = RLIMIT_FSIZE;

    #[doc = "The maximum number of bytes a process can lock into memory.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_MEMLOCK therein"]
    res LockedMemory = RLIMIT_MEMLOCK;

    #[doc = "The maximum number of bytes the user can store in message queues.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_MSGQUEUE therein"]
    res MsgQueue = RLIMIT_MSGQUEUE;

    #[doc = "The maximum value to which the niceness of the process can be raised.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_NICE therein"]
    res Niceness = RLIMIT_NICE;

    #[doc = "The maximum number of file descriptors the process can open.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_NOFILE therein"]
    res FileDescriptors = RLIMIT_NOFILE;

    #[doc = "The maximum number of processes the user can create.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_NPROC therein"]
    res Processes = RLIMIT_NPROC;

    #[doc = "The maximum real time priority of this process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_RTPRIO therein"]
    res Priority = RLIMIT_RTPRIO;

    #[doc = "The maximum number of pending signals for the process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_SIGPENDING therein"]
    res PendingSignals = RLIMIT_SIGPENDING;

    #[doc = "The maximum size of the process stack.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:prlimit(2) and RLIMIT_STACK therein"]
    res Stack = RLIMIT_STACK;
}
