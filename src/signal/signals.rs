// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals, non_camel_case_types)]

use base::prelude::*;
use cty::{self};
use fmt::{Debug, Write};

/// A signal.
///
/// [field, 1]
/// The signal number.
#[repr(C)]
#[derive(Pod, Eq)]
pub struct Signal(pub u8);

macro_rules! create {
    ($($name:ident = $val:ident, $str:expr,)*) => {
        $(#[doc = $str] pub const $name: Signal = Signal(cty::$val as u8);)*

        impl Debug for Signal {
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
    Hangup          = SIGHUP,    "Hangup of the controlling terminal",
    Interrupted     = SIGINT,    "Interrupt from keyboard",
    Quit            = SIGQUIT,   "Quit from keyboard",
    Illegal         = SIGILL,    "Illegal instruction",
    Breakpoint      = SIGTRAP,   "Breakpoint trap",
    Abort           = SIGABRT,   "Abort",
    Bus             = SIGBUS,    "Bus error",
    ArithmeticError = SIGFPE,    "Arithmetic error",
    Kill            = SIGKILL,   "Kill",
    User1           = SIGUSR1,   "User defined signal 1",
    User2           = SIGUSR2,   "User defined signal 2",
    InvalidAddress  = SIGSEGV,   "Invalid memory access",
    Pipe            = SIGPIPE,   "Write to pipe with no reader",
    Timer           = SIGALRM,   "Timer signal",
    Termination     = SIGTERM,   "Termination",
    Child           = SIGCHLD,   "Child stopped or terminated",
    Continue        = SIGCONT,   "Continue if stopped",
    Stop            = SIGSTOP,   "Stop process",
    TermStop        = SIGTSTP,   "Stop from terminal",
    TermBackIn      = SIGTTIN,   "Input for terminal background process",
    TermBackOut     = SIGTTOU,   "Output for terminal background process",
    Urgent          = SIGURG,    "Urgent socket data",
    CpuLimit        = SIGXCPU,   "CPU limit exceeded",
    FileLimit       = SIGXFSZ,   "File size limit exceeded",
    VirtualAlarm    = SIGVTALRM, "Virtual alarm clock",
    Profiling       = SIGPROF,   "Profiling timer expired",
    Window          = SIGWINCH,  "Window size changed",
    Io              = SIGIO,     "I/O now possible",
    Power           = SIGPWR,    "Power failure",
    Syscall         = SIGSYS,    "Bad system call",
}
