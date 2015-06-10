// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals, non_camel_case_types)]

#[prelude_import] use base::prelude::*;
use fmt::{Debug, Write};
use core::{mem};
use cty::alias::{ProcessId};
use cty::{
    self, c_int, __user_cap_data_struct, c_uint, CAP_LAST_CAP,
};
use syscall::{capget_v3, capset_v3};

/// A Linux capability.
///
/// [field, 1]
/// The numeric representation of the capability.
#[derive(Pod, Eq)]
pub struct Capability(pub c_int);

macro_rules! create {
    ($($(#[$meta:meta])* cap $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: Capability = Capability(cty::$val as c_int);)*

        impl Debug for Capability {
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
    cap ChangeOwner = CAP_CHOWN;
    cap ReadWriteExecute = CAP_DAC_OVERRIDE;
    cap ReadSearch = CAP_DAC_READ_SEARCH;
    cap FileOwner = CAP_FOWNER;
    cap FileSetId = CAP_FSETID;
    cap SendSignals = CAP_KILL;
    cap SetGroupIds = CAP_SETGID;
    cap SetUserIds = CAP_SETUID;
    cap SetCapabilities = CAP_SETPCAP;
    cap ImmutableFile = CAP_LINUX_IMMUTABLE;
    cap PrivilegedPorts = CAP_NET_BIND_SERVICE;
    cap Broadcast = CAP_NET_BROADCAST;
    cap Network = CAP_NET_ADMIN;
    cap RawSockets = CAP_NET_RAW;
    cap LockMemory = CAP_IPC_LOCK;
    cap IpcOwner = CAP_IPC_OWNER;
    cap KernelModules = CAP_SYS_MODULE;
    cap RawIo = CAP_SYS_RAWIO;
    cap ChangeRoot = CAP_SYS_CHROOT;
    cap Trace = CAP_SYS_PTRACE;
    cap Accounting = CAP_SYS_PACCT;
    cap Admin = CAP_SYS_ADMIN;
    cap Reboot = CAP_SYS_BOOT;
    cap Scheduling = CAP_SYS_NICE;
    cap Resources = CAP_SYS_RESOURCE;
    cap Clock = CAP_SYS_TIME;
    cap Tty = CAP_SYS_TTY_CONFIG;
    cap DeviceFiles = CAP_MKNOD;
    cap Lease = CAP_LEASE;
    cap AuditWrite = CAP_AUDIT_WRITE;
    cap AuditControl = CAP_AUDIT_CONTROL;
    cap FileCapabilities = CAP_SETFCAP;
    cap MacOverride = CAP_MAC_OVERRIDE;
    cap MacAdmin = CAP_MAC_ADMIN;
    cap Syslog = CAP_SYSLOG;
    cap Wakeup = CAP_WAKE_ALARM;
    cap BlockSuspend = CAP_BLOCK_SUSPEND;
    cap AuditRead = CAP_AUDIT_READ;
}

/// A thread's capability set.
#[derive(Pod, Eq)]
pub struct CapSet {
    effective:   u64,
    permitted:   u64,
    inheritable: u64,
}

impl CapSet {
    /// Returns an empty set.
    pub fn new() -> CapSet {
        mem::zeroed()
    }

    /// Returns whether the set contains an effective capability.
    ///
    /// [argument, cap]
    /// The capability to check.
    pub fn has_effective(&self, cap: Capability) -> bool {
        let cap = cap.0 as c_uint;
        (cap < 64) && (self.effective & (1 << cap) != 0)
    }

    /// Returns whether the set contains a permitted capability.
    ///
    /// [argument, cap]
    /// The capability to check.
    pub fn has_permitted(&self, cap: Capability) -> bool {
        let cap = cap.0 as c_uint;
        (cap < 64) && (self.effective & (1 << cap) != 0)
    }

    /// Returns whether the set contains an inheritable capability.
    ///
    /// [argument, cap]
    /// The capability to check.
    pub fn has_inheritable(&self, cap: Capability) -> bool {
        let cap = cap.0 as c_uint;
        (cap < 64) && (self.effective & (1 << cap) != 0)
    }

    /// Sets whether the set contains an effective capability.
    ///
    /// [argument, cap]
    /// The capability to modify.
    ///
    /// [argument, has]
    /// Whether the set contains the capability.
    pub fn set_effective(&mut self, cap: Capability, has: bool) {
        CapSet::set_common(cap, &mut self.effective, has)
    }

    /// Sets whether the set contains a permitted capability.
    ///
    /// [argument, cap]
    /// The capability to modify.
    ///
    /// [argument, has]
    /// Whether the set contains the capability.
    pub fn set_permitted(&mut self, cap: Capability, has: bool) {
        CapSet::set_common(cap, &mut self.permitted, has)
    }

    /// Sets whether the set contains an inheritable capability.
    ///
    /// [argument, cap]
    /// The capability to modify.
    ///
    /// [argument, has]
    /// Whether the set contains the capability.
    pub fn set_inheritable(&mut self, cap: Capability, has: bool) {
        CapSet::set_common(cap, &mut self.inheritable, has)
    }

    fn set_common(cap: Capability, dst: &mut u64, has: bool) {
        let cap = cap.0 as c_uint;
        if cap >= 64 {
            return;
        }
        if has {
            *dst |= 1 << cap
        } else {
            *dst &= !(1 << cap)
        }
    }
}

impl Debug for CapSet {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        try!(w.write_all(b"CapSet { "));
        for &(name, c) in &[("effective: ", self.effective),
                            (", permitted: ", self.permitted),
                            (", inheritable: ", self.inheritable)][..] {
            try!(w.write_all(name.as_bytes()));
            let mut first = true;
            for i in 0..CAP_LAST_CAP+1 {
                if c & (1 << i) != 0 {
                    if !first { try!(w.write_all(b"+")); }
                    first = false;
                    try!(write!(w, "{:?}", Capability(i)));
                }
            }
            if first { try!(w.write_all(b"0")); }
        }
        try!(w.write_all(b" }"));
        Ok(())
    }
}

/// Returns the capabilities of a thread.
///
/// [argument, thread]
/// The thread id of the thread or `0` for the current thread.
///
/// = See also
///
/// * link:man:capget(2)
pub fn capabilities(thread: ProcessId) -> Result<CapSet> {
    let mut buf = [mem::zeroed(); 2];
    try!(rv!(capget_v3(thread, &mut buf)));
    Ok(CapSet {
        effective: buf[0].effective as u64 | ((buf[1].effective as u64) << 32),
        permitted: buf[0].permitted as u64 | ((buf[1].permitted as u64) << 32),
        inheritable: buf[0].inheritable as u64 | ((buf[1].inheritable as u64) << 32),
    })
}

/// Sets the capabilities of the current thread.
///
/// [argument, caps]
/// The new capabilities.
///
/// = See also
///
/// * link:man:capset(2)
pub fn set_capabilities(caps: CapSet) -> Result {
    let caps = [
        __user_cap_data_struct {
            effective: caps.effective as u32,
            permitted: caps.permitted as u32,
            inheritable: caps.inheritable as u32,
        },
        __user_cap_data_struct {
            effective: (caps.effective >> 32) as u32,
            permitted: (caps.permitted >> 32) as u32,
            inheritable: (caps.inheritable >> 32) as u32,
        },
    ];
    rv!(capset_v3(&caps))
}
