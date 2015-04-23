// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::{error};
use {Allocator, MAX_SIZE};
use linux_libc as libc;

/// The libc heap
///
/// Note that this allocator ignores the alignment argument and always returns maximally
/// aligned pointers.
pub struct LibcHeap;

impl Allocator for LibcHeap {
    unsafe fn allocate_raw(size: usize, alignment: usize) -> Result<*mut u8> {
        LibcHeap::reallocate_raw(0 as *mut u8, 0, size, alignment)
    }

    unsafe fn free_raw(ptr: *mut u8, size: usize, alignment: usize) {
        LibcHeap::reallocate_raw(ptr, size, 0, alignment);
    }

    unsafe fn reallocate_raw(old_ptr: *mut u8, oldsize: usize, newsize: usize,
                             alignment: usize) -> Result<*mut u8> {
        let _ = oldsize;
        let _ = alignment;
        if newsize > MAX_SIZE {
            Err(error::InvalidArgument)
        } else {
            let ptr = libc::realloc(old_ptr, newsize);
            if ptr.is_null() {
                Err(error::NoMemory)
            } else {
                Ok(ptr)
            }
        }
    }
}
