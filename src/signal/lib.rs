// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_signal"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_fmt as fmt;
extern crate lrs_fd as fd;
extern crate lrs_rv as rv;
extern crate lrs_time_base as time_base;

pub mod lrs { pub use fmt::lrs::*; pub use cty; }

#[prelude_import] use base::prelude::*;
use core::{mem};
use base::{error};
use cty::{
    sigset_t, SigsetVal, _NSIG, SIG_BLOCK, SIG_UNBLOCK, SIG_SETMASK, c_int, siginfo_t,
    sigaction, SIG_DFL, SIG_IGN, c_ulong, SA_SIGINFO, SA_RESTORER,
};
use signals::{Signal};
use fmt::{Debug, Write};
use time_base::{Time, time_to_timespec};
use syscall::{
    rt_sigprocmask, rt_sigpending, rt_sigsuspend, rt_sigtimedwait, rt_sigaction,
};
use flags::{SigFlags};
use rv::{retry};

pub mod sigfd;
pub mod signals;
pub mod flags;

/// A set of signals.
#[derive(Pod, Eq)]
pub struct Sigset {
    data: sigset_t,
}

impl Sigset {
    /// Creates a new empty set.
    pub fn new() -> Sigset {
        mem::zeroed()
    }

    /// Removes all signals from the set.
    pub fn clear(&mut self) {
        *self = Sigset::new();
    }

    /// Adds all signals to the set.
    ///
    /// = Remarks
    ///
    /// This includes the reserved signals 32 to 34.
    pub fn fill(&mut self) {
        for b in &mut self.data.sig[..] {
            *b = !0;
        }
    }

    /// Adds a signal to the set.
    ///
    /// [argument, val]
    /// The signal to add.
    pub fn set(&mut self, val: Signal) -> Result {
        let val = val.0.saturating_sub(1) as usize;
        if val >= _NSIG as usize {
            return Err(error::InvalidArgument);
        }
        let disc = 8 * mem::size_of::<SigsetVal>();
        self.data.sig[val / disc] |= 1 << (val % disc);
        Ok(())
    }

    /// Adds all signals from another set to this set.
    ///
    /// [argument, set]
    /// The set to add.
    pub fn set_all(&mut self, set: Sigset) {
        for i in 0..self.data.sig.len() {
            self.data.sig[i] |= set.data.sig[i];
        }
    }

    /// Returns whether a signal is in the set.
    ///
    /// [argument, val]
    /// The signal to check.
    pub fn is_set(&self, val: Signal) -> Result<bool> {
        let val = val.0.saturating_sub(1) as usize;
        if val >= _NSIG as usize {
            return Err(error::InvalidArgument);
        }
        let disc = 8 * mem::size_of::<SigsetVal>();
        Ok(self.data.sig[val / disc] & (1 << (val % disc)) != 0)
    }

    /// Returns whether this set is a superset of another set.
    ///
    /// [argument, set]
    /// The subset.
    pub fn all_set(&self, set: Sigset) -> bool {
        for i in 0..self.data.sig.len() {
            if self.data.sig[i] & set.data.sig[i] != set.data.sig[i] {
                return false;
            }
        }
        true
    }

    /// Returns whether this set and another set are disjoint.
    ///
    /// [argument, set]
    /// The other set.
    pub fn disjoint(&self, set: Sigset) -> bool {
        for i in 0..self.data.sig.len() {
            if self.data.sig[i] & set.data.sig[i] != 0 {
                return false;
            }
        }
        true
    }

    /// Removes a signal from the set.
    ///
    /// [argument, val]
    /// The signal to remove.
    pub fn unset(&mut self, val: Signal) -> Result {
        let val = val.0.saturating_sub(1) as usize;
        if val >= _NSIG as usize {
            return Err(error::InvalidArgument);
        }
        let disc = 8 * mem::size_of::<SigsetVal>();
        self.data.sig[val / disc] &= !(1 << (val % disc));
        Ok(())
    }

    /// Removes all signals in another set from this set.
    ///
    /// [argument, set]
    /// The signals to remove.
    pub fn unset_all(&mut self, set: Sigset) {
        for i in 0..self.data.sig.len() {
            self.data.sig[i] &= !set.data.sig[i];
        }
    }
}

impl Debug for Sigset {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let mut first = true;
        for i in 0.._NSIG as u8 {
            if self.is_set(Signal(i + 1)) == Ok(true) {
                if !first {
                    try!(w.write_all(b","));
                }
                first = false;
                try!(Signal(i + 1).fmt(w));
            }
        }
        Ok(())
    }
}

/// Returns the set of signals that are blocked by this thread.
///
/// = See also
///
/// * link:man:sigprocmask(2)
pub fn blocked_signals() -> Result<Sigset> {
    let mut old = mem::zeroed();
    try!(rv!(rt_sigprocmask(0, None, Some(&mut old))));
    Ok(Sigset { data: old })
}

fn block_common(how: c_int, set: &Sigset) -> Result<Sigset> {
    let mut old = mem::zeroed();
    try!(rv!(rt_sigprocmask(how, Some(&set.data), Some(&mut old))));
    Ok(Sigset { data: old })
}

fn block_one_common(how: c_int, sig: Signal) -> Result<Sigset> {
    let mut set: Sigset = mem::zeroed();
    set.set(sig);
    block_common(how, &set)
}

/// Blocks a signal from this thread.
///
/// [argument, sig]
/// The signal to block.
///
/// [return_value]
/// Returns of signals that were blocked before the call.
///
/// = See also
///
/// * link:man:sigprocmask(2)
pub fn block_signal(sig: Signal) -> Result<Sigset> {
    block_one_common(SIG_BLOCK, sig)
}

