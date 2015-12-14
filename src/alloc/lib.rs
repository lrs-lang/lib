// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_alloc"]
#![crate_type = "lib"]
#![feature(no_std, const_fn, link_llvm_intrinsics)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
#[cfg(not(freestanding))] extern crate lrs_syscall as syscall;
#[cfg(not(no_libc))] extern crate lrs_libc;

use base::prelude::*;
use core::marker::{Leak};
use core::{mem};
use base::{error};

pub use no::{NoMem};
pub use ta::{TaPool};
pub use fc::{FcPool};
pub use one::{OncePool};
pub use align::{AlignAlloc};
#[cfg(not(no_libc))] pub use libc::{Libc};
#[cfg(not(freestanding))] pub use bda::{Bda};

#[cfg(jemalloc)]
pub use jemalloc::{JeMalloc};

mod std { pub use base::std::*; }

mod no;
mod align;
mod ta;
mod one;
mod fc;
#[cfg(not(no_libc))] mod libc;
#[cfg(not(freestanding))] mod bda;

#[cfg(jemalloc)]
mod jemalloc;

// NOTE: The default allocator should be `Default`, `Copy`, `Send`, `Sync`, etc.

/// The default allocator
#[cfg(not(no_libc))] pub type Heap = Libc;

/// The default allocator
#[cfg(all(no_libc, not(freestanding)))] pub type Heap = Bda;

/// The default allocator
#[cfg(all(no_libc, freestanding))] pub type Heap = NoMem<'static>;

pub type FbHeap = Heap;

/// The maximum size of an allocation
pub const MAX_SIZE: usize = isize::max() as usize;

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
pub trait MemPool: Leak {
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
    unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Result<*mut u8>;

    /// Reallocates a chunk of bytes.
    ///
    /// [argument, pool]
    /// The pool from which to draw memory.
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
    /// `allocate_raw` or `reallocate_raw` with the same allocator and pool. The alignment
    /// argument must be the same alignment that was previously used to allocate the
    /// object. The oldsize argument must be the current size of the object.
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
    unsafe fn realloc(&mut self, ptr: *mut u8, oldsize: usize,
                      newsize: usize, alignment: usize) -> Result<*mut u8>;

    /// Deallocates a chunk of bytes with the specified properties.
    ///
    /// [argument, pool]
    /// The pool to which the memory is returned.
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
    /// `allocate_raw` or `reallocate_raw` with the same allocator and pool. The alignment
    /// argument must be the same alignment that was previously used to allocate the
    /// object. The size argument must be the current size of the object.
    ///
    /// Otherwise the behavior is undefined.
    ///
    /// This function always succeeds and the pointer argument must no longer be used
    /// after this call.
    unsafe fn free(&mut self, ptr: *mut u8, size: usize, alignment: usize);

    unsafe fn realloc_in_place(&mut self, ptr: *mut u8, oldsize: usize,
                               newsize: usize, alignment: usize) -> Result {
        let _ = ptr;
        let _ = oldsize;
        let _ = newsize;
        let _ = alignment;
        Err(error::NoMemory)
    }

    unsafe fn usable_size(&self, ptr: *mut u8, size: usize, alignment: usize) -> usize {
        let _ = ptr;
        let _ = alignment;
        size
    }
}

impl<'a, T: MemPool> MemPool for &'a mut T {
    unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Result<*mut u8> {
        (**self).alloc(size, alignment)
    }

    unsafe fn realloc(&mut self, ptr: *mut u8, oldsize: usize,
                      newsize: usize, alignment: usize) -> Result<*mut u8> {
        (**self).realloc(ptr, oldsize, newsize, alignment)
    }

    unsafe fn free(&mut self, ptr: *mut u8, size: usize, alignment: usize) {
        (**self).free(ptr, size, alignment)
    }
}

/// Allocates an object of the specified type.
///
/// [argument, pool]
/// The pool from which to draw memory.
///
/// [return_value]
/// Returns a pointer to an allocated object.
///
/// = Remarks
///
/// If `T` has size `0`, a call to this function behaves like a call to `empty_ptr`.
pub unsafe fn alloc<T, M: ?Sized>(pool: &mut M) -> Result<*mut T>
    where M: MemPool,
{
    if mem::size_of::<T>() == 0 {
        Ok(empty_ptr())
    } else {
        pool.alloc(mem::size_of::<T>(), mem::align_of::<T>()).map(|r| r as *mut T)
    }
}

