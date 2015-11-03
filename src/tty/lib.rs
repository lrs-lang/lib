// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_tty"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_fd as fd;
extern crate lrs_dev as dev;
extern crate lrs_fmt as fmt;
extern crate lrs_signal as signal;
extern crate lrs_file as file;

use base::prelude::*;
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
    ioctl_tcxonc, ioctl_tcgets2, ioctl_tcsets2, vhangup,
};
use signal::signals::{Signal};
use disc::{LineDiscipline};
use attr::{TtyAttr};

mod std { pub use fmt::std::*; pub use cty; }

pub mod disc;
pub mod attr;
pub mod key;

pub struct Tty {
    fd: c_int,
    owned: bool,
}

impl Tty {
    /// Creates a new pseudo terminal.
    ///
    /// [argument, flags]
    /// Flags used for opening the master.
    ///
    /// [return_value]
    /// Returns the master.
    ///
    /// = See also
    ///
    /// * link:man:pty(7)
    pub fn new_pty(flags: FileFlags) -> Result<Tty> {
        Ok(Tty::from_owned(try!(File::open("/dev/ptmx", flags, Mode(0))).unwrap()))
    }

    /// Allows opening the slave.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCSPTLCK therein
    pub fn enable_slave(&self) -> Result {
        rv!(ioctl_tiocsptlck(self.fd, false))
    }

    /// Disallows opening the slave.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCSPTLCK therein
    pub fn disable_slave(&self) -> Result {
        rv!(ioctl_tiocsptlck(self.fd, true))
    }

    /// Returns whether the slave can be opened.
    ///
    /// = Remarks
    ///
    /// == Kernel versions
    ///
    /// The required kernel version is 3.8.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCGPTLCK therein
    pub fn slave_enabled(&self) -> Result<bool> {
        let mut res = false;
        try!(rv!(ioctl_tiocgptlck(self.fd, &mut res)));
        Ok(res)
    }

    /// Returns the id of this pseudo terminal.
    ///
    /// = Remarks
    ///
    /// The slave can be found under `/dev/pts/{id}`.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCGPTN therein
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
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOSIG therein
    pub fn signal(&self, sig: Signal) -> Result {
        rv!(ioctl_tiocsig(self.fd, sig.0 as c_int))
    }

    /// Enabled packet mode for this terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCPKT therein
    pub fn enable_packet_mode(&self) -> Result {
        rv!(ioctl_tiocpkt(self.fd, true))
    }

    /// Disables packet mode for this terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCPKT therein
    pub fn disable_packet_mode(&self) -> Result {
        rv!(ioctl_tiocpkt(self.fd, false))
    }

    /// Returns whether packet mode is enabled.
    ///
    /// = Remarks
    ///
    /// == Kernel versions
    ///
    /// The required kernel version is 3.8.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCGPKT therein
    pub fn packet_mode(&self) -> Result<bool> {
        let mut mode = false;
        try!(rv!(ioctl_tiocgpkt(self.fd, &mut mode)));
        Ok(mode)
    }

    /// Adds a single byte to the input queue of the terminal.
    ///
    /// [argument, b]
    /// The byte to add.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCSTI therein
    pub fn push_byte(&self, b: u8) -> Result {
        rv!(ioctl_tiocsti(self.fd, b))
    }

    /// Returns the size of the terminal window.
    ///
    /// [return_value]
    /// Returns (width, height).
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCGWINSZ therein
    pub fn window_size(&self) -> Result<(u16, u16)> {
        let mut size: winsize = mem::zeroed();
        try!(rv!(ioctl_tiocgwinsz(self.fd, &mut size)));
        Ok((size.ws_col as u16, size.ws_row as u16))
    }

    /// Sets the size of the terminal window.
    ///
    /// [argument, width]
    /// The width of the window.
    ///
    /// [argument, height]
    /// The height of the window.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCSWINSZ therein
    pub fn set_window_size(&self, width: u16, height: u16) -> Result {
        let mut size: winsize = mem::zeroed();
        size.ws_col = width as c_ushort;
        size.ws_row = height as c_ushort;
        rv!(ioctl_tiocswinsz(self.fd, &size))
    }

