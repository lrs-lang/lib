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

// rt_sigaction
// rt_sigqueueinfo
// rt_tgsigqueueinfo

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

pub mod sigfd;
pub mod signals;
pub mod flags;

#[derive(Pod, Eq)]
pub struct Sigset {
    data: sigset_t,
}

impl Sigset {
    pub fn new() -> Sigset {
        mem::zeroed()
    }

    pub fn clear(&mut self) {
        *self = Sigset::new();
    }

    pub fn fill(&mut self) {
        for b in &mut self.data.sig[..] {
            *b = !0;
        }
    }

    pub fn set(&mut self, val: Signal) -> Result {
        let val = val.0.saturating_sub(1) as usize;
        if val >= _NSIG as usize {
            return Err(error::InvalidArgument);
        }
        let disc = 8 * mem::size_of::<SigsetVal>();
        self.data.sig[val / disc] |= 1 << (val % disc);
        Ok(())
    }

    pub fn set_all(&mut self, set: Sigset) {
        for i in 0..self.data.sig.len() {
            self.data.sig[i] |= set.data.sig[i];
        }
    }

    pub fn is_set(&self, val: Signal) -> Result<bool> {
        let val = val.0.saturating_sub(1) as usize;
        if val >= _NSIG as usize {
            return Err(error::InvalidArgument);
        }
        let disc = 8 * mem::size_of::<SigsetVal>();
        Ok(self.data.sig[val / disc] & (1 << (val % disc)) != 0)
    }

    pub fn all_set(&self, set: Sigset) -> bool {
        for i in 0..self.data.sig.len() {
            if self.data.sig[i] & set.data.sig[i] != set.data.sig[i] {
                return false;
            }
        }
        true
    }

    pub fn disjoint(&self, set: Sigset) -> bool {
        for i in 0..self.data.sig.len() {
            if self.data.sig[i] & set.data.sig[i] != 0 {
                return false;
            }
        }
        true
    }

    pub fn unset(&mut self, val: Signal) -> Result {
        let val = val.0.saturating_sub(1) as usize;
        if val >= _NSIG as usize {
            return Err(error::InvalidArgument);
        }
        let disc = 8 * mem::size_of::<SigsetVal>();
        self.data.sig[val / disc] &= !(1 << (val % disc));
        Ok(())
    }

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

pub fn block_signal(sig: Signal) -> Result<Sigset> {
    block_one_common(SIG_BLOCK, sig)
}

pub fn unblock_signal(sig: Signal) -> Result<Sigset> {
    block_one_common(SIG_UNBLOCK, sig)
}

pub fn block_signals(set: Sigset) -> Result<Sigset> {
    block_common(SIG_BLOCK, &set)
}

pub fn unblock_signals(set: Sigset) -> Result<Sigset> {
    block_common(SIG_UNBLOCK, &set)
}

pub fn set_blocked_signals(set: Sigset) -> Result<Sigset> {
    block_common(SIG_SETMASK, &set)
}

pub fn pending_signals() -> Result<Sigset> {
    let mut set: Sigset = mem::zeroed();
    try!(rv!(rt_sigpending(&mut set.data)));
    Ok(set)
}

pub fn suspend(mask: Sigset) {
    rt_sigsuspend(&mask.data);
}

#[derive(Pod)]
pub struct SigInfo {
    data: siginfo_t,
}

impl SigInfo {
    pub fn new() -> SigInfo {
        mem::zeroed()
    }

    pub fn signal(&self) -> Signal {
        Signal(self.data.si_signo() as u8)
    }
}

pub fn wait(set: Sigset) -> Result<SigInfo> {
    let mut info = SigInfo::new();
    try!(rv!(rt_sigtimedwait(&set.data, &mut info.data, None)));
    Ok(info)
}

pub fn wait_timeout(set: Sigset, timeout: Time) -> Result<SigInfo> {
    let mut info = SigInfo::new();
    let timeout = time_to_timespec(timeout);
    try!(rv!(rt_sigtimedwait(&set.data, &mut info.data, Some(&timeout))));
    Ok(info)
}

pub enum SigHandler {
    Func(extern fn(signal: Signal, info: &SigInfo, context: usize)),
    Default,
    Ignore,
}

pub fn set_handler(sig: Signal, mask: Sigset, handler: SigHandler,
                   flags: SigFlags) -> Result {
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

#[link(name = "lrs_asm")]
extern {
    fn lrs_restore();
}