/// Allocates an array of the specified type.
///
/// [argument, pool]
/// The pool from which to draw memory.
///
/// [argument, num]
/// The number of elements in the array.
///
/// [return_value]
/// Returns a pointer to an allocated array.
///
/// = Remarks
///
/// If `T` has size `0`, a call to this function behaves like a call to `empty_ptr`.
/// Otherwise, if `num` is `0`, the behavior is undefined.
pub unsafe fn alloc_array<T, M: ?Sized>(pool: &mut M,
                                        num: usize) -> Result<(*mut T, usize)>
    where M: MemPool,
{
    let size = mem::size_of::<T>();
    if size == 0 {
        Ok((empty_ptr(), 0))
    } else {
        match num.checked_mul(size) {
            Some(buf_size) => {
                let align = mem::align_of::<T>();
                let ptr = try!(pool.alloc(buf_size, align));
                let num = pool.usable_size(ptr, buf_size, align) / size;
                Ok((ptr as *mut T, num))
            },
            _ => Err(error::InvalidArgument),
        }
    }
}

/// Reallocates an array.
///
/// [argument, pool]
/// The pool from which to draw memory.
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
/// If `T` has size `0`, a call to this function behaves like a call to `empty_ptr`.
/// Otherwise:
///
/// The pointer argument must be a pointer returned by a previous call to
/// `allocate_array` or `reallocate_array` with the same allocator and pool. The
/// oldnum argument must be the number of elements pointed to by the pointer argument.
///
/// Otherwise the behavior is undefined.
///
/// If `newnum` is `0`, the behavior is undefined.
///
/// If this function returns successfully, the old pointer becomes invalid and must no
/// longer be used. Otherwise the old pointer can continued to be used.
pub unsafe fn realloc_array<T, M: ?Sized>(pool: &mut M, ptr: *mut T, oldnum: usize,
                                          newnum: usize) -> Result<(*mut T, usize)>
    where M: MemPool,
{
    let size = mem::size_of::<T>();
    if size == 0 {
        Ok((empty_ptr(), 0))
    } else {
        match newnum.checked_mul(size) {
            Some(buf_size) => {
                let align = mem::align_of::<T>();
                let old_ptr = ptr as *mut u8;
                let old_size = oldnum * size;
                let new_ptr = try!(pool.realloc(old_ptr, old_size, size, align));
                let num = pool.usable_size(new_ptr, buf_size, align) / size;
                Ok((new_ptr as *mut T, num))
            },
            _ => Err(error::InvalidArgument),
        }
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
/// If `T` has size `0`, this function performs no operation. Otherwise:
///
/// The pointer argument must be a pointer returned by a previous call to
/// `allocate_array` or `reallocate_array` with the same allocator. The num argument
/// must be the number of elements in the array.
///
/// Otherwise the behavior is undefined.
///
/// After this function returns the pointer argument becomes invalid and must no
/// longer be used.
pub unsafe fn free_array<T, M: ?Sized>(pool: &mut M, ptr: *mut T, num: usize)
    where M: MemPool,
{
    if mem::size_of::<T>() != 0 {
        pool.free(ptr as *mut u8, num * mem::size_of::<T>(), mem::align_of::<T>());
    }
}

/// Frees an object.
///
/// [argument, pool]
/// The pool to which the memory is returned.
///
/// [argument, ptr]
/// The pointer to the object.
///
/// = Remarks
///
/// If `T` has size `0`, this function performs no operation. Otherwise:
///
/// The pointer argument must be a pointer returned by a previous call to `allocate`
/// with the same allocator and pool. Otherwise the behavior is undefined.
///
/// After this function returns the pointer argument becomes invalid and must no
/// longer be used.
pub unsafe fn free<T: ?Sized, M: ?Sized>(pool: &mut M, ptr: *mut T)
    where M: MemPool,
{
    let size = mem::size_of_val(&*ptr);
    if size != 0 {
        pool.free(ptr as *mut u8, size, mem::align_of_val(&*ptr));
    }
}
