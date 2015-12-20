// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_mem"]
#![crate_type = "lib"]
#![feature(custom_derive, associated_consts)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_fmt as fmt;
extern crate lrs_fd as fd;
extern crate lrs_syscall as syscall;

use base::prelude::*;
use core::{slice, mem};
use core::ops::{Range};
use base::{error};
use base::undef::{UndefState};
use cty::{MAP_SHARED, MAP_PRIVATE, c_int, PAGE_SIZE, MAP_FIXED, MREMAP_FIXED};
use flags::{
    MemMapFlags, MemProtFlags, MemReMapFlags, MMAP_ANON, MemSyncFlags, MemLockFlags,
};
use syscall::{
    mmap, munmap, mremap, msync, mprotect, madvise, mlock, munlock, mlockall, munlockall,
    mincore,
};
use fd::{FdContainer};
use adv::{MemAdvice};

mod std { pub use fmt::std::*; pub use cty; }

pub mod flags;
pub mod adv;

const PAGE_SIZE_MASK: usize = PAGE_SIZE - 1;

pub struct MemMap {
    ptr: *mut u8,
    len: usize,
}

impl MemMap {
    fn common(mut len: usize, protection: MemProtFlags, shared: bool,
              flags: MemMapFlags, fd: c_int, at: u64) -> Result<MemMap> {
        let flags = flags.0 | match shared {
            true => MAP_SHARED,
            _ => MAP_PRIVATE,
        };
        if flags & MAP_FIXED != 0 {
            abort!();
        }
        len = (len + PAGE_SIZE_MASK) & !PAGE_SIZE_MASK;
        let rv = unsafe { mmap(0, len, protection.0, flags, fd, at) };
        if rv < 0 && rv > -4096 {
            Err(error::Errno(-rv as c_int))
        } else {
            Ok(MemMap { ptr: rv as *mut u8, len: len })
        }
    }

    pub unsafe fn from_raw_parts(ptr: *mut u8, size: usize) -> MemMap {
        MemMap {
            ptr: ptr,
            len: size,
        }
    }

    pub fn into_raw_parts(self) -> (*mut u8, usize) {
        let ptr = self.ptr;
        let size = self.len;
        mem::forget(self);
        (ptr, size)
    }

    /// Creates an memory mapping which is not backed by a file.
    ///
    /// [argument, len]
    /// The size of the mapping.
    ///
    /// [argument, protection]
    /// The protection of the mapped region.
    ///
    /// [argument, shared]
    /// Whether this mapping can be shared with other processes.
    ///
    /// [argument, flags]
    /// Flags to use when creating this mapping.
    ///
    /// = Remarks
    ///
    /// The real size of the mapping can be larger than the `len` argument.
    ///
    /// The `MAP_FIXED` flag must not be used with this interface.
    ///
    /// = See also
    ///
    /// * link:man:mmap(2) and MAP_ANONYMOUS therein
    pub fn anon(len: usize, protection: MemProtFlags, shared: bool,
                flags: MemMapFlags) -> Result<MemMap> {
        MemMap::common(len, protection, shared, flags | MMAP_ANON, -1, 0)
    }

    /// Creates an memory mapping of a file.
    ///
    /// [argument, file]
    /// The file that will be mapped.
    ///
    /// [argument, at]
    /// The position in the file at which the mapping starts.
    ///
    /// [argument, len]
    /// The size of the mapping.
    ///
    /// [argument, protection]
    /// The protection of the mapped region.
    ///
    /// [argument, shared]
    /// Whether this mapping can be shared with other processes.
    ///
    /// [argument, flags]
    /// Flags to use when creating this mapping.
    ///
    /// = Remarks
    ///
    /// The real size of the mapping can be larger than the `len` argument.
    ///
    /// = See also
    ///
    /// * link:man:mmap(2)
    pub fn file<F>(file: &F, at: u64, len: usize, protection: MemProtFlags, shared: bool,
                   flags: MemMapFlags) -> Result<MemMap>
        where F: FdContainer,
    {
        MemMap::common(len, protection, shared, flags, file.borrow(), at)
    }