/// Unblocks a signal from this thread.
///
/// [argument, sig]
/// The signal to unblock.
///
/// [return_value]
/// Returns of signals that were blocked before the call.
///
/// = See also
///
/// * link:man:sigprocmask(2)
pub fn unblock_signal(sig: Signal) -> Result<Sigset> {
    block_one_common(SIG_UNBLOCK, sig)
}

/// Blocks a set of signals from this thread.
///
/// [argument, set]
/// The signals to block.
///
/// [return_value]
/// Returns of signals that were blocked before the call.
///
/// = See also
///
/// * link:man:sigprocmask(2)
pub fn block_signals(set: Sigset) -> Result<Sigset> {
    block_common(SIG_BLOCK, &set)
}

/// Unblocks a set of signals from this thread.
///
/// [argument, set]
/// The signals to unblock.
///
/// [return_value]
/// Returns of signals that were blocked before the call.
///
/// = See also
///
/// * link:man:sigprocmask(2)
pub fn unblock_signals(set: Sigset) -> Result<Sigset> {
    block_common(SIG_UNBLOCK, &set)
}

/// Sets the set of signals that are blocked from this thread.
///
/// [argument, set]
/// The new set of blocked signals.
///
/// [return_value]
/// Returns of signals that were blocked before the call.
///
/// = See also
///
/// * link:man:sigprocmask(2)
pub fn set_blocked_signals(set: Sigset) -> Result<Sigset> {
    block_common(SIG_SETMASK, &set)
}

/// Returns the set of blocked signals that are currently blocked from this thread.
///
/// = See also
///
/// * link:man:sigpending(2)
pub fn pending_signals() -> Result<Sigset> {
    let mut set: Sigset = mem::zeroed();
    try!(rv!(rt_sigpending(&mut set.data)));
    Ok(set)
}

/// Suspends the calling thread until a signal handler is invoked.
///
/// [argument, mask]
/// The signal mask that is temporarily set during this call.
///
/// = See also
///
/// * link:man:sigsuspend(2)
pub fn suspend(mask: Sigset) {
    rt_sigsuspend(&mask.data);
}

/// Information about a received signal.
#[derive(Pod)]
pub struct SigInfo {
    data: siginfo_t,
}

impl SigInfo {
    /// Creates a new object.
    pub fn new() -> SigInfo {
        mem::zeroed()
    }

    /// Returns the signal.
    pub fn signal(&self) -> Signal {
        Signal(self.data.si_signo() as u8)
    }
}

/// Suspends the calling thread until a certain signal arrives.
///
/// [argument, set]
/// The set of signals to wait for.
///
/// [return_value]
/// Returns information about the received signal.
///
/// = Remarks
///
/// The signals in the set must first be blocked. This function can return prematurely if
/// a signal handler was invoked. If lrs was compiled with the `retry` option, the
/// operation will automatically be restarted in this case.
///
/// = See also
///
/// * link:man:sigtimedwait(2)
pub fn wait(set: Sigset) -> Result<SigInfo> {
    let mut info = SigInfo::new();
    try!(retry(|| rt_sigtimedwait(&set.data, &mut info.data, None)));
    Ok(info)
}

/// Suspends the calling thread until a certain signal arrives or a timeout expires.
///
/// [argument, set]
/// The set of signals to wait for.
///
/// [argument, timeout]
/// The timeout.
///
/// [return_value]
/// Returns information about the received signal.
///
/// = Remarks
///
/// The signals in the set must first be blocked. This function can return prematurely if
/// a signal handler was invoked.
///
/// = See also
///
/// * link:man:sigtimedwait(2)
pub fn wait_timeout(set: Sigset, timeout: Time) -> Result<SigInfo> {
    let mut info = SigInfo::new();
    let timeout = time_to_timespec(timeout);
    try!(retry(|| rt_sigtimedwait(&set.data, &mut info.data, Some(&timeout))));
    Ok(info)
}

/// A signal handler.
pub enum SigHandler {
    /// A function that is invoked when a signal arrives.
    ///
    /// [field, 1]
    /// The function. The first argument is the signal that was invoked. The second
    /// argument contains information about the signal.
    Func(extern fn(signal: Signal, info: &SigInfo, context: usize)),

    /// The default handler.
    Default,

    /// The signal will be ignored.
    Ignore,
}

/// Sets the handler of a signal.
///
/// [argument, sig]
/// The signal whose handler will be set.
///
/// [argument, mask]
/// A set of signals that will be blocked while the signal handler is being invoked.
///
/// [argument, handler]
/// The handler of the signal.
///
/// [argument, flags]
/// Additional flags.
pub fn set_handler(sig: Signal, mask: Sigset, handler: SigHandler,
                   flags: SigFlags) -> Result {
    #[link(name = "lrs_asm")]
    extern { fn lrs_restore(); }

    let action = sigaction {
        sa_handler: match handler {
            SigHandler::Func(f) => unsafe { mem::cast(f) },
            SigHandler::Default => SIG_DFL,
            SigHandler::Ignore => SIG_IGN,
        },
        sa_flags: (flags.0 | SA_SIGINFO | SA_RESTORER) as c_ulong,
        sa_restorer: unsafe { mem::cast(lrs_restore) },
        sa_mask: mask.data,
    };
    rv!(rt_sigaction(sig.0 as c_int, Some(&action), None))
}
