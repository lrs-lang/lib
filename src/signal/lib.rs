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

pub mod lrs { pub use fmt::lrs::*; pub use cty; }

// rt_sigaction
// rt_sigtimedwait
// rt_sigqueueinfo
// rt_tgsigqueueinfo

#[prelude_import] use base::prelude::*;
use core::{mem};
use base::{error};
use cty::{sigset_t, SigsetVal, _NSIG, SIG_BLOCK, SIG_UNBLOCK, SIG_SETMASK, c_int};
use signals::{Signal};
use fmt::{Debug, Write};
use syscall::{rt_sigprocmask, rt_sigpending, rt_sigsuspend};

pub mod sigfd;
pub mod signals;

#[derive(Pod, Eq)]
pub struct Sigset {
    data: sigset_t,
}

impl Sigset {
    pub fn new() -> Sigset {
        mem::zeroed()
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

// pub fn wait(set: Sigset) -> SigInfo {
// }
