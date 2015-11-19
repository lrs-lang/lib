// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_thread"]
#![crate_type = "lib"]
#![feature(no_std, optin_builtin_traits, custom_derive, negate_unsigned)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_libc as libc;
extern crate lrs_syscall as syscall;
extern crate lrs_cty as cty;
extern crate lrs_lock as lock;
extern crate lrs_time_base as time_base;
extern crate lrs_fmt as fmt;
extern crate lrs_iter as iter;
extern crate lrs_clone as clone;
extern crate lrs_fd as fd;
extern crate lrs_mem as mmem;
extern crate lrs_rt as rt;
extern crate lrs_atomic as atomic;

use base::prelude::*;
use core::ops::{Index};
use core::{mem};
use cty::{c_int};
use cty::alias::{ProcessId};
use iter::{IteratorExt};
use fmt::{Debug, Write};
use fd::{FDContainer};
use clone::flags::{CloneFlags};

mod std { pub use fmt::std::*; pub use cty; pub use fd; }

#[cfg(not(no_libc))] #[path = "libc/mod.rs"] pub mod imp;
#[cfg(no_libc)] #[path = "no_libc/mod.rs"] mod imp;

pub use imp::*;

pub mod ids;
pub mod sched;
pub mod cap;

/// Returns the number of CPUs available to this thread.
///
/// = Remarks
///
/// Use link:lrs::thread::cpus[cpus] to get the number of CPUs available to other threads.
///
/// = See also
///
/// * link:man:sched_getaffinity(2)
pub fn cpu_count() -> Result<usize> {
    // XXX: Up to 512 CPUs which is the default maximum for ia64
    let mut buf = [0; 512 / 8];
    cpus(0, &mut buf).map(|c| c.count())
}

/// A bit-mask of CPUs.
///
/// = Remarks
///
/// The CPUs are stored in form of a `[u8]` such that available CPUs are stored as a `1`
/// bit and unavailable CPUs are stored as a `0` bit. The structure supports indexing to
/// check whether a CPU is available: `let have_one = mask[0]`. The value used to index
/// must be at most `mask.len()`.
pub struct CpuMask {
    buf: [u8],
}

impl CpuMask {
    /// Creates a new mask from a buffer.
    ///
    /// [argument, buf]
    /// The buffer which contains the bitset of CPUs.
    pub fn new(buf: &[u8]) -> &CpuMask {
        unsafe { mem::cast(buf) }
    }

    /// Creates a new mask from a buffer.
    ///
    /// [argument, buf]
    /// The buffer which contains the bitset of CPUs.
    pub fn new_mut(buf: &mut [u8]) -> &mut CpuMask {
        unsafe { mem::cast(buf) }
    }

    /// Returns the number of slots in this CPU mask.
    pub fn len(&self) -> usize {
        self.buf.len() * 8
    }

    /// Returns the number of online CPUs in this mask.
    pub fn count(&self) -> usize {
        self.buf.iter().map(|b| b.count_ones()).sum(0) as usize
    }

    /// Marks a CPU as online.
    ///
    /// [argument, id]
    /// The id of the CPU to mark.
    pub fn set(&mut self, id: usize) {
        assert!(id < self.len());
        self.buf[id / 8] |= 1 << id % 8;
    }

    /// Marks a CPU as offline.
    ///
    /// [argument, id]
    /// The id of the CPU to mark.
    pub fn unset(&mut self, id: usize) {
        assert!(id < self.len());
        self.buf[id / 8] &= !(1 << id % 8);
    }
}

impl Deref for CpuMask {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.buf
    }
}

impl DerefMut for CpuMask {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.buf
    }
}

impl Index<usize> for CpuMask {
    type Output = bool;
    fn index(&self, val: usize) -> &bool {
        assert!(val < self.len());
        static TRUE: bool = true;
        static FALSE: bool = false;
        if self.buf[val / 8] & (1 << val % 8) != 0 {
            &TRUE
        } else {
            &FALSE
        }
    }
}

impl Debug for CpuMask {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let mut first = true;
        for i in 0..self.len() {
            if self[i] {
                if !first { try!(write!(w, ",")); }
                first = false;
                try!(write!(w, "{}", i));
            }
        }
        Ok(())
    }
}

/// Returns the CPU mask of a thread.
///
/// [argument, thread]
/// The thread to inspect or `0` for this thread.
///
/// [argument, buf]
/// The buffer in which the mask will be stored.
///
/// = See also
///
/// * link:man:sched_getaffinity(2)
pub fn cpus(thread: ProcessId, mut buf: &mut [u8]) -> Result<&mut CpuMask> {
    let len = buf.len();
    let buf = &mut buf[..len/8*8];
    let len = try!(rv!(syscall::sched_getaffinity(thread, buf), -> usize));
    Ok(CpuMask::new_mut(&mut buf[..len]))
}

/// Sets the CPU mask of a thread.
///
/// [argument, thread]
/// The thread whose mask to set or `0` for this thread.
///
/// [argument, cpus]
/// The CPU mask.
///
/// = See also
///
/// * link:man:sched_setaffinity(2)
pub fn set_cpus(thread: ProcessId, cpus: &CpuMask) -> Result {
    rv!(syscall::sched_setaffinity(thread, &cpus.buf))
}

/// Returns the thread id of the calling thread.
///
/// = See also
///
/// * link:man:gettid(2)
pub fn thread_id() -> ProcessId {
    syscall::gettid()
}

/// Terminates the current thread.
///
/// [argument, code]
/// The exit code of the thread.
///
/// = Remarks
///
/// This is unsafe because `!Leak` data will not be destroyed.
///
/// XXX: This is not actually unsafe unless libc does some weird thing that makes the
/// thread being dead observable. Otherwise this is equivalent to the thread not making
/// any progress as the memory stays in place.
pub unsafe fn exit(code: c_int) -> ! {
    syscall::exit(code)
}

/// Relinquish the CPU.
///
/// = See also
///
/// * link:man:sched_yield(2)
pub fn deschedule() {
    // This function should actually be called yield.
    syscall::sched_yield();
}

/// Disassociate parts of the thread's execution context.
///
/// [argument, flags]
/// What to disassociate.
///
/// = See also
///
/// * link:man:unshare(2)
pub fn unshare(flags: CloneFlags) -> Result {
    rv!(syscall::unshare(flags.0))
}

/// Returns the CPU the thread is currently running on.
///
/// = See also
///
/// * link:man:getcpu(2)
pub fn current_cpu() -> Result<u32> {
    let mut cpu = 0;
    try!(rv!(syscall::getcpu(Some(&mut cpu), None)));
    Ok(cpu as u32)
}

/// Associates this thread with a namespace.
///
/// [argument, ns]
/// A file descriptor referring to a namespace.
///
/// [argument, kind]
/// Restricts what kind of namespace can be joined.
///
/// = See also
///
/// * link:man:setns(2)
pub fn join_namespace<F>(ns: &F, kind: CloneFlags) -> Result
    where F: FDContainer,
{
    rv!(syscall::setns(ns.borrow(), kind.0))
}

/// Enables strict seccomp mode for this thread.
///
/// = Remarks
///
/// :read: link:man:read(2)
/// :write: link:man:write(2)
/// :exit_group: link:man:exit_group(2)
///
/// After a successful call, the thread can only make calls to {read}, {write}, and
/// {exit_group}.
///
/// = See also
///
/// * link:man:seccomp(2) and SECCOMP_SET_MODE_STRICT therein
pub fn enter_strict_mode() -> Result {
    rv!(syscall::seccomp_seccomp_set_mode_strict())
}
