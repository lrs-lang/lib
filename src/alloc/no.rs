// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::{error};
use {Allocator};

/// Heap without memory backing it
///
/// = Remarks
///
/// This allocator does not inspect the argumnets passed to it and always returns that no
/// memory is available.
pub struct NoMem<'a>(PhantomData<&'a ()>);

impl<'a> Allocator for NoMem<'a> {
    unsafe fn allocate_raw(_: usize, _: usize) -> Result<*mut u8> {
        Err(error::NoMemory)
    }
    unsafe fn free_raw(_: *mut u8, _: usize, _: usize) { }
    unsafe fn reallocate_raw(_: *mut u8, _: usize, _: usize,
                             _: usize) -> Result<*mut u8> {
        Err(error::NoMemory)
    }
}
