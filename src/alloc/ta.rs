// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{ptr, cmp, mem, slice};
use base::{error};
use {MemPool};

/// Throw-away allocator.
///
/// = Remarks
///
/// This allocator draws its memory from a provided byte slice. It will not reuse any
/// memory and reallocations will always allocate a new object.
#[derive(Copy)]
pub struct TaPool<'a> {
    pool: *mut *mut [u8],
    _data: PhantomData<&'a ()>,
}

impl<'a> TaPool<'a> {
    /// Creates a new pool from a byte slice.
    ///
    /// [argument, pool]
    /// A reference to the slice that will be used for allocation.
    pub fn new(pool: &'a mut &mut [u8]) -> TaPool<'a> {
        TaPool {
            pool: unsafe { mem::cast(pool) },
            _data: PhantomData,
        }
    }

    // It would be better if the return value here were *mut *mut, but this is a bit more
    // convenient. Just remember that you cannot move a slice into here whose lifetime is
    // shorter than the lifetime of the original slice.
    unsafe fn get(&mut self) -> &mut &mut [u8] {
        mem::cast(self.pool)
    }
}

impl<'a> MemPool for TaPool<'a> {
    unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Result<*mut u8> {
        let pool = self.get();
        let mask = alignment - 1;
        let start = ((!(pool.as_ptr() as usize) & mask) + 1) & mask;
        let real_size = size + start;
        if pool.len() < real_size {
            return Err(error::NoMemory);
        }
        let ptr = pool.as_mut_ptr().add(start);

        // The compiler fails to understand the lifetimes here so we first have to move
        // the pool out before moving it in again.
        *pool = &mut mem::replace(pool, &mut [])[real_size..];

        Ok(ptr)
    }

    unsafe fn free(&mut self, _: *mut u8, _: usize, _: usize) { }

    unsafe fn realloc(&mut self, ptr: *mut u8, oldsize: usize,
                             newsize: usize, alignment: usize) -> Result<*mut u8> {
        {
            let pool = self.get();
            if ptr.add(oldsize) == pool.as_mut_ptr() {
                if oldsize > newsize {
                    let len = oldsize - newsize + pool.len();
                    *pool = slice::from_ptr(ptr.add(newsize), len); 
                } else if newsize - oldsize <= pool.len() {
                    *pool = &mut mem::replace(pool, &mut [])[newsize - oldsize..];
                } else {
                    return Err(error::NoMemory);
                }
                return Ok(ptr);
            }
        }

        let new = try!(self.alloc(newsize, alignment));
        let min = cmp::min(oldsize, newsize);
        ptr::memcpy(new, ptr, min);
        Ok(new)
    }
}