    /// Redirects `/dev/console` output to this terminal.
    ///
    /// = Remarks
    ///
    /// This requires `CAP_SYS_ADMIN` privileges.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCCONS therein
    pub fn redirect_console(&self) -> Result {
        rv!(ioctl_tioccons(self.fd))
    }

    /// Enables or disables exclusive mode for this terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCEXCL and TIOCNXCL therein
    pub fn set_exclusive(&self, exclusive: bool) -> Result {
        if exclusive {
            rv!(ioctl_tiocexcl(self.fd))
        } else {
            rv!(ioctl_tiocnxcl(self.fd))
        }
    }

    /// Returns whether exclusive mode is enabled.
    ///
    /// = Remarks
    ///
    /// == Kernel versions
    ///
    /// The required kernel version is 3.8.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCGEXCL therein
    pub fn exclusive(&self) -> Result<bool> {
        let mut excl = false;
        try!(rv!(ioctl_tiocgexcl(self.fd, &mut excl)));
        Ok(excl)
    }

    /// Gives up this controlling terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCNOTTY therein
    pub fn give_up(&self) -> Result {
        rv!(ioctl_tiocnotty(self.fd))
    }

    /// Acquire this terminal as the controlling terminal.
    ///
    /// [argument, steal]
    /// Whether to steal this terminal if it's already a controlling terminal of another
    /// session.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCSCTTY therein
    pub fn acquire(&self, steal: bool) -> Result {
        rv!(ioctl_tiocsctty(self.fd, steal))
    }

    /// Returns the foreground process group of this terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCGPGRP therein
    pub fn foreground_group(&self) -> Result<ProcessId> {
        let mut id = 0;
        try!(rv!(ioctl_tiocgpgrp(self.fd, &mut id)));
        Ok(id)
    }

    /// Sets the foreground process group of this terminal.
    ///
    /// [argument, group]
    /// The new foreground process group.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCSPGRP therein
    pub fn set_foreground_group(&self, group: ProcessId) -> Result {
        rv!(ioctl_tiocspgrp(self.fd, group))
    }

    /// Returns the session of this terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCGSID therein
    pub fn session(&self) -> Result<ProcessId> {
        let mut id = 0;
        try!(rv!(ioctl_tiocgsid(self.fd, &mut id)));
        Ok(id)
    }

    /// Returns the line discipline of this terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCGETD therein
    pub fn line_discipline(&self) -> Result<LineDiscipline> {
        let mut ld = 0;
        try!(rv!(ioctl_tiocgetd(self.fd, &mut ld)));
        Ok(LineDiscipline(ld))
    }

    /// Sets the line discipline of this terminal.
    ///
    /// [argument, disc]
    /// The new line discipline.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCSETD therein
    pub fn set_line_discipline(&self, disc: LineDiscipline) -> Result {
        rv!(ioctl_tiocsetd(self.fd, disc.0))
    }

    /// Simulates a hang-up of this terminal.
    ///
    /// = Remarks
    ///
    /// The caller must have the CAP_SYS_ADMIN capability.
    pub fn hang_up(&self) -> Result {
        rv!(ioctl_tiocvhangup(self.fd))
    }

    /// Returns the device associated with this terminal.
    pub fn device(&self) -> Result<Device> {
        let mut dev = 0;
        try!(rv!(ioctl_tiocgdev(self.fd, &mut dev)));
        Ok(Device::from_id(dev, DeviceType::Character))
    }

    /// Discards all unread input of this terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TCFLSH therein
    pub fn discard_input(&self) -> Result {
        rv!(ioctl_tcflsh(self.fd, TCIFLUSH))
    }

    /// Discards all unwritten output of this terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TCFLSH therein
    pub fn discard_output(&self) -> Result {
        rv!(ioctl_tcflsh(self.fd, TCOFLUSH))
    }

