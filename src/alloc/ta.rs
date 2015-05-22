// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{ptr, cmp, mem};
use base::{error};
use {Allocator};

/// Throw-away allocator.
///
/// = Remarks
///
/// This allocator draws its memory from a provided byte slice. It will not reuse any
/// memory and reallocations will always allocate a new object.
pub struct TaAlloc<'a>(PhantomData<&'a ()>);

impl<'a> Allocator for TaAlloc<'a> {
    type Pool = TaPool<'a>;

    unsafe fn allocate_raw(pool: &mut TaPool<'a>, size: usize,
                           alignment: usize) -> Result<*mut u8> {
        let pool = pool.get();
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

    unsafe fn free_raw(_: &mut TaPool<'a>, _: *mut u8, _: usize, _: usize) { }

    unsafe fn reallocate_raw(pool: &mut TaPool<'a>, ptr: *mut u8, oldsize: usize,
                             newsize: usize, alignment: usize) -> Result<*mut u8> {
        let new = try!(TaAlloc::allocate_raw(pool, newsize, alignment));
        let min = cmp::min(oldsize, newsize);
        ptr::memcpy(new, ptr, min);
        Ok(new)
    }
}

// Not super sure about aliasing here. There are several things: We have the original
// `&mut` that is passed to `new`. Then we have all those copies of `TaPool` that have
// their own `&mut` behind a `*mut`. Are there any aliasing guarantees wrt `*mut`
// pointers? Is the mutable borrowing in `new` enough to inform the compiler that all
// `TaPool` copies can modify the `&mut`? Maybe we should write `*mut Cell<&'a mut [u8]>`
// here.


/// The memory pool of TaAllo.
#[derive(Copy)]
pub struct TaPool<'a> {
    pool: *mut &'a mut [u8],
}

impl<'a> TaPool<'a> {
    /// Creates a new pool from a byte slice.
    ///
    /// [argument, pool]
    /// A reference to the slice that will be used for allocation.
    pub fn new(pool: &'a mut &'a mut [u8]) -> TaPool<'a> {
        TaPool { pool: pool }
    }

    unsafe fn get(&mut self) -> &'a mut &'a mut [u8] {
        &mut *self.pool
    }
}
