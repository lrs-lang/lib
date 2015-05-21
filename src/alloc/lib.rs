// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_alloc"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_libc;

#[prelude_import] use base::prelude::*;
use core::marker::{Leak};
use core::{num, mem};
use base::{error};

pub use libc::{Libc};
pub use no::{NoMem};
pub use bda::{Bda};
pub use align::{AlignAlloc};

#[cfg(jemalloc)]
pub use jemalloc::{JeMalloc};

mod libc;
mod no;
mod bda;
mod align;

#[cfg(jemalloc)]
mod jemalloc;

/// The default allocator
pub type Heap = Libc;

pub type FbHeap = Heap;

/// The maximum size of an allocation
pub const MAX_SIZE: usize = num::isize::MAX as usize;

/// Returns a non-null pointer that points to a valid address and has pointer alignment.
pub fn empty_ptr<T>() -> *mut T {
    static EMPTY: usize = 0;
    &EMPTY as *const _ as *mut _
}

/// Allocators.
///
/// = Remarks
///
/// == Bugs
///
/// This needs better documentation.
pub trait Allocator: Leak {
    /// Allocates a chunk of bytes with the specified properties.
    ///
    /// [argument, size]
    /// The size of the allocated object.
    ///
    /// [argument, alignment]
    /// The alignment of the allocated object.
    ///
    /// [return_value]
    /// Returns the allocated object.
    ///
    /// = Remarks
    ///
    /// The alignment must be a power of two or the behavior is undefined. Not all
    /// allocators support all alignments. In general, only alignments that are required
    /// by the hardware can be satisfied.
    ///
    /// If `size` is zero, the behavior is undefined.
    unsafe fn allocate_raw(size: usize, alignment: usize) -> Result<*mut u8>;

    /// Reallocates a chunk of bytes.
    ///
    /// [argument, ptr]
    /// The pointer that should be reallocated.
    ///
    /// [argument, oldsize]
    /// The previous size of the allocation.
    ///
    /// [argument, newsize]
    /// The new size.
    ///
    /// [argument, alignment]
    /// The alignment of the allocation.
    ///
    /// [return_value]
    /// Returns the new object.
    ///
    /// = Remarks
    ///
    /// The pointer argument must have been returned by a previous invocation of
    /// `allocate_raw` or `reallocate_raw` with the same allocator. The alignment argument
    /// must be the same alignment that was previously used to allocate the object. The
    /// oldsize argument must be the current size of the object.
    ///
    /// Otherwise the behavior is undefined.
    ///
    /// If this function returns successfully, the pointer argument becomes invalid and
    /// must no longer be used. Otherwise the pointer argument can continued to be used.
    ///
    /// If the function returns successfully, the first `oldsize` bytes in the new object
    /// are the same as the first `oldsize` bytes in the old object.
    ///
    /// If `newsize` is `0`, the behavior is undefined.
    unsafe fn reallocate_raw(ptr: *mut u8, oldsize: usize, newsize: usize,
                             alignment: usize) -> Result<*mut u8>;

    /// Deallocates a chunk of bytes with the specified properties.
    ///
    /// [argument, ptr]
    /// The pointer that should be deallocated.
    ///
    /// [argument, size]
    /// The size of the allocation.
    ///
    /// [argument, alignment]
    /// The alignment of the allocation.
    ///
    /// = Remarks
    ///
    /// The pointer argument must have been returned by a previous invocation of
    /// `allocate_raw` or `reallocate_raw` with the same allocator. The alignment argument
    /// must be the same alignment that was previously used to allocate the object. The
    /// size argument must be the current size of the object.
    ///
    /// Otherwise the behavior is undefined.
    ///
    /// This function always succeeds and the pointer argument must no longer be used
    /// after this call.
    unsafe fn free_raw(ptr: *mut u8, size: usize, alignment: usize);

    /// Allocates an object of the specified type.
    ///
    /// [return_value]
    /// Returns a pointer to an allocated object.
    ///
    /// = Remarks
    ///
    /// If `T` has size `0`, the behavior is undefined.
    unsafe fn allocate<T>() -> Result<*mut T> {
        Self::allocate_raw(mem::size_of::<T>(), mem::align_of::<T>())
                    .map(|r| r as *mut T)
    }

    /// Allocates an array of the specified type.
    ///
    /// [argument, num]
    /// The number of elements in the array.
    ///
    /// [return_value]
    /// Returns a pointer to an allocated array.
    ///
    /// = Remarks
    ///
    /// If `num` is `0` or `T` has size `0`, the behavior is undefined.
    unsafe fn allocate_array<T>(num: usize) -> Result<*mut T> {
        match num.checked_mul(mem::size_of::<T>()) {
            Some(size) => Self::allocate_raw(size, mem::align_of::<T>())
                                    .map(|r| r as *mut T),
            _ => Err(error::InvalidArgument),
        }
    }

    /// Reallocates an array.
    ///
    /// [argument, ptr]
    /// The pointer that should be reallocated.
    ///
    /// [argument, num]
    /// The old number of elements in the array.
    ///
    /// [argument, newnum]
    /// The new number of elements in the array.
    ///
    /// [return_value]
    /// Returns a pointer to the new array.
    ///
    /// = Remarks
    ///
    /// The pointer argument must be a pointer returned by a previous call to
    /// `allocate_array` or `reallocate_array` with the same allocator. The oldnum
    /// argument must be the number of elements pointed to by the pointer argument.
    ///
    /// Otherwise the behavior is undefined.
    ///
    /// If `newnum` is `0`, the behavior is undefined.
    ///
    /// If this function returns successfully, the old pointer becomes invalid and must no
    /// longer be used. Otherwise the old pointer can continued to be used.
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

    /// Frees an array.
    ///
    /// [argument, ptr]
    /// The pointer to the array.
    ///
    /// [argument, num]
    /// The number of elements in the array.
    ///
    /// = Remarks
    ///
    /// The pointer argument must be a pointer returned by a previous call to
    /// `allocate_array` or `reallocate_array` with the same allocator. The num argument
    /// must be the number of elements in the array.
    ///
    /// Otherwise the behavior is undefined.
    ///
    /// After this function returns the pointer argument becomes invalid and must no
    /// longer be used.
    unsafe fn free_array<T>(ptr: *mut T, num: usize) {
        Self::free_raw(ptr as *mut u8, num * mem::size_of::<T>(), mem::align_of::<T>());
    }

    /// Frees an object.
    ///
    /// [argument, ptr]
    /// The pointer to the object.
    ///
    /// = Remarks
    ///
    /// The pointer argument must be a pointer returned by a previous call to `allocate`
    /// with the same allocator. Otherwise the behavior is undefined.
    ///
    /// After this function returns the pointer argument becomes invalid and must no
    /// longer be used.
    unsafe fn free<T>(ptr: *mut T) {
        Self::free_raw(ptr as *mut u8, mem::size_of::<T>(), mem::align_of::<T>());
    }
}
