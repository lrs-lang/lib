// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_poll"]
#![crate_type = "lib"]

#[macro_use]
extern crate linux_core as core;
extern crate linux_time_base as time_base;

use std::{mem};

use core::cty::{self, c_int, EPOLL_CLOEXEC, EPOLL_CTL_ADD, EPOLL_CTL_MOD, EPOLL_CTL_DEL,
                epoll_event};
use core::result::{Result};
use core::syscall::{epoll_create1, epoll_ctl, epoll_pwait, close};
use core::fd_container::{FDContainer};
use core::util::{retry};
use core::ext::{SaturatingCast};

use time_base::{Time};

/// Flags for modifying a polled file descriptor.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Flags(u32);

impl Flags {
    /// Creates a new Flags with all flags unset.
    pub fn new() -> Flags {
        Flags(0)
    }

    /// If set the poll will check for readability.
    pub fn readable(&self)       -> bool { self.0 & cty::EPOLLIN      != 0 }
    /// If set the poll will check for writability.
    pub fn writable(&self)       -> bool { self.0 & cty::EPOLLOUT     != 0 }
    /// If set the poll checks that the peer has hung up his write end.
    pub fn read_hang_up(&self)   -> bool { self.0 & cty::EPOLLRDHUP  != 0 }
    /// If set the poll checks for priority data.
    pub fn priority(&self)       -> bool { self.0 & cty::EPOLLPRI    != 0 }
    /// If set the poll is edge triggered.
    pub fn edge_triggered(&self) -> bool { self.0 & cty::EPOLLET      != 0 }
    /// If set the fd will checked only once and then has to be re-enabled. 
    pub fn one_shot(&self)       -> bool { self.0 & cty::EPOLLONESHOT != 0 }
    /// If set then the system cannot suspend after this file descriptor becomes ready and
    /// before another call to `wait` is made.
    pub fn wake_up(&self)        -> bool { self.0 & cty::EPOLLWAKEUP  != 0 }

    pub fn set_readable(&mut       self, val: bool) { self.set_bit(cty::EPOLLIN      , val) }
    pub fn set_writable(&mut       self, val: bool) { self.set_bit(cty::EPOLLOUT     , val) }
    pub fn set_read_hang_up(&mut   self, val: bool) { self.set_bit(cty::EPOLLRDHUP  , val) }
    pub fn set_priority(&mut       self, val: bool) { self.set_bit(cty::EPOLLPRI    , val) }
    pub fn set_edge_triggered(&mut self, val: bool) { self.set_bit(cty::EPOLLET      , val) }
    pub fn set_one_shot(&mut       self, val: bool) { self.set_bit(cty::EPOLLONESHOT , val) }
    pub fn set_wake_up(&mut        self, val: bool) { self.set_bit(cty::EPOLLWAKEUP  , val) }

    fn set_bit(&mut self, bit: u32, val: bool) {
        if val {
            self.0 |= bit
        } else {
            self.0 &= !bit
        }
    }
}

/// Constructor for creating `Event` arrays.
pub const EMPTY_EVENT: Event = Event { data: epoll_event { events: 0, data: 0 } };

/// An event returned from a `wait` call.
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Event {
    data: epoll_event,
}

impl Event {
    /// If set the descriptor is readable.
    pub fn readable(self) -> bool { self.data.events & cty::EPOLLIN != 0 }

    /// If set the descriptor is writable.
    pub fn writable(self) -> bool { self.data.events & cty::EPOLLOUT != 0 }

    /// If set the peer has hung up his write end.
    pub fn read_hang_up(self) -> bool { self.data.events & cty::EPOLLRDHUP != 0 }

    /// If set there is priority data.
    pub fn priority(self) -> bool { self.data.events & cty::EPOLLPRI != 0 }

    /// If set an error condition happened on the file descriptor.
    pub fn error(self) -> bool { self.data.events & cty::EPOLLERR != 0 }

    /// If set the file descriptor was hung up.
    pub fn hang_up(self) -> bool { self.data.events & cty::EPOLLHUP != 0 }

    /// Returns the associated file descriptor.
    pub fn fd(self) -> c_int { self.data.data as c_int }
}

/// An epoll instance.
#[derive(Debug, Eq, PartialEq)]
pub struct Epoll {
    fd: c_int,
    owned: bool,
}

impl Epoll {
    /// Creates a new epoll instance.
    pub fn new() -> Result<Epoll> {
        let fd = try!(rv!(epoll_create1(EPOLL_CLOEXEC), -> c_int));
        Ok(Epoll { fd: fd, owned: true })
    }

    /// Adds a file descriptor to the epoll instance.
    pub fn add<T: FDContainer>(&self, fd: &T, flags: Flags) -> Result {
        let mut event = epoll_event { events: flags.0, data: fd.borrow() as u64 };
        rv!(epoll_ctl(self.fd, EPOLL_CTL_ADD, fd.borrow(), Some(&mut event)))
    }

    /// Modifies the flags associated with an added file descriptor.
    pub fn modify<T: FDContainer>(&self, fd: &T, flags: Flags) -> Result {
        let mut event = epoll_event { events: flags.0, data: fd.borrow() as u64 };
        rv!(epoll_ctl(self.fd, EPOLL_CTL_MOD, fd.borrow(), Some(&mut event)))
    }

    /// Removes a file descriptor from an epoll instance.
    pub fn remove<T: FDContainer>(&self, fd: &T) -> Result {
        rv!(epoll_ctl(self.fd, EPOLL_CTL_DEL, fd.borrow(), None))
    }

    /// Waits for an event to occur.
    pub fn wait<'a>(&self, events: &'a mut [Event]) -> Result<&'a mut [Event]> {
        self.wait_common(events, -1)
    }

    /// Waits for an event to occur or the timeout to expire.
    pub fn wait_timeout<'a>(&self, events: &'a mut [Event],
                            timeout: Time) -> Result<&'a mut [Event]> {
        let timeout = timeout.seconds * 1_000 + timeout.nanoseconds / 1_000_000;
        self.wait_common(events, timeout.saturating_cast())
    }

    fn wait_common<'a>(&self, events: &'a mut [Event],
                           timeout: c_int) -> Result<&'a mut [Event]> {
        let events: &mut [epoll_event] = unsafe { mem::transmute(events) };
        let ret = try!(retry(|| epoll_pwait(self.fd, events, timeout, None)));
        let events: &mut [Event] = unsafe { mem::transmute(events) };
        Ok(&mut events[..ret as usize])
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
        unsafe { mem::forget(self); }
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
