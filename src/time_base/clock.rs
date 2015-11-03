// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]

use base::prelude::*;
use core::{mem};
use cty::{clockid_t, c_int, TIMER_ABSTIME, TFD_NONBLOCK};
use syscall::{clock_gettime, clock_settime, clock_getres, clock_nanosleep,
                    timerfd_create};
use rv::{retry};
use fd::{FDContainer};

use super::{Time, time_from_timespec, time_to_timespec};
use timer::{Timer};

/// A clock that can be used to measure time.
///
/// [field, 1]
/// The integer representing the clock.
#[derive(Pod, Eq)]
pub struct Clock(pub clockid_t);

/// Real ("wall time") clock that measures the time since 1970-01-01T00:00:00+00:00.
pub const REAL: Clock = Clock(0);

/// Real coarse ("wall time") clock that measures the time since
/// 1970-01-01T00:00:00+00:00.
///
/// = Remarks
///
/// This is less precise but more efficient than `REAL`.
pub const REAL_COARSE: Clock = Clock(5);

/// A monotonic clock since some arbitrary point in the past which isn't affected by time
/// jumps.
pub const MONO: Clock = Clock(1);

/// A coarse monotonic clock since some arbitrary point in the past which isn't affected
/// by time jumps.
///
/// = Remarks
///
/// This is less precise but more efficient than `MONO`.
pub const MONO_COARSE: Clock = Clock(6);

/// A monotonic clock since some arbitrary point in the past which isn't affected by time
/// jumps or time adjustments.
pub const MONO_RAW: Clock = Clock(4);

/// A clock that measures the CPU time used by this process.
pub const PROCESS: Clock = Clock(2);

/// A clock that measures the CPU time used by this thread.
pub const THREAD: Clock = Clock(3);

/// A monotonic clock since some arbitrary point in the past which isn't affected by time
/// jumps and continues to run while the system is suspended.
pub const BOOT: Clock = Clock(7);

// TODO: Names for the clocks below

// pub const CLOCK_REALTIME_ALARM     : Clock = Clock(8);
// pub const CLOCK_BOOTTIME_ALARM     : Clock = Clock(9);
// pub const CLOCK_SGI_CYCLE          : Clock = Clock(10);
// pub const CLOCK_TAI                : Clock = Clock(11);

impl Clock {
    /// Returns the current time of the clock.
    pub fn get_time(self) -> Result<Time> {
        let mut timespec = mem::zeroed();
        try!(rv!(clock_gettime(self.0, &mut timespec)));
        Ok(time_from_timespec(timespec))
    }

    /// Sets the time of the clock.
    ///
    /// [argument, t]
    /// The new time of the clock.
    pub fn set_time(self, t: Time) -> Result {
        let timespec = time_to_timespec(t);
        rv!(clock_settime(self.0, &timespec))
    }

    /// Returns the resolution of the clock.
    pub fn resolution(self) -> Result<Time> {
        let mut timespec = mem::zeroed();
        try!(rv!(clock_getres(self.0, &mut timespec)));
        Ok(time_from_timespec(timespec))
    }

    /// Sleeps until an absolute time.
    ///
    /// [argument, t]
    /// The time until which to sleep.
    pub fn sleep_to(self, t: Time) -> Result {
        let time = time_to_timespec(t);
        let mut rem = mem::zeroed();
        retry(|| clock_nanosleep(self.0, TIMER_ABSTIME, &time, &mut rem)).map(|_| ())
    }

    /// Sleeps for an amount of time.
    ///
    /// [argument, t]
    /// The amount of time to sleep.
    pub fn sleep_for(self, t: Time) -> Result {
        let now = try!(self.get_time());
        self.sleep_to(now + t)
    }

    /// Creates a new timer.
    pub fn timer(self) -> Result<Timer> {
        let timer = try!(rv!(timerfd_create(self.0, 0), -> c_int));
        Ok(Timer::from_owned(timer))
    }

    /// Creates a new non-blocking timer.
    pub fn timer_non_blocking(self) -> Result<Timer> {
        let timer = try!(rv!(timerfd_create(self.0, TFD_NONBLOCK), -> c_int));
        Ok(Timer::from_owned(timer))
    }
}
