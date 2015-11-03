// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use core::marker::{Leak};
use core::cmp::{max};
use {Allocator};

/// An allocator wrapper with minimal alignment.
///
/// = Remarks
///
/// This allocator allocates memory that's aligned for `T` objects. That is, it replaces
/// the alignment arguments by the maximum of the argument and `T`s alignment.
pub struct AlignAlloc<T, H>
    where T: 'static,
          H: Allocator
{
    _data: PhantomData<(T, H)>,
}

impl<T, H> Allocator for AlignAlloc<T, H>
    where T: 'static + Leak,
          H: Allocator
{
    type Pool = H::Pool;

    unsafe fn allocate_raw(pool: &mut H::Pool, size: usize,
                           alignment: usize) -> Result<*mut u8> {
        H::allocate_raw(pool, size, max(mem::align_of::<T>(), alignment))
    }

    unsafe fn free_raw(pool: &mut H::Pool, ptr: *mut u8, size: usize, alignment: usize) {
        H::free_raw(pool, ptr, size, max(mem::align_of::<T>(), alignment))
    }

    unsafe fn reallocate_raw(pool: &mut H::Pool, old_ptr: *mut u8, oldsize: usize,
                             newsize: usize, alignment: usize) -> Result<*mut u8> {
        H::reallocate_raw(pool, old_ptr, oldsize, newsize, max(mem::align_of::<T>(),
                          alignment))
    }
}
