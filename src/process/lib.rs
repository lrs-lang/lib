// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_process"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts)]

extern crate lrs_base as base;
extern crate lrs_syscall as syscall;
extern crate lrs_cty as cty;
extern crate lrs_fmt as fmt;
extern crate lrs_str_one as str_one;
extern crate lrs_str_two as str_two;
extern crate lrs_str_three as str_three;
extern crate lrs_alloc as alloc;
extern crate lrs_c_ptr_ptr as c_ptr_ptr;
extern crate lrs_rt as rt;
extern crate lrs_file as file;
extern crate lrs_rmo as rmo;
extern crate lrs_rv as rv;
extern crate lrs_time_base as time_base;
extern crate lrs_env as env;

mod std { pub use fmt::std::*; pub use {cty}; }

use base::prelude::*;
use core::{mem};
use syscall::{
    getpid, getppid, exit_group, umask, times, setsid, getsid, setpgid, getpgid,
    getrusage, prlimit,
};
use cty::alias::{ProcessId};
use cty::{c_int, tms, rusage, rlimit64};
use file::flags::{Mode};
use time_base::{Time};
use res_user::{ResourceUser};
use fmt::{Debug, Write};
use res::{Resource};

pub mod exec;
pub mod wait;
pub mod res_user;
pub mod res;

/// Returns the process id of this process.
pub fn process_id() -> ProcessId {
    getpid()
}

/// Returns the process id of the parent process.
pub fn parent_process_id() -> ProcessId {
    getppid()
}

/// Exits the process.
///
/// [argument, code]
/// The exit code that will be shown to the parent process.
///
/// = See also
///
/// * link:man:exit_group(2)
pub fn exit(code: u8) -> ! {
    exit_group(code as c_int);
}

/// Sets the file mode creation mask of the process.
///
/// [argument, mode]
/// The mode to be masked.
///
/// [return_value]
/// Returns the previous mask.
///
/// = Remarks
///
/// The mask will be *subtracted* from the mask used in `open` etc. That is, if a bit is
/// set in the mask, it will be unset in calls to `open` etc.
///
/// = See also
///
/// * link:man:umask(2)
pub fn set_file_mask(mode: Mode) -> Mode {
    Mode(umask(mode.0))
}

/// The times used by a process and its children.
pub struct Times {
    data: tms,
}

impl Times {
    /// Returns the user-space time used by this process.
    pub fn user_time(&self) -> Time {
        Time::milliseconds(self.data.tms_utime as i64 * 10)
    }

    /// Returns the kernel-space time used by this process.
    pub fn kernel_time(&self) -> Time {
        Time::milliseconds(self.data.tms_stime as i64 * 10)
    }

    /// Returns the user-space time used by the children of this process.
    ///
    /// = Remarks
    ///
    /// Only the time used by children that have been reaped is counted.
    pub fn children_user_time(&self) -> Time {
        Time::milliseconds(self.data.tms_cutime as i64 * 10)
    }

    /// Returns the kernel-space time used by the children of this process.
    ///
    /// = Remarks
    ///
    /// Only the time used by children that have been reaped is counted.
    pub fn children_kernel_time(&self) -> Time {
        Time::milliseconds(self.data.tms_cstime as i64 * 10)
    }
}

/// Returns the CPU times used by this process and its children.
///
/// = See also
///
/// * link:man:times(2)
pub fn used_time() -> Result<Times> {
    let mut data = mem::zeroed();
    try!(rv!(times(&mut data)));
    Ok(Times { data: data })
}

/// Creates a new session.
///
/// [return_value]
/// Returns the session id of the new session.
///
/// = Remarks
///
/// This fails if the current process is already a group leader.
///
/// = See also
///
/// * link:man:setsid(2)
pub fn new_session() -> Result<ProcessId> {
    rv!(setsid(), -> ProcessId)
}

/// Get the session id of this or another process.
///
/// [argument, pid]
/// The process whose session id to return or `None` to get the session id of this
/// process.
///
/// = See also
///
/// * link:man:getsid(2)
pub fn session(pid: Option<ProcessId>) -> Result<ProcessId> {
    rv!(getsid(pid.unwrap_or(0)), -> ProcessId)
}

/// Sets the process group of this or another process.
///
/// [argument, process]
/// The process to move to the process group or `None` to move this process.
///
/// [argument, group]
/// The new group of the process.
///
/// = See also
///
/// * link:man:setpgid(2)
pub fn set_process_group(process: Option<ProcessId>, group: ProcessId) -> Result {
    rv!(setpgid(process.unwrap_or(0), group))
}

