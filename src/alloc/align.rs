// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use core::marker::{Leak};
use core::cmp::{max};
use {MemPool};

/// An allocator wrapper with minimal alignment.
///
/// = Remarks
///
/// This allocator allocates memory that's aligned for `T` objects. That is, it replaces
/// the alignment arguments by the maximum of the argument and `T`s alignment.
#[derive(Copy)]
pub struct AlignAlloc<T, H: ?Sized>
    where T: 'static,
          H: MemPool
{
    _data: PhantomData<*const T>,
    alloc: H,
}

impl<T1, H1: ?Sized, H2, T2 = T1> To<AlignAlloc<T2, H2>> for AlignAlloc<T1, H1>
    where T1: 'static,
          T2: 'static,
          H1: MemPool+To<H2>,
          H2: MemPool,
{
    fn to(&self) -> AlignAlloc<T2, H2> {
        AlignAlloc {
            _data: PhantomData,
            alloc: self.alloc.to(),
        }
    }
}

impl<T1, H1: ?Sized, H2, T2 = T1> TryTo<AlignAlloc<T2, H2>> for AlignAlloc<T1, H1>
    where T1: 'static,
          T2: 'static,
          H1: MemPool+TryTo<H2>,
          H2: MemPool,
{
    fn try_to(&self) -> Result<AlignAlloc<T2, H2>> {
        Ok(AlignAlloc {
            _data: PhantomData,
            alloc: try!(self.alloc.try_to()),
        })
    }
}

impl<T, H> AlignAlloc<T, H>
    where T: 'static,
          H: MemPool
{
    pub fn new(pool: H) -> AlignAlloc<T, H> {
        AlignAlloc {
            _data: PhantomData,
            alloc: pool,
        }
    }
}

impl<T, H> Default for AlignAlloc<T, H>
    where T: 'static,
          H: MemPool+Default,
{
    fn default() -> AlignAlloc<T, H> {
        AlignAlloc {
            _data: PhantomData,
            alloc: H::default(),
        }
    }
}

unsafe impl<T, H> Leak for AlignAlloc<T, H>
    where T: 'static,
          H: MemPool
{
}

impl<T, H> MemPool for AlignAlloc<T, H>
    where T: 'static,
          H: MemPool,
{
    unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Result<*mut u8> {
        self.alloc.alloc(size, max(mem::align_of::<T>(), alignment))
    }

    unsafe fn free(&mut self, ptr: *mut u8, size: usize, alignment: usize) {
        self.alloc.free(ptr, size, max(mem::align_of::<T>(), alignment))
    }

    unsafe fn realloc(&mut self, old_ptr: *mut u8, oldsize: usize, newsize: usize,
                      alignment: usize) -> Result<*mut u8> {
        self.alloc.realloc(old_ptr, oldsize, newsize, max(mem::align_of::<T>(),
                           alignment))
    }
}
