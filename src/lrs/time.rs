// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Time handling.
//!
//! = Description
//!
//! This module provides methods for time handling. The essential types are
//!
//! * `Time` - An offset from an unspecified point in time,
//! * `DateTime` - A time represented in a human-readable format,
//! * `Zone` - A timezone,
//! * `Clock` - A clock that can be used to measure time.
//! * `Timer` - A timer.
//!
//! = Examples
//!
//! The following example prints the current time in the local timezone to stdout:
//!
//! ----
//! use lrs::{time};
//!
//! let now = time::Real::get_time().unwrap();
//! let local = time::Zone::local().unwrap();
//! let expanded = local.expand(now);
//!
//! println!("It is {:02}:{:02}:{:02}", expanded.hour, expanded.minute, expanded.second);
//!
//! // Example output:
//! // It is 13:06:27
//! ----
//!
//! The next example prints the last time the current time zone was changed when viewed
//! from Tokyo:
//!
//! ----
//! use lrs::{file, time};
//!
//! let info = file::info_no_follow("/etc/localtime").unwrap();
//! let last_mod = info.last_modification();
//! let tokyo = time::Zone::load("Asia/Tokyo").unwrap();
//! let expanded = tokyo.expand(last_mod);
//!
//! println!("{:?}", expanded);
//!
//! // Example output:
//! // 2013-08-06T07:06:55+09:00
//! ----
//!
//! The next example creates a timer that starts ticking after five seconds every second,
//! sleeps for ten seconds, and then prints the number of times the timer expired.
//!
//! ----
//! use lrs::time::{self, Time};
//!
//! let timer = time::Real.timer().unwrap();
//! timer.interval_in(Time::seconds(1), Time::seconds(5)).unwrap();
//!
//! time::Real.sleep_for(Time::seconds(10)).unwrap();
//!
//! println!("{:?}", timer.ticks().unwrap());
//!
//! // Example output:
//! // 6
//! ----

pub use lrs_time_base::{Time};
pub use lrs_time_ext::{DateTime, Weekday, Zone};

#[cfg(not(freestanding))] pub use lrs_time_base::timer::{Timer};
#[cfg(not(freestanding))] pub use lrs_time_base::clock::{Clock};
#[cfg(not(freestanding))]
pub use lrs_time_base::clock::{REAL, MONO, PROCESS, THREAD, MONO_RAW, REAL_COARSE,
                               MONO_COARSE, BOOT};
