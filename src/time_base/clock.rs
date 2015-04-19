// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]

#[prelude_import] use base::prelude::*;
use base::{mem};
use base::cty::{clockid_t, c_int, TIMER_ABSTIME, TFD_NONBLOCK, TFD_CLOEXEC};
use base::syscall::{clock_gettime, clock_settime, clock_getres, clock_nanosleep,
                    timerfd_create};
use base::result::{Result};
use base::util::{retry};
use base::fd_container::{FDContainer};

use super::{Time, time_from_timespec, time_to_timespec};
use timer::{Timer};

/// A clock that can be used to measure time.
#[derive(Copy, Eq)]
pub struct Clock(clockid_t);

/// Real ("wall time") clock that measures the time since 1970-01-01T00:00:00+00:00.
pub const Real: Clock = Clock(0);
/// Like `Real` but more efficient and less precise.
pub const RealCoarse: Clock = Clock(5);
/// A monotonic clock since some arbitrary point in the past which isn't affected by time
/// jumps.
pub const Mono: Clock = Clock(1);
/// Like `Mono` but more efficient and less precise.
pub const MonoCoarse: Clock = Clock(6);
/// Like `Mono` but it's also not affected by `adjtime`.
pub const MonoRaw: Clock = Clock(4);
/// A clock that measures the CPU time used by this process.
pub const Process: Clock = Clock(2);
/// A clock that measures the CPU time used by this thread.
pub const Thread: Clock = Clock(3);
/// Like `Mono` but doesn't stop when the machine is suspended.
pub const Boot: Clock = Clock(7);

// TODO: Names for the clocks below

pub const CLOCK_REALTIME_ALARM     : Clock = Clock(8);
pub const CLOCK_BOOTTIME_ALARM     : Clock = Clock(9);
pub const CLOCK_SGI_CYCLE          : Clock = Clock(10);
pub const CLOCK_TAI                : Clock = Clock(11);

impl Clock {
    /// Returns the current time of the clock.
    pub fn get_time(self) -> Result<Time> {
        let mut timespec = unsafe { mem::zeroed() };
        try!(rv!(clock_gettime(self.0, &mut timespec)));
        Ok(time_from_timespec(timespec))
    }

    /// Sets the time of the clock.
    pub fn set_time(self, t: Time) -> Result {
        let timespec = time_to_timespec(t);
        rv!(clock_settime(self.0, &timespec))
    }

    /// Returns the resolution of the clock.
    pub fn resolution(self) -> Result<Time> {
        let mut timespec = unsafe { mem::zeroed() };
        try!(rv!(clock_getres(self.0, &mut timespec)));
        Ok(time_from_timespec(timespec))
    }

    /// Sleeps to the specified absolute time.
    pub fn sleep_to(self, t: Time) -> Result {
        let time = time_to_timespec(t);
        let mut rem = unsafe { mem::zeroed() };
        retry(|| clock_nanosleep(self.0, TIMER_ABSTIME, &time, &mut rem)).map(|_| ())
    }

    /// Sleeps for the specified amount of time.
    pub fn sleep_for(self, t: Time) -> Result {
        let now = try!(self.get_time());
        self.sleep_to(now + t)
    }

    /// Creates a new timer.
    pub fn timer(self) -> Result<Timer> {
        let timer = try!(rv!(timerfd_create(self.0, TFD_CLOEXEC), -> c_int));
        Ok(Timer::from_owned(timer))
    }

    /// Creates a new non-blocking timer.
    pub fn timer_non_blocking(self) -> Result<Timer> {
        let timer = try!(rv!(timerfd_create(self.0, TFD_NONBLOCK|TFD_CLOEXEC), -> c_int));
        Ok(Timer::from_owned(timer))
    }
}
