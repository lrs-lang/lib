// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_poll"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive, associated_consts)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base       as base;
extern crate lrs_cty        as cty;
extern crate lrs_syscall    as syscall;
extern crate lrs_fd         as fd;
extern crate lrs_rv         as rv;
extern crate lrs_saturating as saturating;
extern crate lrs_fmt        as fmt;
extern crate lrs_time_base  as time_base;

use base::prelude::*;
mod std { pub use base::std::*; pub use {cty}; }

use core::{mem};
use core::ops::{BitOr, Not, BitAnd};
use base::undef::{UndefState};
use cty::{
    c_int, EPOLL_CTL_ADD, EPOLL_CTL_MOD, EPOLL_CTL_DEL, epoll_event,
    POLLIN, POLLOUT, POLLRDHUP, POLLPRI, EPOLLET, EPOLLONESHOT, EPOLLWAKEUP,
};
use syscall::{epoll_create, epoll_ctl, epoll_pwait, close};
use fd::{FDContainer};
use rv::{retry};
use saturating::{SaturatingCast};
use fmt::{Debug, Write};

use time_base::{Time};

/// Flags for modifying a polled file descriptor.
///
/// [field, 1]
/// The numeric representation of the flags.
#[derive(Pod, Eq)]
pub struct PollFlags(pub u32);

impl BitOr for PollFlags {
    type Output = PollFlags;
    fn bitor(self, other: PollFlags) -> PollFlags {
        PollFlags(self.0 | other.0)
    }
}

impl BitAnd for PollFlags {
    type Output = PollFlags;
    fn bitand(self, other: PollFlags) -> PollFlags {
        PollFlags(self.0 & other.0)
    }
}

impl Not for PollFlags {
    type Output = PollFlags;
    fn not(self) -> PollFlags {
        PollFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const POLL_NONE: PollFlags = PollFlags(0);

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: PollFlags = PollFlags($val);)*

        impl Debug for PollFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & $val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                let _ = first;
                Ok(())
            }
        }
    }
}

create! {
    #[doc = "Signal when the file descriptor becomes ready for reading.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:epoll_ctl(2) and EPOLLIN therein"]
    flag POLL_READ = POLLIN;

    #[doc = "Signal when the file descriptor becomes ready for writing.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:epoll_ctl(2) and EPOLLOUT therein"]
    flag POLL_WRITE = POLLOUT;

    #[doc = "Signal when the peer has hung up his write end.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:epoll_ctl(2) and EPOLLRDHUP therein"]
    flag POLL_READ_HANG_UP = POLLRDHUP;

    #[doc = "Signal when priority data becomes available for reading.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:epoll_ctl(2) and EPOLLPRI therein"]
    flag POLL_PRIORITY = POLLPRI;

    #[doc = "Enable edge-triggered polling.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:epoll_ctl(2) and EPOLLET therein"]
    flag POLL_EDGE_TRIGGERED = EPOLLET;

    #[doc = "Signal only once for this file descriptor.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:epoll_ctl(2) and EPOLLONESHOT therein"]
    flag POLL_ONE_SHOT = EPOLLONESHOT;

    #[doc = "Don't allow the system to suspend after data becomes available.\n"]
    #[doc = "= Remarks"]
    #[doc = "== Kernel versions"]
    #[doc = "The required kernel version is 3.5.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:epoll_ctl(2) and EPOLLWAKEUP therein"]
    flag POLL_WAKE_UP = EPOLLWAKEUP;
}

impl PollFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: PollFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: PollFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: PollFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

/// Constructor for creating `Event` arrays.
pub const EMPTY_EVENT: Event = Event { data: epoll_event { events: 0, data: 0 } };

/// An event returned after polling.
#[repr(C)]
#[derive(Pod, Eq)]
pub struct Event {
    data: epoll_event,
}

impl Event {
    /// Returns whether the file descriptor is readable.
    pub fn is_read(self) -> bool { self.data.events & cty::POLLIN != 0 }

    /// Returns whether the file descriptor is writable.
    pub fn is_write(self) -> bool { self.data.events & cty::POLLOUT != 0 }

    /// Returns whether the peer has hung up his write end.
    pub fn is_read_hang_up(self) -> bool { self.data.events & cty::POLLRDHUP != 0 }

    /// Returns whether priority data is available for reading.
    pub fn is_priority(self) -> bool { self.data.events & cty::POLLPRI != 0 }

    /// Returns whether an error condition occurred on the file descriptor.
    pub fn is_error(self) -> bool { self.data.events & cty::POLLERR != 0 }

