// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_tty"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_fd as fd;
extern crate lrs_signal as signal;
extern crate lrs_file as file;

#[prelude_import] use base::prelude::*;
use core::{mem};
use cty::{c_int};
use fd::{FDContainer};
use file::flags::{FileFlags, Mode};
use file::{File};
use syscall::{
    ioctl_tiocgptn, ioctl_tiocsptlck, ioctl_tiocgptlck, ioctl_tiocsig,
};
use signal::signals::{Signal};

mod lrs { pub use base::lrs::*; pub use cty; }

pub struct Tty {
    fd: c_int,
    owned: bool,
}

impl Tty {
    pub fn new_pty(flags: FileFlags) -> Result<Tty> {
        Ok(Tty::from_owned(try!(File::open("/dev/ptmx", flags, Mode(0))).unwrap()))
    }

    pub fn enable_slave(&self) -> Result {
        rv!(ioctl_tiocsptlck(self.fd, false))
    }

    pub fn disable_slave(&self) -> Result {
        rv!(ioctl_tiocsptlck(self.fd, true))
    }

    pub fn slave_enabled(&self) -> Result<bool> {
        let mut res = false;
        try!(rv!(ioctl_tiocgptlck(self.fd, &mut res)));
        Ok(res)
    }

    pub fn pty_id(&self) -> Result<u32> {
        let mut id = 0;
        try!(rv!(ioctl_tiocgptn(self.fd, &mut id)));
        Ok(id)
    }

    /// Sends a signal to the process group at the other end of a pseudo terminal.
    ///
    /// [argument, sig]
    /// The signal to send.
    ///
    /// = Remarks
    ///
    /// Only `Interrupted`, `Quit`, and `TermStop` can be sent.
    pub fn signal(&self, sig: Signal) -> Result {
        rv!(ioctl_tiocsig(self.fd, sig.0 as c_int))
    }

    pub fn set_packet_mode(&self, enabled: bool) -> Result {
    }
}

impl FDContainer for Tty {
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

    fn from_owned(fd: c_int) -> Tty {
        Tty { fd: fd, owned: true }
    }

    fn from_borrowed(fd: c_int) -> Tty {
        Tty { fd: fd, owned: false }
    }
}
