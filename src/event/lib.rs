// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_event"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive, associated_consts)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_fd as fd;
extern crate lrs_io as io;
extern crate lrs_fmt as fmt;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;

use base::prelude::*;
use base::undef::{UndefState};
use cty::{c_int};
use syscall::{close, eventfd2};
use core::{mem};
use fd::{FDContainer};
use flags::{EventfdFlags};
use io::{Read, Write};

mod std { pub use fmt::std::*; pub use cty; }

pub mod flags;

/// An eventfd.
///
/// = See also
///
/// * link:man:eventfd(2)
pub struct Eventfd {
    fd: c_int,
    owned: bool,
}

impl Eventfd {
    /// Creates a new eventfd.
    ///
    /// [argument, flags]
    /// Flags to use when creating the eventfd.
    ///
    /// = See also
    ///
    /// * link:lrs::event::Eventfd::new_init
    /// * link:man:eventfd(2)
    pub fn new(flags: EventfdFlags) -> Result<Eventfd> {
        Eventfd::new_init(0, flags)
    }

    /// Creates a new eventfd with an initial value.
    ///
    /// [argument, val]
    /// The initial value to store in the eventfd.
    ///
    /// [argument, flags]
    /// Flags to use when creating the eventfd.
    ///
    /// = See also
    ///
    /// * link:man:eventfd(2)
    pub fn new_init(val: u32, flags: EventfdFlags) -> Result<Eventfd> {
        let fd = try!(rv!(eventfd2(val, flags.0), -> c_int));
        Ok(Eventfd::from_owned(fd))
    }

    /// Adds a value to the eventfd.
    ///
    /// [argument, val]
    /// The value to add to the eventfd.
    ///
    /// = See also
    ///
    /// * link:man:eventfd(2)
    pub fn add(&self, val: u64) -> Result {
        self.as_fdio().write(val.as_ref()).ignore_ok()
    }

    /// Reads the value of the eventfd.
    ///
    /// = Remarks
    ///
    /// The returned value depends on the flags the eventfd was created with:
    ///
    /// == Default behavior
    ///
    /// By default, the current value of the eventfd will be returned and the value of the
    /// eventfd value will be set to `0`.
    ///
    /// == Semaphore behavior
    ///
    /// If the `EFD_SEMAPHORE` flag was used, `1` is returned and the eventfd value is
    /// reduced by `1`.
    ///
    /// = See also
    ///
    /// * link:man:eventfd(2)
    pub fn get(&self) -> Result<u64> {
        let mut val = 0;
        self.as_fdio().read(val.as_mut());
        Ok(val)
    }
}

unsafe impl UndefState for Eventfd {
    fn num() -> usize { bool::num() }

    unsafe fn set_undef(val: *mut Eventfd, n: usize) {
        bool::set_undef(&mut (*val).owned, n);
    }

    unsafe fn is_undef(val: *const Eventfd, n: usize) -> bool {
        bool::is_undef(&(*val).owned, n)
    }
}

impl Drop for Eventfd {
    fn drop(&mut self) {
        close(self.fd);
    }
}

impl FDContainer for Eventfd {
    fn unwrap(self) -> c_int {
        let fd = self.fd;
        mem::forget(fd);
        fd
    }

    fn is_owned(&self) -> bool {
        self.owned
    }

    fn borrow(&self) -> c_int {
        self.fd
    }

    fn from_owned(fd: c_int) -> Eventfd {
        Eventfd { fd: fd, owned: true }
    }
    fn from_borrowed(fd: c_int) -> Eventfd {
        Eventfd { fd: fd, owned: false }
    }
}
