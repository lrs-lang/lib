// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_sys"]
#![crate_type = "lib"]
#![feature(custom_derive)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_fmt as fmt;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_str_one as str_one;
extern crate lrs_str_two as str_two;
extern crate lrs_rv as rv;
extern crate lrs_time_base as time_base;
extern crate lrs_iter as iter;
extern crate lrs_rmo as rmo;
extern crate lrs_alloc as alloc;

use base::prelude::*;
mod std { pub use fmt::std::*; pub use {cty}; }

use core::{mem};
use core::ops::{Eq};
use fmt::{Write};
use cty::{
    new_utsname, sysinfo, GRND_NONBLOCK, PATH_MAX, LINUX_REBOOT_CMD_CAD_OFF,
    LINUX_REBOOT_CMD_CAD_ON, LINUX_REBOOT_CMD_HALT, LINUX_REBOOT_CMD_KEXEC,
    LINUX_REBOOT_CMD_POWER_OFF, LINUX_REBOOT_CMD_RESTART, LINUX_REBOOT_CMD_RESTART2,
    LINUX_REBOOT_CMD_SW_SUSPEND,
};
use syscall::{
    uname, sysinfo, getrandom, acct, sethostname, setdomainname,
    reboot,
};
use str_one::{ByteStr, CStr};
use str_two::{CString};
use rv::{retry};
use rmo::{Rmo, ToRmo};
use alloc::{FbHeap, FcPool, OncePool};

use time_base::{Time};