    /// Resizes an existing mapping.
    ///
    /// [argument, new_size]
    /// The new size of the mapping.
    ///
    /// [argument, flags]
    /// Flags to use when remapping this mapping.
    ///
    /// = Remarks
    ///
    /// The `MREMAP_FIXED` flag must not be used with this interface.
    ///
    /// = See also
    ///
    /// * link:man:mremap(2)
    pub fn resize(&mut self, mut new_size: usize, flags: MemReMapFlags) -> Result {
        if flags.0 & MREMAP_FIXED != 0 {
            abort!();
        }
        new_size = (new_size + PAGE_SIZE_MASK) & !PAGE_SIZE_MASK;
        let rv = unsafe { mremap(self.ptr as usize, self.len, new_size, flags.0, 0) };
        if rv < 0 && rv > -4096 {
            Err(error::Errno(-rv as c_int))
        } else {
            self.ptr = rv as *mut u8;
            self.len = new_size;
            Ok(())
        }
    }

    fn to_range(&self, range: Range<Option<usize>>) -> Range<usize> {
        match range {
            Range { start: None, end: None } => Range { start: 0, end: self.len },
            Range { start: None, end: Some(e) } => Range { start: 0, end: e },
            Range { start: Some(s), end: None } => Range { start: s, end: self.len },
            Range { start: Some(s), end: Some(e) } => Range { start: s, end: e },
        }
    }

    /// Flushes changes to a mapped file to the filesystem.
    ///
    /// [argument, range]
    /// The range to be flushed.
    ///
    /// [argument, flags]
    /// Flags to use for synchronization.
    ///
    /// = See also
    ///
    /// * link:man:msync(2)
    pub fn sync<R>(&self, range: R, flags: MemSyncFlags) -> Result
        where R: Into<Range<Option<usize>>>,
    {
        let range = self.to_range(range.into());
        if range.start > range.end || range.end > self.len {
            return Err(error::InvalidArgument);
        }
        rv!(msync(self.ptr as usize + range.start, range.end - range.start, flags.0))
    }

    /// Advise the kernel of a certain memory usage pattern.
    ///
    /// [argument, range]
    /// The range for which the advice holds. Must be page-aligned.
    ///
    /// [argument, advice]
    /// The advice given.
    ///
    /// = Remarks
    ///
    /// The `DontFork` and `HwPoison` advices cannot be used safely.
    /// Trying to use them with this interface causes a process abort.
    ///
    /// = See also
    ///
    /// * link:man:madvise(2)
    pub fn advise<R>(&mut self, range: R, advice: MemAdvice) -> Result
        where R: Into<Range<Option<usize>>>,
    {
        let range = self.to_range(range.into());
        if range.start > range.end || range.end > self.len {
            return Err(error::InvalidArgument);
        }
        match advice {
            adv::DontFork | adv::HwPoison => abort!(),
            _ => { },
        }
        rv!(msync(self.ptr as usize + range.start, range.end - range.start, advice.0))
    }

    /// Change the memory protection of a region.
    ///
    /// [argument, range]
    /// The range for which the protection holds. Must be page-aligned.
    ///
    /// [argument, protection]
    /// The new protection.
    ///
    /// = See also
    ///
    /// * link:man:mprotect(2)
    pub fn protect<R>(&self, range: R, protection: MemProtFlags) -> Result
        where R: Into<Range<Option<usize>>>,
    {
        let range = self.to_range(range.into());
        if range.start > range.end || range.end > self.len {
            return Err(error::InvalidArgument);
        }
        rv!(mprotect(self.ptr as usize + range.start, range.end - range.start,
                     protection.0))
    }
}

unsafe impl UndefState for MemMap {
    fn num() -> usize { <&u8>::num() }