    /// Returns the number of pending output bytes.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TIOCOUTQ therein
    pub fn pending_output(&self) -> Result<usize> {
        let mut buf = 0;
        try!(rv!(ioctl_tiocoutq(self.fd, &mut buf)));
        Ok(buf)
    }

    /// Returns the number of pending input bytes.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and FIONREAD therein
    pub fn pending_input(&self) -> Result<usize> {
        let mut buf = 0;
        try!(rv!(ioctl_fionread(self.fd, &mut buf)));
        Ok(buf)
    }

    /// Suspends output.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TCONC therein
    pub fn suspend_output(&self) -> Result {
        rv!(ioctl_tcxonc(self.fd, TCOOFF))
    }

    /// Restarts output.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TCONC therein
    pub fn start_output(&self) -> Result {
        rv!(ioctl_tcxonc(self.fd, TCOON))
    }

    /// Suspends input.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TCONC therein
    pub fn suspend_input(&self) -> Result {
        rv!(ioctl_tcxonc(self.fd, TCIOFF))
    }

    /// Restarts input.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TCONC therein
    pub fn start_input(&self) -> Result {
        rv!(ioctl_tcxonc(self.fd, TCION))
    }

    /// Retrieves the attributes of the terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TCGETS therein
    pub fn attributes(&self) -> Result<TtyAttr> {
        let mut attrs: TtyAttr = mem::zeroed();
        try!(rv!(ioctl_tcgets2(self.fd, &mut attrs.0)));

        if attrs.0.c_cflag & cty::CBAUD != cty::BOTHER {
            let rate = match attrs.0.c_cflag & cty::CBAUD {
                cty::B0       => 0,
                cty::B50      => 50,
                cty::B75      => 75,
                cty::B110     => 110,
                cty::B134     => 134,
                cty::B150     => 150,
                cty::B200     => 200,
                cty::B300     => 300,
                cty::B600     => 600,
                cty::B1200    => 1200,
                cty::B1800    => 1800,
                cty::B2400    => 2400,
                cty::B4800    => 4800,
                cty::B9600    => 9600,
                cty::B19200   => 19200,
                cty::B38400   => 38400,
                cty::B57600   => 57600,
                cty::B115200  => 115200,
                cty::B230400  => 230400,
                cty::B460800  => 460800,
                cty::B500000  => 500000,
                cty::B576000  => 576000,
                cty::B921600  => 921600,
                cty::B1000000 => 1000000,
                cty::B1152000 => 1152000,
                cty::B1500000 => 1500000,
                cty::B2000000 => 2000000,
                cty::B2500000 => 2500000,
                cty::B3000000 => 3000000,
                cty::B3500000 => 3500000,
                cty::B4000000 => 4000000,
                _ => abort!(),
            };
            attrs.0.c_cflag &= !cty::CBAUD;
            attrs.0.c_cflag |= cty::BOTHER;
            attrs.0.c_ispeed = rate;
            attrs.0.c_ospeed = rate;
        }

        Ok(attrs)
    }

    /// Sets the attributes of the terminal.
    ///
    /// = See also
    ///
    /// * link:man:tty_ioctl(4) and TCSETS therein
    pub fn set_attributes(&self, attrs: TtyAttr) -> Result {
        rv!(ioctl_tcsets2(self.fd, &attrs.0))
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

/// Returns whether a file descriptor refers to a tty.
///
/// [argument, fd]
/// The file descriptor to check.
pub fn is_a_tty<F>(fd: &F) -> bool
    where F: FDContainer
{
    Tty::from_borrowed(fd.borrow()).line_discipline().is_ok()
}

/// Hangs up the controlling terminal of this process.
///
/// = Remarks
///
/// This requires the CAP_SYS_TTY_CONFIG capability.
///
/// = See also
///
/// * link:man:vhangup(2)
pub fn hang_up() -> Result {
    rv!(vhangup())
}
