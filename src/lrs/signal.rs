// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Signal handling.
//!
//! = Remarks
//!
//! Signals can be in one of three states:
//!
//! * Unblocked
//! * Blocked
//! * Ignored
//!
//! Note that `Ignored` is a state that applies to the whole process while `Blocked` only
//! applies to a single thread. If an ignored signal is sent to a thread or process, it is
//! discarded.
//!
//! If a signal is sent to a thread and the thread has the signal blocked, it is queued
//! until the signal unblocks it or retrieves it in some other way. The same principle
//! applies if a signal is sent to a process and all threads have the signal blocked.
//!
//! If a signal is sent to a thread that has the signal unblocked or to a process where at
//! least one thread has the signal unblocked, one of the following actions is taken:
//!
//! * The process is terminated
//! * The process is terminated and a core-dump is generated
//! * The process is stopped
//! * A signal handler is invoked
//!
//! By default, signal handlers are invoked on the stack of the thread that handles the
//! signal. It is possible to use a separate stack which is necessary to handle invalid
//! address access caused by stack overflow.
//!
//! = Threading and forking
//!
//! New threads inherit the set of signals blocked by the creating thread. A forked
//! process inherits all settings from the parent process. Upon execve, all signal
//! handlers are set to the default but the set of blocked and ignored signals is
//! preserved. In most cases, a thread that calls execve should unblock all signals after
//! forking and set the handler to the default.
//!
//! = Real-time and reserved signals
//!
//! Valid signal numbers range from 1 to 64. The signals in the range 32 to 64 are known
//! as real-time signals and have slightly differente semantics from the classic signals
//! which are in the range 1 to 31. Signals 32 to 34 are reserved by the implementation
//! and should not be used by applications.
//!
//! = See also
//!
//! * link:man:signal(7)
//! * link:man:sigaction(2)
//! * link:man:nptl(7)

pub use lrs_signal::{
    Sigset, blocked_signals, block_signal, unblock_signal, block_signals, unblock_signals,
    set_blocked_signals, pending_signals, SigInfo, wait, wait_timeout,
    SigHandler, set_handler, suspend_with, send, send_to_thread,
};
pub use lrs_signal::signals::{Signal};
pub use lrs_signal::sigfd::{Sigfd, SigfdInfo};
pub use lrs_signal::sigfd::flags::{SigfdFlags};

/// Flags for signalfds and signal handlers.
pub mod flags {
    pub use lrs_signal::sigfd::flags::{
        SIGFD_DONT_BLOCK, SIGFD_CLOSE_ON_EXEC, SIGFD_NONE,
    };
    pub use lrs_signal::flags::{
        SA_NONE, SA_NOCLDSTOP, SA_NOCLDWAIT, SA_ALT_STACK, SA_RESTART,
        SA_NODEFER, SA_RESETHAND,
    };
}

/// Known signal constants.
pub mod signals {
    pub use lrs_signal::signals::{
        Hangup, Interrupted, Quit, Illegal, Breakpoint, Abort, Bus, ArithmeticError, Kill,
        User1, User2, InvalidAddress, Pipe, Timer, Termination, Child, Continue, Stop,
        TermStop, TermBackIn, TermBackOut, Urgent, CpuLimit, FileLimit, VirtualAlarm,
        Profiling, Window, Io, Power, Syscall,
    };
}