    unsafe fn set_undef(val: *mut MemMap, n: usize) {
        <&mut u8>::set_undef(&mut &mut *(*val).ptr, n)
    }

    unsafe fn is_undef(val: *const MemMap, n: usize) -> bool {
        <&mut u8>::is_undef(&&mut *(*val).ptr, n)
    }
}

impl Deref for MemMap {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl DerefMut for MemMap {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl Drop for MemMap {
    fn drop(&mut self) {
        unsafe { rv!(munmap(self.ptr as usize, self.len)).unwrap(); }
    }
}

/// Change the memory protection of a region.
///
/// [argument, range]
/// The range for which the protection holds. Must be page-aligned.
///
/// [argument, protection]
/// The new protection.
///
/// = See also
///
/// * link:man:mprotect(2)
pub fn protect(range: Range<usize>, protection: MemProtFlags) -> Result {
    if range.start > range.end {
        return Err(error::InvalidArgument);
    }
    rv!(mprotect(range.start, range.end - range.start, protection.0))
}

/// Advise the kernel of a certain memory usage pattern.
///
/// [argument, range]
/// The range for which the advice holds. Must be page-aligned.
///
/// [argument, advice]
/// The advice given.
///
/// = Remarks
///
/// The `DontNeed`, `Remove`, `DontFork`, and `HwPoison` advices cannot be used safely.
/// Trying to use them with this interface causes a process abort.
///
/// = See also
///
/// * link:man:madvise(2)
pub fn advise(range: Range<usize>, advice: MemAdvice) -> Result {
    if range.start > range.end {
        return Err(error::InvalidArgument);
    }
    match advice {
        adv::DontNeed | adv::Remove | adv::DontFork | adv::HwPoison => abort!(),
        _ => { },
    }
    unsafe { rv!(madvise(range.start, range.end - range.start, advice.0)) }
}

/// Lock a memory range in memory.
///
/// [argument, range]
/// The range to lock.
///
/// = See also
///
/// * link:man:mlock(2)
pub fn lock(range: Range<usize>) -> Result {
    if range.start > range.end {
        return Err(error::InvalidArgument);
    }
    rv!(mlock(range.start, range.end - range.start))
}

/// Unlock a memory range.
///
/// [argument, range]
/// The range to unlock.
///
/// = See also
///
/// * link:man:munlock(2)
pub fn unlock(range: Range<usize>) -> Result {
    if range.start > range.end {
        return Err(error::InvalidArgument);
    }
    rv!(munlock(range.start, range.end - range.start))
}

/// Lock all pages in memory.
///
/// [argument, flags]
/// Flags to used for locking.
///
/// = See also
///
/// * link:man:mlockall(2)
pub fn lock_all(flags: MemLockFlags) -> Result {
    rv!(mlockall(flags.0))
}

/// Unlock all pages in memory.
///
/// = See also
///
/// * link:man:munlockall(2)
pub fn unlock_all() -> Result {
    rv!(munlockall())
}

/// Availability of pages in memory.
#[derive(Pod)]
pub struct Availability(pub u8);

impl Availability {
    /// Returns whether a page is in memory.
    pub fn available(self) -> bool {
        self.0 & 1 != 0
    }
}

/// Checks whether pages are in memory or swapped out.
///
/// [argument, range]
/// The range to check. Must be page aligned.
///
/// [argument, buf]
/// The buffer in which the result will be stored.
///
/// = Remarks
///
/// The buffer must have at least on entry per page.
///
/// = See also
///
/// * link:man:mincore(2)
pub fn availability(range: Range<usize>, buf: &mut [u8]) -> Result<&mut [Availability]> {
    if range.start > range.end {
        return Err(error::InvalidArgument);
    }
    let len = range.end - range.start;
    let pages = (buf.len() + PAGE_SIZE - 1) / PAGE_SIZE;
    try!(rv!(mincore(range.start, len, buf.as_mut())));
    Ok(unsafe { mem::cast(&mut buf[..pages]) })
}
