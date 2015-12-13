// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use base::{error};
use {MemPool, MAX_SIZE};
use lrs_libc as libc;

/// The libc allocator
///
/// = Remarks
///
/// This allocator ignores the alignment argument and always returns maximally aligned
/// pointers.
#[derive(Copy)]
pub struct Libc;

impl OutOf for Libc {
    fn out_of(_: ()) -> Libc {
        Libc
    }
}

impl MemPool for Libc {
    unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Result<*mut u8> {
        self.realloc(0 as *mut u8, 0, size, alignment)
    }

    unsafe fn free(&mut self, ptr: *mut u8, size: usize, alignment: usize) {
        self.realloc(ptr, size, 0, alignment);
    }

    unsafe fn realloc(&mut self, old_ptr: *mut u8, oldsize: usize, newsize: usize,
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
