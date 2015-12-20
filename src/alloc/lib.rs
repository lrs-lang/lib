// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_alloc"]
#![crate_type = "lib"]
#![feature(no_std, const_fn, link_llvm_intrinsics, optin_builtin_traits)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
#[cfg(not(freestanding))] extern crate lrs_syscall as syscall;
#[cfg(not(freestanding))] extern crate lrs_tlalc as tlalc;
#[cfg(not(freestanding))] extern crate lrs_lock as lock;
#[cfg(not(no_libc))] extern crate lrs_libc;

use base::prelude::*;
use core::marker::{Leak};
use core::{mem};
use base::{error};

pub use no::{Dummy};
pub use ta::{TaPool};
pub use fc::{FcPool};
pub use one::{OncePool};
pub use align::{AlignAlloc};
#[cfg(not(no_libc))] pub use libc::{Libc};
#[cfg(not(freestanding))] pub use bda::{Bda};
#[cfg(not(freestanding))] pub use tl::{TlAlc};

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
#[cfg(not(freestanding))] mod tl;

#[cfg(jemalloc)]
mod jemalloc;

// NOTE: The default allocator should be `Default`, `Copy`, `Send`, `Sync`, etc.

/// The default allocator.
#[cfg(not(no_libc))] pub type Heap = Libc;

/// The default allocator.
#[cfg(all(no_libc, not(freestanding)))] pub type Heap = Bda;

/// The default allocator.
#[cfg(all(no_libc, freestanding))] pub type Heap = Dummy<'static>;

/// The default thread-local allocator.
#[cfg(not(freestanding))] pub type ThreadHeap = TlAlc;

/// The default thread-local allocator.
#[cfg(freestanding)] pub type ThreadHeap = Dummy<'static>;

pub type FbHeap = Heap;

/// The maximum size of an allocation
pub const MAX_SIZE: usize = isize::max() as usize;

/// Returns a non-null pointer that points to a valid address and has pointer alignment.
pub fn empty_ptr<T>() -> *mut T {
    static EMPTY: usize = 0;
    &EMPTY as *const _ as *mut _
}

/// Memory pools.
///
/// = Description
///
/// == Definitions
///
/// This section defines various terms used in the remained of the documentation.
///
/// === slot
///
/// An opaque object representing an allocation.
///
/// === range of a slot
///
/// A non-empty continuous memory region.
///
/// === address of a slot
///
/// The positive minimal address of the range of the slot.
///
/// === alignment of a slot
///
/// A power of two that is a factor of the address of the slot.
///
/// === pointer to a slot
///
/// A pointer that contains the address of the slot.
///
/// === size of a slot
///
/// The size of the range of the slot that is bounded above by the largest value that can
/// be stored in an `isize` object.
///
/// === content of a slot
///
/// The bytes stored in the range of the slot.
///
/// === minimal size of a slot
///
/// A positive unsigned integer bounded above by the size of the slot.
pub trait MemPool: Leak {
    /// Attempts to create a slot.
    ///
    /// [argument, size]
    /// A lower bound of the size of the slot.
    ///
    /// [argument, alignment]
    /// See the description.
    ///
    /// [return_value]
    /// Returns a pointer to the slot or an error.
    ///
    /// = Description
    ///
    /// On success: the returned pointer does not alias any other pointer; the minimal
    /// size of the slot is bounded above by the `size` argument.
    ///
    /// The `alignment` argument is a power of two. It is implementation defined whether
    /// it has any influence on the alignment of the slot.
    ///
    /// The contents of the slot are unspecified. The caller has exclusive read and write
    /// access to the range of the slot.
    unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Result<*mut d8>;

    /// Attempts to resize a slot.
    ///
    /// [argument, self]
    /// The allocator that created the slot.
    ///
    /// [argument, ptr]
    /// A pointer to the slot.
    ///
    /// [argument, cur_size]
    /// An integer bounded below by the minimal size of the slot and above by the size of
    /// the slot.
    ///
    /// [argument, new_size]
    /// A lower bound for the new size of the slot.
    ///
    /// [argument, alignment]
    /// The same value as the `alignment` argument used to create the slot.
    ///
    /// [return_value]
    /// Returns a pointer to the resized slot or an error.
    ///
    /// = Description
    ///
    /// On success: the address of the slot can have changed; the minimal size of the slot
    /// is bounded above by the `new_size` argument.
    unsafe fn realloc(&mut self, ptr: *mut d8, cur_size: usize,
                      new_size: usize, alignment: usize) -> Result<*mut d8>;

    unsafe fn free(&mut self, ptr: *mut d8, size: usize, alignment: usize);

    unsafe fn realloc_in_place(&mut self, ptr: *mut d8, oldsize: usize,
                               newsize: usize, alignment: usize) -> Result {
        let _ = ptr;
        let _ = oldsize;
        let _ = newsize;
        let _ = alignment;
        Err(error::NoMemory)
    }

    unsafe fn usable_size(&self, ptr: *mut d8, size: usize, alignment: usize) -> usize {
        let _ = ptr;
        let _ = alignment;
        size
    }
}

impl<'a, T: MemPool> MemPool for &'a mut T {
    unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Result<*mut d8> {
        (**self).alloc(size, alignment)
    }

    unsafe fn realloc(&mut self, ptr: *mut d8, oldsize: usize,
                      newsize: usize, alignment: usize) -> Result<*mut d8> {
        (**self).realloc(ptr, oldsize, newsize, alignment)
    }

    unsafe fn free(&mut self, ptr: *mut d8, size: usize, alignment: usize) {
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
            Some(total_size) => {
                let align = mem::align_of::<T>();
                let ptr = try!(pool.alloc(total_size, align));
                let num = pool.usable_size(ptr, total_size, align) / size;
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
pub unsafe fn realloc_array<T, M: ?Sized>(pool: &mut M, old_ptr: *mut T, old_num: usize,
                                          new_num: usize) -> Result<(*mut T, usize)>
    where M: MemPool,
{
    let size = mem::size_of::<T>();
    if size == 0 {
        Ok((empty_ptr(), 0))
    } else {
        match new_num.checked_mul(size) {
            Some(new_size) => {
                let align = mem::align_of::<T>();
                let old_ptr = old_ptr as *mut d8;
                let old_size = old_num * size;
                let new_ptr = try!(pool.realloc(old_ptr, old_size, new_size, align));
                let new_num = pool.usable_size(new_ptr, new_size, align) / size;
                Ok((new_ptr as *mut T, new_num))
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
        pool.free(ptr as *mut d8, num * mem::size_of::<T>(), mem::align_of::<T>());
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
        pool.free(ptr as *mut d8, size, mem::align_of_val(&*ptr));
    }
}
