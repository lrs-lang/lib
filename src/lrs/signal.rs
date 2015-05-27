// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_signal::{
    Sigset, blocked_signals, block_signal, unblock_signal, block_signals, unblock_signals,
    set_blocked_signals, pending_signals, suspend,
};
pub use lrs_signal::signals::{Signal};

pub mod signals {
    pub use lrs_signal::signals::{
        Hangup, Interrupted, Quit, Illegal, Breakpoint, Abort, Bus, ArithmeticError, Kill,
        User1, User2, InvalidAddress, Pipe, Timer, Termination, Child, Continue, Stop,
        TermStop, TermBackIn, TermBackOut, Urgent, CpuLimit, FileLimit, VirtualAlarm,
        Profiling, Window, Io, Power,
    };
}
