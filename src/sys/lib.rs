// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_sys"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux_base as base;
#[prelude_import] use base::prelude::*;
mod linux { pub use base::linux::*; }
mod core { pub use base::core::*; }

extern crate linux_time_base as time_base;

use base::{mem, fmt};
use base::ops::{Eq};
use base::io::{Write};
use base::cty::{new_utsname, sysinfo, GRND_NONBLOCK, PATH_MAX};
use base::syscall::{sched_getaffinity, uname, sysinfo, getrandom, acct, sethostname,
                    setdomainname};
use base::string::{ToCString, AsByteStr, AsCStr, ByteStr};
use base::rmo::{AsRef};
use base::result::{Result};
use base::util::{retry};

use time_base::{Time};

/// Returns the number of CPU available to this thread.
pub fn cpu_count() -> usize {
    // XXX: Up to 512 CPUs which is the default maximum for ia64
    let mut buf = [0; 512 / 8];
    sched_getaffinity(0, &mut buf);
    buf.iter().map(|b| b.count_ones()).sum(0) as usize
}

/// Returns information about the system in form of strings.
#[derive(Copy)]
pub struct StrInfo {
    buf: new_utsname,
    sysname_len:    u8,
    nodename_len:   u8,
    release_len:    u8,
    version_len:    u8,
    machine_len:    u8,
    domainname_len: u8,
}

impl StrInfo {
    /// Creates a new StrInfo.
    ///
    /// This instance has not yet any information in it. You have to call `update` to fill
    /// it.
    #[inline(always)]
    pub fn new() -> StrInfo {
        unsafe { mem::zeroed() }
    }

    /// Retrieves information from the system and stores it in the Strinfo.
    pub fn update(&mut self) -> Result {
        try!(rv!(uname(&mut self.buf)));
        self.sysname_len    = self.buf.sysname.as_cstr().unwrap().len()    as u8;
        self.nodename_len   = self.buf.nodename.as_cstr().unwrap().len()   as u8;
        self.release_len    = self.buf.release.as_cstr().unwrap().len()    as u8;
        self.version_len    = self.buf.version.as_cstr().unwrap().len()    as u8;
        self.machine_len    = self.buf.machine.as_cstr().unwrap().len()    as u8;
        self.domainname_len = self.buf.domainname.as_cstr().unwrap().len() as u8;
        Ok(())
    }

    /// The name of the system.
    pub fn system_name(&self) -> &ByteStr {
        self.buf.sysname[..self.sysname_len as usize].as_byte_str()
    }

    /// The hostname.
    pub fn host_name(&self) -> &ByteStr {
        self.buf.nodename[..self.nodename_len as usize].as_byte_str()
    }

    /// The kernel release.
    pub fn release(&self) -> &ByteStr {
        self.buf.release[..self.release_len as usize].as_byte_str()
    }

    /// The kernel version.
    pub fn version(&self) -> &ByteStr {
        self.buf.version[..self.version_len as usize].as_byte_str()
    }

    /// The machine.
    pub fn machine(&self) -> &ByteStr {
        self.buf.machine[..self.machine_len as usize].as_byte_str()
    }

    /// The domain name.
    pub fn domain_name(&self) -> &ByteStr {
        self.buf.domainname[..self.domainname_len as usize].as_byte_str()
    }
}

impl Eq for StrInfo {
    fn eq(&self, other: &StrInfo) -> bool {
           self.system_name() == other.system_name()
        && self.host_name()   == other.host_name()
        && self.release()     == other.release()
        && self.version()     == other.version()
        && self.machine()     == other.machine()
        && self.domain_name() == other.domain_name()
    }

    fn ne(&self, other: &StrInfo) -> bool {
           self.system_name() != other.system_name()
        || self.host_name()   != other.host_name()
        || self.release()     != other.release()
        || self.version()     != other.version()
        || self.machine()     != other.machine()
        || self.domain_name() != other.domain_name()
    }
}

impl fmt::Debug for StrInfo {
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "StrInfo {{ system_name: {:?}, host_name: {:?}, release: {:?}, \
                                version: {:?}, machine: {:?}, domain_name: {:?} }}",
               self.system_name(), self.host_name(), self.release(), self.version(),
               self.machine(), self.domain_name())
    }
}

