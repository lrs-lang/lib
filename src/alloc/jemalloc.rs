// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::{error};
use {Allocator, MAX_SIZE};
use cty::{c_int};

#[cfg(target_arch = "arm")]
const MIN_ALIGN: usize = 8;
#[cfg(not(target_arch = "arm"))]
const MIN_ALIGN: usize = 16;

macro_rules! mallocx_align {
    ($val:expr) => { if $val < MIN_ALIGN { 0 } else { $val.trailing_zeros() as c_int } }
}

#[cfg(not(no_link_args))]
#[link(name = "jemalloc")]
extern { }

#[allow(improper_ctypes)]
extern {
    fn je_mallocx(size: usize, flags: c_int) -> *mut u8;
    fn je_rallocx(ptr: *mut u8, size: usize, flags: c_int) -> *mut u8;
    fn je_sdallocx(ptr: *mut u8, size: usize, flags: c_int) -> *mut u8;
}

/// The jemalloc allocator
///
/// = Remarks
///
/// To use this you have to compile lrs with the `jemalloc` option, compile a recent
/// version of jemalloc with the `je_` prefix and add `-L path_to_jemalloc` to your
/// compiler invocation.
pub struct JeMalloc;

impl Allocator for JeMalloc {
    unsafe fn allocate_raw(size: usize, alignment: usize) -> Result<*mut u8> {
        if size > MAX_SIZE {
            Err(error::InvalidArgument)
        } else {
            let flags = mallocx_align!(alignment);
            let ptr = je_mallocx(size, flags);
            if ptr.is_null() {
                Err(error::NoMemory)
            } else {
                Ok(ptr)
            }
        }
    }

    unsafe fn free_raw(ptr: *mut u8, size: usize, alignment: usize) {
        let flags = mallocx_align!(alignment);
        je_sdallocx(ptr, size, flags);
    }

    unsafe fn reallocate_raw(old_ptr: *mut u8, oldsize: usize, newsize: usize,
                             alignment: usize) -> Result<*mut u8> {
        let _ = oldsize;
        if newsize > MAX_SIZE {
            Err(error::InvalidArgument)
        } else {
            let flags = mallocx_align!(alignment);
            let ptr = je_rallocx(old_ptr, newsize, flags);
            if ptr.is_null() {
                Err(error::NoMemory)
            } else {
                Ok(ptr)
            }
        }
    }
}
