// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_signal::{
    Sigset, blocked_signals, block_signal, unblock_signal, block_signals, unblock_signals,
    set_blocked_signals, pending_signals, suspend, SigInfo, wait, wait_timeout,
    SigHandler, set_handler,
};
pub use lrs_signal::signals::{Signal};
pub use lrs_signal::sigfd::{Sigfd, SigfdInfo};
pub use lrs_signal::sigfd::flags::{SigfdFlags};

pub mod flags {
    pub use lrs_signal::sigfd::flags::{
        SIGFD_DONT_BLOCK, SIGFD_CLOSE_ON_EXEC, SIGFD_NONE,
    };
    pub use lrs_signal::flags::{
        SA_NONE, SA_NOCLDSTOP, SA_NOCLDWAIT, SA_SIGINFO, SA_ONSTACK, SA_RESTART,
        SA_NODEFER, SA_RESETHAND,
    };
}

pub mod signals {
    pub use lrs_signal::signals::{
        Hangup, Interrupted, Quit, Illegal, Breakpoint, Abort, Bus, ArithmeticError, Kill,
        User1, User2, InvalidAddress, Pipe, Timer, Termination, Child, Continue, Stop,
        TermStop, TermBackIn, TermBackOut, Urgent, CpuLimit, FileLimit, VirtualAlarm,
        Profiling, Window, Io, Power,
    };
}
