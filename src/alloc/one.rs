// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use base::{error};
use {MemPool};

#[derive(Copy)]
pub struct OncePool<'a> {
    ptr: *mut d8,
    len: usize,
    _data: PhantomData<&'a ()>,
}

impl<'a> OutOf for OncePool<'a> {
    fn out_of(_: ()) -> Self {
        OncePool {
            ptr: 0 as *mut _,
            len: 0,
            _data: PhantomData,
        }
    }
}

impl<'a> OncePool<'a> {
    pub fn new(pool: &'a mut [d8]) -> OncePool<'a> {
        OncePool {
            ptr: pool.as_mut_ptr(),
            len: pool.len(),
            _data: PhantomData,
        }
    }
}

impl<'a> MemPool for OncePool<'a> {
    unsafe fn alloc(&mut self, size: usize, align: usize) -> Result<*mut d8> {
        if self.ptr.is_null() {
            return Err(error::NoMemory);
        }

        if align <= self.len {
            let skip = (-(self.ptr as isize)) as usize & (align - 1);
            let len = self.len - skip;
            if size <= len {
                let ptr = self.ptr.add(skip);
                self.ptr = 0 as *mut _;
                self.len = len;
                return Ok(ptr);
            }
        }

        Err(error::NoMemory)
    }

    unsafe fn free(&mut self, _: *mut d8, _: usize, _: usize) { }

    unsafe fn realloc(&mut self, ptr: *mut d8, _: usize, newsize: usize,
                      _: usize) -> Result<*mut d8> {
        if newsize <= self.len {
            Ok(ptr)
        } else {
            Err(error::NoMemory)
        }
    }

    unsafe fn usable_size(&self, _: *mut d8, _: usize, _: usize) -> usize {
        self.len
    }
}
