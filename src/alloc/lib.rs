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

// TODO: Write an allocator

#[allow(improper_ctypes)]
extern {
    fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
}

pub unsafe fn allocate(size: usize, alignment: usize) -> *mut u8 {
    let _ = alignment;
    realloc(0 as *mut u8, size)
}

pub unsafe fn allocate_typed<T>() -> *mut T {
    realloc(0 as *mut u8, core::mem::size_of::<T>()) as *mut T
}

pub unsafe fn free(ptr: *mut u8, size: usize, alignment: usize) {
    let _ = size;
    let _ = alignment;
    realloc(ptr, 0);
}

pub unsafe fn free_typed<T>(ptr: *mut T) {
    realloc(ptr as *mut u8, 0);
}

pub unsafe fn reallocate(ptr: *mut u8, oldsize: usize, newsize: usize,
                         alignment: usize) -> *mut u8 {
    let _ = oldsize;
    let _ = alignment;
    realloc(ptr, newsize)
}
