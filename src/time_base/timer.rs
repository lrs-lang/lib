// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use cty::{c_int, itimerspec, TFD_TIMER_ABSTIME};
use syscall::{close, timerfd_settime, timerfd_gettime, read};
use fd::{FDContainer};
use rv::{retry};

use super::{Time, time_to_timespec, time_from_timespec};

/// A timer.
pub struct Timer {
    fd: c_int,
    owned: bool,
}

impl Timer {
    /// Disables the timer.
    pub fn disable(&self) -> Result {
        let arg = unsafe { mem::zeroed() };
        rv!(timerfd_settime(self.fd, 0, &arg, None))
    }

    /// Sets the timer to expire every `iv` time units.
    pub fn interval(&self, iv: Time) -> Result {
        let arg = itimerspec {
            it_interval: time_to_timespec(iv),
            it_value: time_to_timespec(iv),
        };
        rv!(timerfd_settime(self.fd, 0, &arg, None))
    }

    /// Sets the timer to expire every `iv` time units, starting at the absolute `start`.
    pub fn interval_from(&self, iv: Time, start: Time) -> Result {
        let arg = itimerspec {
            it_interval: time_to_timespec(iv),
            it_value: time_to_timespec(start),
        };
        rv!(timerfd_settime(self.fd, TFD_TIMER_ABSTIME, &arg, None))
    }

    /// Sets the timer to expire every `iv` time units, starting in `when` units.
    pub fn interval_in(&self, iv: Time, when: Time) -> Result {
        let arg = itimerspec {
            it_interval: time_to_timespec(iv),
            it_value: time_to_timespec(when),
        };
        rv!(timerfd_settime(self.fd, 0, &arg, None))
    }

    /// Sets the timer to expire once at the absolute `when`.
    pub fn once_at(&self, when: Time) -> Result {
        let arg = itimerspec {
            it_interval: unsafe { mem::zeroed() },
            it_value: time_to_timespec(when),
        };
        rv!(timerfd_settime(self.fd, TFD_TIMER_ABSTIME, &arg, None))
    }

    /// Sets the timer to expire in `when` time units.
    pub fn once_in(&self, when: Time) -> Result {
        let arg = itimerspec {
            it_interval: unsafe { mem::zeroed() },
            it_value: time_to_timespec(when),
        };
        rv!(timerfd_settime(self.fd, 0, &arg, None))
    }

    /// Returns the status of the timer.
    ///
    /// TODO: Document this.
    pub fn status(&self) -> Result<(Time, Time)> {
        let mut arg = unsafe { mem::zeroed() };
        try!(rv!(timerfd_gettime(self.fd, &mut arg)));
        Ok((time_from_timespec(arg.it_interval), time_from_timespec(arg.it_value)))
    }

    /// Returns the number of times the timer expired since this function was last called.
    pub fn ticks(&self) -> Result<u64> {
        let mut buf = [0; 8];
        try!(retry(|| read(self.fd, &mut buf[..])));
        Ok(unsafe { mem::cast(buf) })
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if self.owned {
            close(self.fd);
        }
    }
}

impl FDContainer for Timer {
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

    fn from_owned(fd: c_int) -> Timer {
        Timer { fd: fd, owned: true }
    }

    fn from_borrowed(fd: c_int) -> Timer {
        Timer { fd: fd, owned: false }
    }
}
