// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_alloc"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_cty as cty;
extern crate linux_syscall as syscall;
extern crate linux_libc;

#[prelude_import] use base::prelude::*;
use core::{num, mem};
use base::{error};

pub use libc::{LibcHeap};
pub use no::{NoHeap};
pub use bda::{Bda};
pub use jemalloc::{JeMalloc};

mod libc;
mod no;
mod bda;
mod jemalloc;

/// The default allocator
pub type Heap = Bda;

/// The maximum size of an allocation
pub const MAX_SIZE: usize = num::isize::MAX as usize;

/// Returns a non-null pointer that points to a vaild address and has pointer alignment.
pub fn empty_ptr<T>() -> *mut T {
    static EMPTY: usize = 0;
    &EMPTY as *const _ as *mut _
}

/// Allocators
pub trait Allocator {
    /// Allocates a chunk of bytes with the specified properties.
    ///
    /// `alignment` must be a power of two.
    unsafe fn allocate_raw(size: usize, alignment: usize) -> Result<*mut u8>;

    /// Reallocates a chunk of bytes.
    unsafe fn reallocate_raw(ptr: *mut u8, oldsize: usize, newsize: usize,
                             alignment: usize) -> Result<*mut u8>;

    /// Frees a chunk of bytes with the specified properties.
    unsafe fn free_raw(ptr: *mut u8, size: usize, alignment: usize);

    /// Allocates an object of the specified type.
    unsafe fn allocate<T>() -> Result<*mut T> {
        Self::allocate_raw(mem::size_of::<T>(), mem::align_of::<T>())
                    .map(|r| r as *mut T)
    }

    /// Allocates an array of the specified type with space for `num` elements.
    unsafe fn allocate_array<T>(num: usize) -> Result<*mut T> {
        match num.checked_mul(mem::size_of::<T>()) {
            Some(size) => Self::allocate_raw(size, mem::align_of::<T>())
                                    .map(|r| r as *mut T),
            _ => Err(error::InvalidArgument),
        }
    }

    /// Reallocates an array.
    unsafe fn reallocate_array<T>(ptr: *mut T, oldnum: usize,
                                  newnum: usize) -> Result<*mut T> {
        match newnum.checked_mul(mem::size_of::<T>()) {
            Some(size) => Self::reallocate_raw(ptr as *mut u8,
                                               oldnum * mem::size_of::<T>(), size,
                                               mem::align_of::<T>())
                                    .map(|r| r as *mut T),
            _ => Err(error::InvalidArgument),
        }
    }

    /// Frees an array with the specified properties.
    unsafe fn free_array<T>(ptr: *mut T, num: usize) {
        Self::free_raw(ptr as *mut u8, num * mem::size_of::<T>(), mem::align_of::<T>());
    }

    /// Frees an object.
    unsafe fn free<T>(ptr: *mut T) {
        Self::free_raw(ptr as *mut u8, mem::size_of::<T>(), mem::align_of::<T>());
    }
}
