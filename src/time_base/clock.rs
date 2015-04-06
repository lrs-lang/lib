use std::{mem};

use core::cty::{clockid_t, c_int};
use core::syscall::{clock_gettime};
use core::result::{Result};
use core::errno::{Errno};

use super::{Time, time_from_timespec};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Clock(clockid_t);

pub const CLOCK_REALTIME           : Clock = Clock(0);
pub const CLOCK_MONOTONIC          : Clock = Clock(1);
pub const CLOCK_PROCESS_CPUTIME_ID : Clock = Clock(2);
pub const CLOCK_THREAD_CPUTIME_ID  : Clock = Clock(3);
pub const CLOCK_MONOTONIC_RAW      : Clock = Clock(4);
pub const CLOCK_REALTIME_COARSE    : Clock = Clock(5);
pub const CLOCK_MONOTONIC_COARSE   : Clock = Clock(6);
pub const CLOCK_BOOTTIME           : Clock = Clock(7);
pub const CLOCK_REALTIME_ALARM     : Clock = Clock(8);
pub const CLOCK_BOOTTIME_ALARM     : Clock = Clock(9);
pub const CLOCK_SGI_CYCLE          : Clock = Clock(10);
pub const CLOCK_TAI                : Clock = Clock(11);

macro_rules! rv {
    ($x:expr) => { if $x < 0 { Err(Errno(-$x as c_int)) } else { Ok(()) } };
    ($x:expr, -> $t:ty) => { if $x < 0 { Err(Errno(-$x as c_int)) } else { Ok($x as $t) } };
}

impl Clock {
    pub fn get_time(self) -> Result<Time> {
        let mut timespec = unsafe { mem::zeroed() };
        try!(rv!(clock_gettime(self.0, &mut timespec)));
        Ok(time_from_timespec(timespec))
    }
}
