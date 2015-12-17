// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use base::{error};
use {MemPool};

/// Heap without memory backing it
///
/// = Remarks
///
/// This allocator does not inspect the arguments passed to it and always returns that no
/// memory is available.
pub struct Dummy<'a>(PhantomData<&'a ()>);

impl<'a> OutOf for Dummy<'a> {
    fn out_of(_: ()) -> Dummy<'a> {
        Dummy(PhantomData)
    }
}

impl<'a> MemPool for Dummy<'a> {
    unsafe fn alloc(&mut self, _: usize, _: usize) -> Result<*mut u8> {
        Err(error::NoMemory)
    }
    unsafe fn free(&mut self, _: *mut u8, _: usize, _: usize) { }
    unsafe fn realloc(&mut self, _: *mut u8, _: usize, _: usize,
                      _: usize) -> Result<*mut u8> {
        Err(error::NoMemory)
    }
}
