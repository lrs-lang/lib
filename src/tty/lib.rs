// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_tty"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_fd as fd;
extern crate lrs_dev as dev;
extern crate lrs_fmt as fmt;
extern crate lrs_signal as signal;
extern crate lrs_file as file;

#[prelude_import] use base::prelude::*;
use core::{mem};
use cty::{
    c_int, winsize, c_ushort, TCIFLUSH, TCOFLUSH, TCOOFF, TCOON, TCIOFF, TCION,
};
use cty::alias::{ProcessId};
use fd::{FDContainer};
use file::flags::{FileFlags, Mode};
use file::{File};
use dev::{Device, DeviceType};
use syscall::{
    ioctl_tiocgptn, ioctl_tiocsptlck, ioctl_tiocgptlck, ioctl_tiocsig, ioctl_tiocpkt,
    ioctl_tiocgpkt, ioctl_tiocsti, ioctl_tiocgwinsz, ioctl_tiocswinsz, ioctl_tioccons,
    ioctl_tiocexcl, ioctl_tiocgpgrp, ioctl_tiocnxcl, ioctl_tiocgexcl, ioctl_tiocnotty,
    ioctl_tiocsctty, ioctl_tiocspgrp, ioctl_tiocgsid, ioctl_tiocgetd, ioctl_tiocsetd,
    ioctl_tiocvhangup, ioctl_tiocgdev, ioctl_tcflsh, ioctl_tiocoutq, ioctl_fionread,
    ioctl_tcxonc, ioctl_tcgets2,
};
use signal::signals::{Signal};
use disc::{LineDiscipline};
use attr::{TtyAttr};

mod lrs { pub use fmt::lrs::*; pub use cty; }

pub mod disc;
pub mod attr;

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

    pub fn enable_packet_mode(&self) -> Result {
        rv!(ioctl_tiocpkt(self.fd, true))
    }

    pub fn disable_packet_mode(&self) -> Result {
        rv!(ioctl_tiocpkt(self.fd, false))
    }

    pub fn packet_mode(&self) -> Result<bool> {
        let mut mode = false;
        try!(rv!(ioctl_tiocgpkt(self.fd, &mut mode)));
        Ok(mode)
    }

    pub fn push_byte(&self, b: u8) -> Result {
        rv!(ioctl_tiocsti(self.fd, b))
    }

    pub fn window_size(&self) -> Result<(u16, u16)> {
        let mut size: winsize = mem::zeroed();
        try!(rv!(ioctl_tiocgwinsz(self.fd, &mut size)));
        Ok((size.ws_col as u16, size.ws_row as u16))
    }

    pub fn set_window_size(&self, width: u16, height: u16) -> Result {
        let mut size: winsize = mem::zeroed();
        size.ws_col = width as c_ushort;
        size.ws_row = height as c_ushort;
        rv!(ioctl_tiocswinsz(self.fd, &size))
    }

    pub fn redirect_console(&self) -> Result {
        rv!(ioctl_tioccons(self.fd))
    }

    pub fn set_exclusive(&self, exclusive: bool) -> Result {
        if exclusive {
            rv!(ioctl_tiocexcl(self.fd))
        } else {
            rv!(ioctl_tiocnxcl(self.fd))
        }
    }

    pub fn exclusive(&self) -> Result<bool> {
        let mut excl = false;
        try!(rv!(ioctl_tiocgexcl(self.fd, &mut excl)));
        Ok(excl)
    }

    pub fn give_up(&self) -> Result {
        rv!(ioctl_tiocnotty(self.fd))
    }

    pub fn acquire(&self, steal: bool) -> Result {
        rv!(ioctl_tiocsctty(self.fd, steal))
    }

    pub fn foreground_group(&self) -> Result<ProcessId> {
        let mut id = 0;
        try!(rv!(ioctl_tiocgpgrp(self.fd, &mut id)));
        Ok(id)
    }

    pub fn set_foreground_group(&self, group: ProcessId) -> Result {
        rv!(ioctl_tiocspgrp(self.fd, group))
    }

    pub fn session(&self) -> Result<ProcessId> {
        let mut id = 0;
        try!(rv!(ioctl_tiocgsid(self.fd, &mut id)));
        Ok(id)
    }

    pub fn line_discipline(&self) -> Result<LineDiscipline> {
        let mut ld = 0;
        try!(rv!(ioctl_tiocgetd(self.fd, &mut ld)));
        Ok(LineDiscipline(ld))
    }

    pub fn set_line_discipline(&self, disc: LineDiscipline) -> Result {
        rv!(ioctl_tiocsetd(self.fd, disc.0))
    }

    pub fn hang_up(&self) -> Result {
        rv!(ioctl_tiocvhangup(self.fd))
    }

    pub fn device(&self) -> Result<Device> {
        let mut dev = 0;
        try!(rv!(ioctl_tiocgdev(self.fd, &mut dev)));
        Ok(Device::from_id(dev, DeviceType::Character))
    }

    pub fn discard_input(&self) -> Result {
        rv!(ioctl_tcflsh(self.fd, TCIFLUSH))
    }

    pub fn discard_output(&self) -> Result {
        rv!(ioctl_tcflsh(self.fd, TCOFLUSH))
    }

    pub fn pending_output(&self) -> Result<usize> {
        let mut buf = 0;
        try!(rv!(ioctl_tiocoutq(self.fd, &mut buf)));
        Ok(buf)
    }

    pub fn pending_input(&self) -> Result<usize> {
        let mut buf = 0;
        try!(rv!(ioctl_fionread(self.fd, &mut buf)));
        Ok(buf)
    }

    pub fn suspend_output(&self) -> Result {
        rv!(ioctl_tcxonc(self.fd, TCOOFF))
    }

    pub fn start_output(&self) -> Result {
        rv!(ioctl_tcxonc(self.fd, TCOON))
    }

    pub fn suspend_input(&self) -> Result {
        rv!(ioctl_tcxonc(self.fd, TCIOFF))
    }

    pub fn start_input(&self) -> Result {
        rv!(ioctl_tcxonc(self.fd, TCION))
    }

    pub fn attributes(&self) -> Result<TtyAttr> {
        let mut attrs: TtyAttr = mem::zeroed();
        try!(rv!(ioctl_tcgets2(self.fd, &mut attrs.0)));
        Ok(attrs)
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
