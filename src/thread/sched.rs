// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]

use base::prelude::*;
use core::ops::{BitOr, Not, BitAnd};
use core::{mem};
use cty::alias::{ProcessId, UserId};
use cty::{
    self, c_int, sched_attr, PRIO_PROCESS, PRIO_PGRP, PRIO_USER,
};
use syscall::{
    sched_get_priority_max, sched_get_priority_min, sched_setattr, sched_getattr,
    sched_rr_get_interval, getpriority, setpriority,
};
use time_base::{Time, time_from_timespec};
use fmt::{Debug, Write};

/// A process scheduler.
///
/// [field, 1]
/// The integer constant associated with the scheduler.
///
/// = Remarks
///
/// :scheds: link:lrs::thread::sched[sched]
///
/// See {sched} for pre-defined schedulers.
///
/// = See also
///
/// * link:man:sched(7)
/// * {sched}
#[derive(Pod, Eq)]
pub struct Scheduler(pub c_int);

impl Scheduler {
    /// Returns the minimum priority of this scheduler.
    ///
    /// = See also
    ///
    /// * link:man:sched_get_priority_min(2)
    pub fn min_priority(self) -> Result<u8> {
        match self {
            Fifo | Rr => Ok(1),
            Deadline | Normal | Batch | Idle => Ok(0),
            _ => Ok(try!(rv!(sched_get_priority_min(self.0), -> u8))),
        }
    }

    /// Returns the maximum priority of this scheduler.
    ///
    /// = See also
    ///
    /// * link:man:sched_get_priority_max(2)
    pub fn max_priority(self) -> Result<u8> {
        match self {
            Fifo | Rr => Ok(99),
            Deadline | Normal | Batch | Idle => Ok(0),
            _ => Ok(try!(rv!(sched_get_priority_max(self.0), -> u8))),
        }
    }
}

