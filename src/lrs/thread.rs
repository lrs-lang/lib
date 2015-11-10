// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Multi-threading
//!
//! = Examples
//!
//! ----
//! let mut array = [1u8; 1024];
//! {
//!     let res = scoped(|| {
//!         println!("getting to work");
//!         for i in 0..SIZE {
//!             array[i] = 2;
//!         }
//!         println!("done working");
//!     });
//!     println!("joining");
//!     res.unwrap();
//!     println!("joined");
//! }
//! for i in 0..SIZE {
//!     assert!(array[i] == 2);
//! }
//! ----

pub use lrs_thread::{
    Builder, spawn, scoped, JoinGuard, cpu_count, CpuMask, cpus, set_cpus, unshare,
    current_cpu, join_namespace, thread_id, exit, deschedule, enter_strict_mode,
};
pub use lrs_thread::ids::{
    UserIds, GroupIds, drop_user_privileges, drop_group_privileges, set_effective_user_id,
    set_effective_group_id, num_supplementary_groups, supplementary_groups,
    set_supplementary_groups
};
pub use lrs_thread::sched::{
    Scheduler, SchedFlags, SchedAttr, set_scheduler, scheduler, process_niceness,
    group_niceness, user_niceness, set_process_niceness, set_group_niceness,
    set_user_niceness,
};
pub use lrs_thread::cap::{
    Capability, CapSet, capabilities, set_capabilities, has_bounding_cap,
    drop_bounding_cap, keeps_caps, set_keeps_caps,
};

pub mod flags {
    pub use lrs_thread::sched::{
        SCHED_NONE, SCHED_RESET_ON_FORK,
    };
}

pub mod sched {
    pub use lrs_thread::sched::{
        Normal, Fifo, Rr, Batch, Idle, Deadline,
    };
}

pub mod capability {
    pub use lrs_thread::cap::{
        ChangeOwner, ReadWriteExecute, ReadSearch, FileOwner, FileSetId, SendSignals,
        SetGroupIds, SetUserIds, SetCapabilities, ImmutableFile, PrivilegedPorts, Network,
        RawSockets, LockMemory, IpcOwner, KernelModules, RawIo, ChangeRoot, Trace,
        Accounting, Admin, Reboot, Scheduling, Resources, Clock, Tty, DeviceFiles, Lease,
        AuditWrite, AuditControl, FileCapabilities, MacOverride, MacAdmin, Syslog, Wakeup,
        BlockSuspend, AuditRead,
    };
}