    /// Returns whether the peer has hung up.
    pub fn is_hang_up(self) -> bool { self.data.events & cty::POLLHUP != 0 }

    /// Returns the associated file descriptor.
    pub fn fd(self) -> c_int { self.data.data as c_int }
}

impl Debug for Event {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        w.write(b"Event").ignore_ok()
    }
}

/// An epoll instance.
#[derive(Eq)]
pub struct Epoll {
    fd: c_int,
    owned: bool,
}

impl Epoll {
    /// Creates a new epoll instance.
    ///
    /// = Remarks
    ///
    /// Unless lrs has been compiled with the `no-auto-cloexec` flag, this epoll instance
    /// is created with the `EPOLL_CLOEXEC` flag set.
    ///
    /// = See also
    ///
    /// * link:man:epoll_create(2)
    pub fn new() -> Result<Epoll> {
        let fd = try!(rv!(epoll_create(0), -> c_int));
        Ok(Epoll { fd: fd, owned: true })
    }

    /// Adds a file descriptor to the epoll instance.
    ///
    /// [argument, fd]
    /// The file descriptor to add.
    ///
    /// [argument, flags]
    /// The flags to be set.
    pub fn add<T: FDContainer>(&self, fd: &T, flags: PollFlags) -> Result {
        let mut event = epoll_event { events: flags.0, data: fd.borrow() as u64 };
        rv!(epoll_ctl(self.fd, EPOLL_CTL_ADD, fd.borrow(), Some(&mut event)))
    }

    /// Modifies the flags associated with an added file descriptor.
    ///
    /// [argument, fd]
    /// The file descriptor to modify.
    ///
    /// [argument, flags]
    /// The new flags.
    pub fn modify<T: FDContainer>(&self, fd: &T, flags: PollFlags) -> Result {
        let mut event = epoll_event { events: flags.0, data: fd.borrow() as u64 };
        rv!(epoll_ctl(self.fd, EPOLL_CTL_MOD, fd.borrow(), Some(&mut event)))
    }

    /// Removes a file descriptor from an epoll instance.
    ///
    /// [argument, fd]
    /// The file descriptor to remove.
    pub fn remove<T: FDContainer>(&self, fd: &T) -> Result {
        rv!(epoll_ctl(self.fd, EPOLL_CTL_DEL, fd.borrow(), None))
    }

    /// Waits for an event to occur.
    ///
    /// [argument, events]
    /// The buffer in which events will be stored.
    ///
    /// [return_value]
    /// Returns a slice of events that occurred.
    pub fn wait<'a>(&self, events: &'a mut [Event]) -> Result<&'a mut [Event]> {
        self.wait_common(events, -1)
    }

    /// Waits for an event to occur or a timeout to expire.
    ///
    /// [argument, events]
    /// The buffer in which events will be stored.
    ///
    /// [argument, timeout]
    /// The maximum time this call will block.
    ///
    /// [return_value]
    /// Returns a slice of events that occurred.
    pub fn wait_timeout<'a>(&self, events: &'a mut [Event],
                            timeout: Time) -> Result<&'a mut [Event]> {
        let timeout = timeout.seconds * 1_000 + timeout.nanoseconds / 1_000_000;
        self.wait_common(events, timeout.saturating_cast())
    }

    fn wait_common<'a>(&self, events: &'a mut [Event],
                       timeout: c_int) -> Result<&'a mut [Event]> {
        let events: &mut [epoll_event] = unsafe { mem::cast(events) };
        let ret = try!(retry(|| epoll_pwait(self.fd, events, timeout, None)));
        let events: &mut [Event] = unsafe { mem::cast(events) };
        Ok(&mut events[..ret as usize])
    }
}

unsafe impl UndefState for Epoll {
    fn num() -> usize { bool::num() }

    unsafe fn set_undef(val: *mut Epoll, n: usize) {
        bool::set_undef(&mut (*val).owned, n);
    }

    unsafe fn is_undef(val: *const Epoll, n: usize) -> bool {
        bool::is_undef(&(*val).owned, n)
    }
}

impl Drop for Epoll {
    fn drop(&mut self) {
        if self.owned {
            close(self.fd);
        }
    }
}

impl FDContainer for Epoll {
    fn unwrap(self) -> c_int {
        let fd = self.fd;
        mem::forget(self);
        fd
    }

    fn is_owned(&self) -> bool {
        self.owned
    }

    fn borrow(&self) -> c_int {
        self.fd
    }

    fn from_owned(fd: c_int) -> Epoll {
        Epoll { fd: fd, owned: true }
    }

    fn from_borrowed(fd: c_int) -> Epoll {
        Epoll { fd: fd, owned: false }
    }
}
