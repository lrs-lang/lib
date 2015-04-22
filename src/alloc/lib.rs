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
extern crate linux_libc as libc;

#[prelude_import] use core::prelude::*;
use core::{num, mem};

// TODO: Write an allocator

/// The maximum size of an allocation.
pub const MAX_SIZE: usize = num::isize::MAX as usize;

/// Returns a non-null pointer that points to a vaild address and has pointer alignment.
pub fn empty_ptr<T>() -> *mut T {
    static EMPTY: usize = 0;
    &EMPTY as *const _ as *mut _
}

/// Allocates a chunk of bytes with the specified properties.
///
/// `alignment` must be a power of two.
pub unsafe fn allocate_raw(mut size: usize, mut alignment: usize) -> *mut u8 {
    alignment -= 1;
    size += alignment;
    if size > MAX_SIZE {
        return 0 as *mut u8;
    }
    let ptr = libc::realloc(0 as *mut u8, size) as usize;
    ((ptr + alignment) & !alignment) as *mut u8
}

/// Allocates an array of the specified type with space for `num` elements.
pub unsafe fn allocate_array<T>(num: usize) -> *mut T {
    match num.checked_mul(mem::size_of::<T>()) {
        Some(size) => allocate_raw(size, mem::align_of::<T>()) as *mut T,
        _ => 0 as *mut T,
    }
}

/// Allocates an object of the specified type.
pub unsafe fn allocate<T>() -> *mut T {
    allocate_raw(mem::size_of::<T>(), mem::align_of::<T>()) as *mut T
}

/// Frees a chunk of bytes with the specified properties.
pub unsafe fn free_raw(ptr: *mut u8, size: usize, alignment: usize) {
    let _ = size;
    let _ = alignment;
    libc::realloc(ptr, 0);
}

/// Frees an array with the specified properties.
pub unsafe fn free_array<T>(ptr: *mut T, num: usize) {
    free_raw(ptr as *mut u8, num * mem::size_of::<T>(), mem::align_of::<T>());
}

/// Frees an object.
pub unsafe fn free<T>(ptr: *mut T) {
    free_raw(ptr as *mut u8, mem::size_of::<T>(), mem::align_of::<T>());
}

/// Reallocates a chunk of bytes.
pub unsafe fn reallocate_raw(ptr: *mut u8, oldsize: usize, newsize: usize,
                             alignment: usize) -> *mut u8 {
    let _ = oldsize;
    let _ = alignment;
    if newsize > MAX_SIZE {
        return ptr;
    }
    libc::realloc(ptr, newsize)
}

/// Reallocates an array.
pub unsafe fn reallocate_array<T>(ptr: *mut T, oldnum: usize, newnum: usize) -> *mut T {
    match newnum.checked_mul(mem::size_of::<T>()) {
        Some(size) => reallocate_raw(ptr as *mut u8, oldnum * mem::size_of::<T>(),
                                     size, mem::align_of::<T>()) as *mut T,
        _ => 0 as *mut T,
    }
}
