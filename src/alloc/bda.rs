// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::error::{self, Errno};
use syscall::{mmap, munmap, mremap};
use cty::{
    MAP_ANONYMOUS, PROT_READ, PROT_WRITE, MAP_PRIVATE, c_int, MREMAP_MAYMOVE,
};
use {Allocator, MAX_SIZE};

/// The brain-dead allocator
///
/// = Remarks
///
/// :mmap: link:man:mmap(2)
///
/// This allocator has no state and always uses {mmap} to allocate in multiples of the
/// page size.
///
/// = See also
///
/// * {mmap}
pub struct Bda;

impl Allocator for Bda {
    unsafe fn allocate_raw(size: usize, alignment: usize) -> Result<*mut u8> {
        let _ = alignment;
        if size > MAX_SIZE {
            return Err(error::InvalidArgument);
        }
        let ptr = mmap(0, size, PROT_READ | PROT_WRITE,
                       MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
        if ptr < 0 {
            Err(Errno(-ptr as c_int))
        } else {
            Ok(ptr as usize as *mut u8)
        }
    }

    unsafe fn free_raw(ptr: *mut u8, size: usize, alignment: usize) {
        let _ = alignment;
        munmap(ptr as usize, size);
    }

    unsafe fn reallocate_raw(old_ptr: *mut u8, oldsize: usize, newsize: usize,
                             alignment: usize) -> Result<*mut u8> {
        let _ = alignment;
        if newsize > MAX_SIZE {
            return Err(error::InvalidArgument);
        }
        let new_ptr = mremap(old_ptr as usize, oldsize, newsize, MREMAP_MAYMOVE, 0);
        if new_ptr < 0 {
            Err(Errno(-new_ptr as c_int))
        } else {
            Ok(new_ptr as usize as *mut u8)
        }
    }
}
