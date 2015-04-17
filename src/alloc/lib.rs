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

#[prelude_import] use core::prelude::*;
use core::{num, mem};

// TODO: Write an allocator

pub const MAX_SIZE: usize = num::isize::MAX as usize;

#[allow(improper_ctypes)]
extern {
    fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
}

pub fn empty_ptr<T>() -> *mut T {
    static EMPTY: u8 = 0;
    &EMPTY as *const _ as *mut _
}

pub unsafe fn allocate_raw(size: usize, alignment: usize) -> *mut u8 {
    let _ = alignment;
    if size > MAX_SIZE {
        return 0 as *mut u8;
    }
    realloc(0 as *mut u8, size)
}

pub unsafe fn allocate_array<T>(num: usize) -> *mut T {
    match num.checked_mul(mem::size_of::<T>()) {
        Some(size) => allocate_raw(size, mem::align_of::<T>()) as *mut T,
        _ => 0 as *mut T,
    }
}

pub unsafe fn allocate<T>() -> *mut T {
    allocate_raw(mem::size_of::<T>(), mem::align_of::<T>()) as *mut T
}

pub unsafe fn free_raw(ptr: *mut u8, size: usize, alignment: usize) {
    let _ = size;
    let _ = alignment;
    realloc(ptr, 0);
}

pub unsafe fn free_array<T>(ptr: *mut T, num: usize) {
    free_raw(ptr as *mut u8, num * mem::size_of::<T>(), mem::align_of::<T>());
}

pub unsafe fn free<T>(ptr: *mut T) {
    free_raw(ptr as *mut u8, mem::size_of::<T>(), mem::align_of::<T>());
}

pub unsafe fn reallocate_raw(ptr: *mut u8, oldsize: usize, newsize: usize,
                             alignment: usize) -> *mut u8 {
    let _ = oldsize;
    let _ = alignment;
    if newsize > MAX_SIZE {
        return ptr;
    }
    realloc(ptr, newsize)
}

pub unsafe fn reallocate_array<T>(ptr: *mut T, oldnum: usize, newnum: usize) -> *mut T {
    match newnum.checked_mul(mem::size_of::<T>()) {
        Some(size) => reallocate_raw(ptr as *mut u8, oldnum * mem::size_of::<T>(),
                                     size, mem::align_of::<T>()) as *mut T,
        _ => 0 as *mut T,
    }
}