macro_rules! create {
    ($($(#[$meta:meta])* sched $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: Scheduler = Scheduler(cty::$val);)*

        impl Debug for Scheduler {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let s = match *self {
                    $($name => stringify!($name),)*
                    _ => return write!(w, "Unknown({})", self.0),
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    #[doc = "The default scheduler.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sched(7) and SCHED_OTHER therein"]
    sched Normal   = SCHED_NORMAL;

    #[doc = "The FIFO scheduler.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sched(7) and SCHED_FIFO therein"]
    sched Fifo     = SCHED_FIFO;

    #[doc = "The Round-robin scheduler.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sched(7) and SCHED_RR therein"]
    sched Rr       = SCHED_RR;

    #[doc = "The batch scheduler.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sched(7) and SCHED_BATCH therein"]
    sched Batch    = SCHED_BATCH;

    #[doc = "The idle scheduler.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sched(7) and SCHED_IDLE therein"]
    sched Idle     = SCHED_IDLE;

    #[doc = "The Deadline scheduler.\n"]
    #[doc = "= Remarks"]
    #[doc = "== Kernel versions"]
    #[doc = "The required kernel version in 3.14."]
    #[doc = "= See also"]
    #[doc = "* link:man:sched(7) and SCHED_DEADLINE therein"]
    sched Deadline = SCHED_DEADLINE;
}

/// Flags for schedulers.
#[derive(Pod, Eq)]
pub struct SchedFlags(pub u64);

impl BitOr for SchedFlags {
    type Output = SchedFlags;
    fn bitor(self, other: SchedFlags) -> SchedFlags {
        SchedFlags(self.0 | other.0)
    }
}

impl BitAnd for SchedFlags {
    type Output = SchedFlags;
    fn bitand(self, other: SchedFlags) -> SchedFlags {
        SchedFlags(self.0 & other.0)
    }
}

impl Not for SchedFlags {
    type Output = SchedFlags;
    fn not(self) -> SchedFlags {
        SchedFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const SCHED_NONE: SchedFlags = SchedFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: SchedFlags = SchedFlags(cty::$val as u64);)*

        impl Debug for SchedFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & cty::$val as u64 != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("SCHED_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Don't have children inherit the scheduling policy.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sched_setattr(3) and SCHED_FLAG_RESET_ON_FORK therein"]
    flag SCHED_RESET_ON_FORK = SCHED_FLAG_RESET_ON_FORK;
}

/// Scheduler attributes.
///
/// = See also
///
/// * link:man:sched_setattr(2)
pub struct SchedAttr {
    /// The scheduler itself.
    pub scheduler: Scheduler,
    /// Flags that modify the scheduler behavior.
    pub flags: SchedFlags,
    /// The nice value of the thread.
    pub nice: u8,
    /// The static priority of the thread.
    pub priority: u8,
    /// The runtime attribute of the Deadline scheduler.
    pub runtime: u64,
    /// The deadline attribute of the Deadline scheduler.
    pub deadline: u64,
    /// The period attribute of the Deadline scheduler.
    pub period: u64,
}

/// Set a thread's scheduler and its arguments.
///
/// [argument, thread]
/// The thread whose scheduler to set, or `0` for this thread.
///
/// [argument, attributes]
/// The scheduler and attributes to set.
///
/// = Remarks
///
/// == Kernel version
///
/// The required kernel version is 3.14.
///
/// = See also
///
/// * link:man;sched_setattr(2)
pub fn set_scheduler(thread: ProcessId, attributes: SchedAttr) -> Result {
    let mut attr = sched_attr {
        size: mem::size_of::<sched_attr>() as u32,
        sched_policy: attributes.scheduler.0 as u32,
        sched_flags: attributes.flags.0 as u64,
        sched_nice: 20 - attributes.nice as i32,
        sched_priority: attributes.priority as u32,
        sched_runtime: attributes.runtime,
        sched_deadline: attributes.deadline,
        sched_period: attributes.period,
    };
    rv!(sched_setattr(thread, &mut attr, 0))
}

/// Get a thread's scheduler and its arguments.
///
/// [argument, thread]
/// The thread whose scheduler to return, or `0` for this thread.
///
/// = Remarks
///
/// == Kernel version
///
/// The required kernel version is 3.14.
///
/// = See also
///
/// * link:man;sched_getattr(2)
pub fn scheduler(thread: ProcessId) -> Result<SchedAttr> {
    let mut attr = mem::zeroed();
    try!(rv!(sched_getattr(thread, &mut attr, 0)));
    Ok(SchedAttr {
        scheduler: Scheduler(attr.sched_policy as c_int),
        flags: SchedFlags(attr.sched_flags),
        nice: (20 - attr.sched_nice) as u8,
        priority: attr.sched_priority as u8,
        runtime: attr.sched_runtime,
        deadline: attr.sched_deadline,
        period: attr.sched_period,
    })
}

/// Returns the round-robin time slice of a thread.
///
/// [argument, thread]
/// The thread to inspect, or `0` for this thread.
///
/// = Remarks
///
/// The thread should be running the round-robin scheduler.
///
/// = See also
///
/// * link:man:sched_rr_get_interval(2)
pub fn round_robin_time_slice(thread: ProcessId) -> Result<Time> {
    let mut time = mem::zeroed();
    try!(rv!(sched_rr_get_interval(thread, &mut time)));
    Ok(time_from_timespec(time))
}

/// Returns the niceness of a process.
///
/// [argument, id]
/// The process to inspect, or `0` for this process.
///
/// = Remarks
///
/// The niceness is a value between 1 and 40, with higher values meaning more favorable
/// scheduling.
///
/// = See also
///
/// * link:man:getpriority(2)
pub fn process_niceness(id: ProcessId) -> Result<u8> {
    rv!(getpriority(PRIO_PROCESS, id), -> u8)
}

/// Returns the niceness of a process group.
///
/// [argument, id]
/// The process group to inspect, or `0` for this process group.
///
/// = Remarks
///
/// The niceness is a value between 1 and 40, with higher values meaning more favorable
/// scheduling.
///
/// = See also
///
/// * link:man:getpriority(2)
pub fn group_niceness(id: ProcessId) -> Result<u8> {
    rv!(getpriority(PRIO_PGRP, id), -> u8)
}

/// Returns the niceness of a user.
///
/// [argument, id]
/// The user to inspect, or `0` for the real user id of this thread.
///
/// = Remarks
///
/// The niceness is a value between 1 and 40, with higher values meaning more favorable
/// scheduling.
///
/// = See also
///
/// * link:man:getpriority(2)
pub fn user_niceness(id: UserId) -> Result<u8> {
    rv!(getpriority(PRIO_USER, id as i32), -> u8)
}

/// Sets the niceness of a process.
///
/// [argument, id]
/// The process to modify, or `0` for this process.
///
/// [argument, niceness]
/// The new niceness.
///
/// = Remarks
///
/// The niceness is a value between 1 and 40, with higher values meaning more favorable
/// scheduling.
///
/// = See also
///
/// * link:man:setpriority(2)
pub fn set_process_niceness(id: ProcessId, niceness: u8) -> Result {
    rv!(setpriority(PRIO_PROCESS, id, 20 - niceness as c_int))
}

/// Sets the niceness of a process group.
///
/// [argument, id]
/// The process group to modify, or `0` for this process group.
///
/// [argument, niceness]
/// The new niceness.
///
/// = Remarks
///
/// The niceness is a value between 1 and 40, with higher values meaning more favorable
/// scheduling.
///
/// = See also
///
/// * link:man:setpriority(2)
pub fn set_group_niceness(id: ProcessId, niceness: u8) -> Result {
    rv!(setpriority(PRIO_PGRP, id, 20 - niceness as c_int))
}

/// Sets the niceness of a user.
///
/// [argument, id]
/// The user to modify, or `0` for the real user id of this thread.
///
/// [argument, niceness]
/// The new niceness.
///
/// = Remarks
///
/// The niceness is a value between 1 and 40, with higher values meaning more favorable
/// scheduling.
///
/// = See also
///
/// * link:man:setpriority(2)
pub fn set_user_niceness(id: UserId, niceness: u8) -> Result {
    rv!(setpriority(PRIO_USER, id as i32, 20 - niceness as c_int))
}