/// Get the process group of this or another process.
///
/// [argument, pid]
/// The process whose process group to return or `None` to get the process group of this
/// process.
///
/// = See also
///
/// * link:man:getpgid(2)
pub fn process_group(pid: Option<ProcessId>) -> Result<ProcessId> {
    rv!(getpgid(pid.unwrap_or(0)), -> ProcessId)
}

/// Resource usage.
#[derive(Pod, Eq)]
pub struct ResourceUsage {
    data: rusage,
}

impl ResourceUsage {
    /// Returns the CPU time used in user space.
    pub fn user_time(&self) -> Time {
        Time {
            seconds: self.data.ru_utime.tv_sec as i64,
            nanoseconds: self.data.ru_utime.tv_usec as i64 * 1000,
        }
    }

    /// Returns the CPU time used in kernel space.
    pub fn kernel_time(&self) -> Time {
        Time {
            seconds: self.data.ru_stime.tv_sec as i64,
            nanoseconds: self.data.ru_stime.tv_usec as i64 * 1000,
        }
    }

    /// Returns the maximum amount of used memory in RAM.
    ///
    /// = Remarks
    ///
    /// More precisely, this function returns the maximum resident set size used in bytes.
    pub fn max_mem(&self) -> u64 {
        self.data.ru_maxrss as u64 * 1024
    }

    /// Returns the number of page faults that did not require I/O activity.
    pub fn no_io_page_faults(&self) -> u64 {
        self.data.ru_minflt as u64
    }

    /// Returns the number of page faults that required I/O activity.
    pub fn io_page_faults(&self) -> u64 {
        self.data.ru_majflt as u64
    }

    /// Returns the number of filesystem input events.
    pub fn fs_input(&self) -> u64 {
        self.data.ru_inblock as u64
    }

    /// Returns the number of filesystem output events.
    pub fn fs_output(&self) -> u64 {
        self.data.ru_oublock as u64
    }

    /// Returns the number of voluntary context switches.
    pub fn voluntary_context_switches(&self) -> u64 {
        self.data.ru_nvcsw as u64
    }

    /// Returns the number of involuntary context switches.
    pub fn involuntary_context_switches(&self) -> u64 {
        self.data.ru_nivcsw as u64
    }
}

impl Debug for ResourceUsage {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "ResourceUsage {{ user_time: {:?}, kernel_time: {:?}, max_mem: {}, \
                   no_io_page_faults: {}, io_page_faults: {}, fs_input: {}, \
                   fs_output: {}, voluntary_context_switches: {}, \
                   involuntary_context_switches: {} }}",
                   self.user_time(), self.kernel_time(), self.max_mem(),
                   self.no_io_page_faults(), self.io_page_faults(), self.fs_input(),
                   self.fs_output(), self.voluntary_context_switches(),
                   self.involuntary_context_switches())
    }
}

/// Returns the resource usage of this thread, this process, or its children.
///
/// [argument, who]
/// Whose resource usage to return.
///
/// = See also
///
/// * link:man:getrusage(2)
pub fn resource_usage(who: ResourceUser) -> Result<ResourceUsage> {
    let mut usage: ResourceUsage = mem::zeroed();
    try!(rv!(getrusage(who.0, &mut usage.data)));
    Ok(usage)
}

/// Returns a resource limit.
///
/// [argument, process]
/// The process whose resource limit to return.
///
/// [argument, resource]
/// The resource whose limit to return.
///
/// [return_value]
/// Returns the soft limit in the first slot and the hard limit in the second slot.
///
/// = See also
///
/// * link:man:prlimit(2)
pub fn resource_limit(process: ProcessId, resource: Resource) -> Result<(u64, u64)> {
    let mut limit = mem::zeroed();
    try!(rv!(prlimit(process, resource.0, None, Some(&mut limit))));
    Ok((limit.rlim_cur, limit.rlim_max))
}

/// Sets a resource limit.
///
/// [argument, process]
/// The process whose resource limit to set.
///
/// [argument, resource]
/// The resource whose limit to set.
///
/// [argument, soft]
/// The new soft limit.
///
/// [argument, hard]
/// The new hard limit.
///
/// [return_value]
/// Returns the old soft limit in the first slot and the old hard limit in the second
/// slot.
///
/// = See also
///
/// * link:man:prlimit(2)
pub fn set_resource_limit(process: ProcessId, resource: Resource, soft: u64,
                          hard: u64) -> Result<(u64, u64)> {
    let mut old = mem::zeroed();
    let new = rlimit64 {
        rlim_cur: soft,
        rlim_max: hard,
    };
    try!(rv!(prlimit(process, resource.0, Some(&new), Some(&mut old))));
    Ok((old.rlim_cur, old.rlim_max))
}