/// Returns information about the system in form of numbers.
///
/// Someone should find out what the undocumented fields mean.
#[derive(Copy, Eq)]
pub struct NumInfo {
    data: sysinfo,
}

impl NumInfo {
    /// Creates a new NumInfo.
    ///
    /// This instance has not yet any information in it. You have to call `update` to fill
    /// it.
    #[inline(always)]
    pub fn new() -> NumInfo {
        unsafe { mem::zeroed() }
    }

    /// Retrieves information from the system and stores it in the NumInfo.
    pub fn update(&mut self) -> Result {
        rv!(sysinfo(&mut self.data))
    }

    /// Returns the time since the system was last booted.
    pub fn uptime(&self) -> Time {
        Time { seconds: self.data.uptime as i64, nanoseconds: 0 }
    }

    /// Returns the load average of the last minute.
    pub fn load_average_one(&self) -> u64 {
        self.data.loads[0] as u64
    }

    /// Returns the load average of the last five minutes.
    pub fn load_average_five(&self) -> u64 {
        self.data.loads[1] as u64
    }

    /// Returns the load average of the last fifteen minutes.
    pub fn load_average_fifteen(&self) -> u64 {
        self.data.loads[2] as u64
    }

    /// Returns the total amount of memory.
    pub fn total_memory(&self) -> u64 {
        self.data.totalram as u64
    }

    /// Returns the amount of free memory.
    pub fn free_memory(&self) -> u64 {
        self.data.freeram as u64
    }

    pub fn shared_memory(&self) -> u64 {
        self.data.sharedram as u64
    }

    pub fn buffer_memory(&self) -> u64 {
        self.data.bufferram as u64
    }

    pub fn swap_memory(&self) -> u64 {
        self.data.totalswap as u64
    }

    pub fn free_swap_memory(&self) -> u64 {
        self.data.freeswap as u64
    }

    pub fn processes(&self) -> u64 {
        self.data.procs as u64
    }

    pub fn total_high(&self) -> u64 {
        self.data.totalhigh as u64
    }

    pub fn free_high(&self) -> u64 {
        self.data.totalhigh as u64
    }

    pub fn mem_unit(&self) -> u64 {
        self.data.mem_unit as u64
    }
}

impl fmt::Debug for NumInfo {
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "NumInfo {{ \
                        uptime: {:?}, \
                        load_average_one: {}, \
                        load_average_five: {}, \
                        load_average_fifteen: {}, \
                        total_memory: {}, \
                        free_memory: {}, \
                        shared_memory: {}, \
                        buffer_memory: {}, \
                        swap_memory: {}, \
                        free_swap_memory: {}, \
                        processes: {}, \
                        total_high: {}, \
                        free_high: {}, \
                        mem_unit: {} }}",
                        self.uptime(), self.load_average_one(), self.load_average_five(),
                        self.load_average_fifteen(), self.total_memory(),
                        self.free_memory(), self.shared_memory(), self.buffer_memory(),
                        self.swap_memory(), self.free_swap_memory(), self.processes(),
                        self.total_high(), self.free_high(), self.mem_unit())
    }
}

/// Stores random bytes in a prefix of the buffer.
pub fn get_random(buf: &mut [u8]) -> Result<&mut [u8]> {
    let num = try!(retry(|| getrandom(buf, 0)));
    Ok(&mut buf[..num as usize])
}

/// Stores random bytes in a prefix of the buffer without blocking.
pub fn get_random_non_blocking(buf: &mut [u8]) -> Result<&mut [u8]> {
    let num = try!(retry(|| getrandom(buf, GRND_NONBLOCK)));
    Ok(&mut buf[..num as usize])
}

/// Enables process accounting.
pub fn enable_accounting<P>(path: P) -> Result
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf));
    rv!(acct(&path))
}

/// Sets the hostname of the system
pub fn set_host_name<P>(name: P) -> Result
    where P: AsRef<[u8]>,
{
    rv!(sethostname(name.as_ref()))
}

/// Sets the domain name of the system
pub fn set_domain_name<P>(name: P) -> Result
    where P: AsRef<[u8]>,
{
    rv!(setdomainname(name.as_ref()))
}