/// Returns information about the system in form of strings.
#[derive(Pod)]
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
    /// = Remarks
    ///
    /// This instance has not yet any information in it. You have to call `update` to fill
    /// it.
    #[inline(always)]
    pub fn new() -> StrInfo {
        mem::zeroed()
    }

    /// Retrieves information from the system and stores it in the object.
    pub fn update(&mut self) -> Result {
        try!(rv!(uname(&mut self.buf)));
        self.sysname_len    = try!(TryAsRef::<CStr>::try_as_ref(&self.buf.sysname[..])).len()    as u8;
        self.nodename_len   = try!(TryAsRef::<CStr>::try_as_ref(&self.buf.nodename[..])).len()   as u8;
        self.release_len    = try!(TryAsRef::<CStr>::try_as_ref(&self.buf.release[..])).len()    as u8;
        self.version_len    = try!(TryAsRef::<CStr>::try_as_ref(&self.buf.version[..])).len()    as u8;
        self.machine_len    = try!(TryAsRef::<CStr>::try_as_ref(&self.buf.machine[..])).len()    as u8;
        self.domainname_len = try!(TryAsRef::<CStr>::try_as_ref(&self.buf.domainname[..])).len() as u8;
        Ok(())
    }

    /// Returns the name of the system.
    pub fn system_name(&self) -> &ByteStr {
        let bytes: &[u8] = self.buf.sysname[..self.sysname_len as usize].as_ref();
        bytes.as_ref()
    }

    /// Returns the hostname of the system.
    pub fn host_name(&self) -> &ByteStr {
        let bytes: &[u8] = self.buf.nodename[..self.nodename_len as usize].as_ref();
        bytes.as_ref()
    }

    /// Returns the kernel release of the system.
    pub fn release(&self) -> &ByteStr {
        let bytes: &[u8] = self.buf.release[..self.release_len as usize].as_ref();
        bytes.as_ref()
    }

    /// Returns the kernel version of the system.
    pub fn version(&self) -> &ByteStr {
        let bytes: &[u8] = self.buf.version[..self.version_len as usize].as_ref();
        bytes.as_ref()
    }

    /// Returns the machine.
    pub fn machine(&self) -> &ByteStr {
        let bytes: &[u8] = self.buf.machine[..self.machine_len as usize].as_ref();
        bytes.as_ref()
    }

    /// Returns the domain name of the system.
    pub fn domain_name(&self) -> &ByteStr {
        let bytes: &[u8] = self.buf.domainname[..self.domainname_len as usize].as_ref();
        bytes.as_ref()
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
#[derive(Pod, Eq)]
pub struct NumInfo {
    data: sysinfo,
}

impl NumInfo {
    /// Creates a new NumInfo.
    ///
    /// = Remarks
    ///
    /// This instance has not yet any information in it. You have to call `update` to fill
    /// it.
    #[inline(always)]
    pub fn new() -> NumInfo {
        mem::zeroed()
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

/// Retrieves random bytes from the system.
///
/// [argument, buf]
/// The buffer in which the bytes will be stored.
///
/// [return_value]
/// Returns an initial sequence of the slice that contains random bytes.
pub fn get_random(buf: &mut [d8]) -> Result<&mut [u8]> {
    let num = try!(retry(|| getrandom(buf, 0)));
    unsafe { Ok(buf[..num as usize].as_mut_bytes()) }
}

/// Retrieves random bytes from the system without blocking.
///
/// [argument, buf]
/// The buffer in which the bytes will be stored.
///
/// [return_value]
/// Returns an initial sequence of the slice that contains random bytes.
pub fn get_random_non_blocking(buf: &mut [d8]) -> Result<&mut [u8]> {
    let num = try!(retry(|| getrandom(buf, GRND_NONBLOCK)));
    unsafe { Ok(buf[..num as usize].as_mut_bytes()) }
}

pub type Pool<'a> = FcPool<OncePool<'a>, FbHeap>;

fn rmo_cstr<'a, S>(s: &'a S,
                   buf: &'a mut [d8]) -> Result<Rmo<'a, CStr, CString<Pool<'a>>>>
    where S: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
{
    s.to_rmo_with(FcPool::new(OncePool::new(buf), FbHeap::out_of(())))
}

/// Enables process accounting.
///
/// [argument, path]
/// The file to which the accounting information will be written.
pub fn enable_accounting<P>(path: P) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
    rv!(acct(Some(&path)))
}

/// Sets the hostname of the system
///
/// [argument, name]
/// The new hostname.
pub fn set_host_name<P: ?Sized>(name: &P) -> Result
    where P: AsRef<[d8]>,
{
    rv!(sethostname(name.as_ref()))
}

/// Sets the domain name of the system
///
/// [argument, name]
/// The new domain name.
pub fn set_domain_name<P: ?Sized>(name: &P) -> Result
    where P: AsRef<[d8]>,
{
    rv!(setdomainname(name.as_ref()))
}

/// Enable or disable immediate restarting with `ctrl-alt-delete`.
///
/// = Remarks
///
/// If enabled, pressing `ctrl-alt-delete` immediately restarts the system. This can cause
/// data-loss.
///
/// If disabled, pressing `ctrl-alt-delete` sends SIGINT to init which then decides how to
/// proceed.
///
/// = See also
///
/// * link:man:reboot(2) and LINUX_REBOOT_CMD_CAD_OFF and LINUX_REBOOT_CMD_CAD_ON therein
pub fn enable_ctrl_alt_delete(enabled: bool) -> Result {
    let cmd = match enabled {
        true => LINUX_REBOOT_CMD_CAD_ON,
        _ => LINUX_REBOOT_CMD_CAD_OFF,
    };
    rv!(reboot(cmd, CStr::empty()))
}

/// Halts the system.
///
/// = See also
///
/// * link:man:reboot(2) and LINUX_REBOOT_CMD_HALT therein
pub fn halt() -> Result {
    rv!(reboot(LINUX_REBOOT_CMD_HALT, CStr::empty()))
}

/// Executes a new kernel.
///
/// = See also
///
/// * link:man:reboot(2) and LINUX_REBOOT_CMD_KEXEC therein
pub fn exec_new_kernel() -> Result {
    rv!(reboot(LINUX_REBOOT_CMD_KEXEC, CStr::empty()))
}

/// Shuts the system down and powers it off.
///
/// = See also
///
/// * link:man:reboot(2) and LINUX_REBOOT_CMD_POWER_OFF therein
pub fn power_off() -> Result {
    rv!(reboot(LINUX_REBOOT_CMD_POWER_OFF, CStr::empty()))
}

/// Shuts the system down and powers it off.
///
/// = See also
///
/// * link:man:reboot(2) and LINUX_REBOOT_CMD_RESTART and LINUX_REBOOT_CMD_RESTART2
///   therein
pub fn restart<T>(msg: Option<T>) -> Result
    where T: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [d8; 256] = unsafe { mem::uninit() };
    let (cmd, arg): (_, _) = match msg {
        Some(ref msg) => (LINUX_REBOOT_CMD_RESTART2, try!(rmo_cstr(msg, &mut buf))),
        _ => (LINUX_REBOOT_CMD_RESTART, Rmo::Ref(&CStr::empty())),
    };
    rv!(reboot(cmd, &arg))
}

/// Performs a software suspend (suspend-to-disk.)
///
/// = See also
///
/// * link:man:reboot(2) and LINUX_REBOOT_CMD_SW_SUSPEND therein
pub fn hibernate() -> Result {
    rv!(reboot(LINUX_REBOOT_CMD_SW_SUSPEND, CStr::empty()))
}
