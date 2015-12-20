// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_vec"]
#![crate_type = "lib"]
#![feature(optin_builtin_traits)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_str_one as str_one;
extern crate lrs_io as io;
extern crate lrs_box as bx;
extern crate lrs_fmt as fmt;
extern crate lrs_alloc as alloc;

pub mod std {
    pub use fmt::std::*;
    pub mod vec { pub use {Vec}; }
}

use base::prelude::*;
use core::{mem, ptr, cmp, slice};
use core::ptr::{NoAliasMemPtr};
use base::undef::{UndefState};
use core::iter::{IntoIterator};
use fmt::{Write, Debug};
use alloc::{MemPool, empty_ptr};

mod conv;
mod cmp_;
mod drain;
mod byte_vec;

/// A vector.
pub struct Vec<T, Pool: ?Sized = alloc::Heap>
    where Pool: MemPool,
{
    ptr: NoAliasMemPtr<T>,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
    pool: Pool,
}

impl<T, H = alloc::Heap> Vec<T, H>
    where H: MemPool,
{
    /// Creates a new allocating vector.
    pub fn new() -> Vec<T, H>
        where H: OutOf,
    {
        Self::with_pool(H::out_of(()))
    }

    /// Creates a new allocating vector with a memory pool.
    ///
    /// [argument, pool]
    /// The pool to draw memory from.
    pub fn with_pool(pool: H) -> Vec<T, H> {
        let ptr = unsafe { NoAliasMemPtr::new(empty_ptr()) };
        Vec {
            ptr: ptr,
            len: 0,
            cap: 0,
            pool: pool,
            _marker: PhantomData,
        }
    }

    /// Creates a new allocating vector and reserves a certain amount of space for it.
    pub fn with_capacity(cap: usize) -> Result<Vec<T, H>>
        where H: OutOf,
    {
        let mut pool = H::out_of(());
        if cap == 0 || mem::size_of::<T>() == 0 {
            let ptr = unsafe { NoAliasMemPtr::new(empty_ptr()) };
            return Ok(Vec {
                ptr: ptr,
                len: 0,
                cap: cap,
                pool: pool,
                _marker: PhantomData,
            });
        }
        let (ptr, cap) = unsafe { try!(alloc::alloc_array(&mut pool, cap)) };
        let ptr = unsafe { NoAliasMemPtr::new(ptr) };
        Ok(Vec {
            ptr: ptr,
            len: 0,
            cap: cap,
            pool: pool,
            _marker: PhantomData,
        })
    }

    /// Creates a new vector from its raw parts.
    ///
    /// [argument, ptr]
    /// The pointer to the first element of the vector.
    ///
    /// [argument, len]
    /// The number of elements in the vector.
    ///
    /// [argument, cap]
    /// The capacity of the array pointed to by the pointer.
    ///
    /// = Remarks
    ///
    /// The allocator must be the same allocator that was used to allocate the memory.
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize, cap: usize,
                                 pool: H) -> Vec<T, H> {
        Vec {
            ptr: NoAliasMemPtr::new(ptr),
            len: len,
            cap: cap,
            pool: pool,
            _marker: PhantomData,
        }
    }

    pub unsafe fn into_raw_parts(self) -> (*mut T, usize, usize, H) {
        let ptr = self.ptr.get();
        let len = self.len;
        let cap = self.cap;
        let pool = ptr::read(&self.pool);
        mem::unsafe_forget(self);
        (ptr, len, cap, pool)
    }
}

impl<T, H: ?Sized = alloc::Heap> Vec<T, H>
    where H: MemPool,
{
    /// Returns the capacity of the vector.
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Returns the number of available but unused slots.
    pub fn available(&self) -> usize {
        self.cap - self.len
    }

    pub fn unused(&mut self) -> &mut [d8] {
        if mem::size_of::<T>() == 0 {
            &mut []
        } else {
            let size = (self.cap - self.len) * mem::size_of::<T>();
            unsafe {
                slice::from_ptr(self.ptr.get().add(self.len) as *mut d8, size)
            }
        }
    }

    /// Reserves memory for additional elements.
    ///
    /// [argument, n]
    /// The number of elements for which memory should be reserved.
    pub fn reserve(&mut self, n: usize) -> Result {
        if self.cap - self.len >= n {
            return Ok(());
        }
        if mem::size_of::<T>() == 0 {
            self.cap = self.len + n;
            return Ok(());
        }

        let new_cap = self.len + cmp::max(n, self.cap / 2 + 1);
        let (ptr, new_cap) = unsafe {
            if self.ptr.get() == empty_ptr() {
                try!(alloc::alloc_array(&mut self.pool, new_cap))
            } else {
                try!(alloc::realloc_array(&mut self.pool, self.ptr.get(),
                                          self.cap, new_cap))
            }
        };
        self.ptr = unsafe { NoAliasMemPtr::new(ptr) };
        self.cap = new_cap;
        Ok(())
    }

    /// Minimizes the amount of used memory.
    pub fn shrink_to_fit(&mut self) -> Result {
        if mem::size_of::<T>() == 0 {
            self.cap = self.len;
            return Ok(());
        }
        if self.len < self.cap {
            let (ptr, cap) = unsafe {
                try!(alloc::realloc_array(&mut self.pool, self.ptr.get(), self.cap,
                                          self.len))
            };
            self.ptr = unsafe { NoAliasMemPtr::new(ptr) };
            self.cap = cap;
        }
        Ok(())
    }

    /// Appends an element to the vector.
    ///
    /// [argument, val]
    /// The element to append.
    ///
    /// = Remarks
    ///
    /// This method aborts the process if no memory is available and allocating additional
    /// memory fails. To avoid this, use `reserve` or `try_push`.
    pub fn push(&mut self, val: T) {
        if self.cap == self.len {
            self.reserve(1).unwrap();
        }
        unsafe { ptr::write(self.ptr.get().add(self.len), val); }
        self.len += 1;
    }

    /// Tries to append a copyable element to the vector.
    ///
    /// [argument, val]
    /// The element to append.
    pub fn try_push(&mut self, val: T) -> Result where T: Copy {
        if self.cap == self.len {
            try!(self.reserve(1));
        }
        unsafe { ptr::write(self.ptr.get().add(self.len), val); }
        self.len += 1;
        Ok(())
    }

    /// Appends a slice of copyable elements to the vector.
    ///
    /// [argument, vals]
    /// The elements to append.
    ///
    /// = Remarks
    ///
    /// If this operation fails, no elements have been appended.
    pub fn push_all(&mut self, vals: &[T]) -> Result where T: Copy {
        unsafe { self.try_unsafe_push_all(vals) }
    }

    /// Appends a slice of non-copyable elements to the vector.
    ///
    /// [argument, vals]
    /// The elements to append.
    ///
    /// = Remarks
    ///
    /// If this operation fails, no elements have been appended. The elements will be
    /// copied as if they were copyable. The user has to ensure the safety of this
    /// operation.
    pub unsafe fn try_unsafe_push_all(&mut self, vals: &[T]) -> Result {
        try!(self.reserve(vals.len()));
        let tail = slice::from_ptr(self.ptr.get().add(self.len), vals.len());
        mem::unsafe_copy(tail, vals);
        self.len += vals.len();
        Ok(())
    }

    /// Extends the vector by the elements of an iterator.
    ///
    /// [argument, iter]
    /// The iter whose elements will be appended to the vector.
    ///
    /// = Remarks
    ///
    /// This method aborts the process if no memory is available and allocating additional
    /// memory fails.
    pub fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }

    /// Removes an element from the end of the vector.
    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            _ => {
                self.len -= 1;
                unsafe { Some(ptr::read(self.ptr.get().add(self.len))) }
            },
        }
    }

    /// Reduces the length of the vector.
    ///
    /// [argument, len]
    /// The new length of the vector.
    ///
    /// = Remarks
    ///
    /// If len is greater than the current length of the vector, the process is aborted.
    pub fn truncate(&mut self, len: usize) {
        assert!(len <= self.len);
        if mem::needs_drop::<T>() {
            for i in len..self.len {
                unsafe { ptr::drop(self.ptr.get().add(i)); }
            }
        }
        self.len = len;
    }

    /// Sets the length of the vector.
    ///
    /// [argument, len]
    /// The new length of the vector.
    ///
    /// = Remarks
    ///
    /// If len is greater than the current capacity of the vector, the process is aborted.
    pub unsafe fn set_len(&mut self, len: usize) {
        assert!(len <= self.cap);
        self.len = len;
    }

    pub fn insert(&mut self, pos: usize, val: T) {
        let len = self.len();
        if pos == len {
            self.push(val);
        } else if pos <= len {
            self.reserve(1).unwrap();
            unsafe {
                self.set_len(len + 1);
                ptr::memmove(self.as_mut_ptr().add(pos + 1), self.as_ptr().add(pos),
                             len - pos);
                ptr::write(self.as_mut_ptr().add(pos), val);
            }
        } else {
            abort!();
        }
    }
}

unsafe impl<T, H> UndefState for Vec<T, H>
    where H: MemPool,
{
    fn num() -> usize { <NoAliasMemPtr<T> as UndefState>::num() }

    unsafe fn set_undef(val: *mut Vec<T, H>, n: usize) {
        <NoAliasMemPtr<T> as UndefState>::set_undef(&mut (*val).ptr, n);
    }

    unsafe fn is_undef(val: *const Vec<T, H>, n: usize) -> bool {
        <NoAliasMemPtr<T> as UndefState>::is_undef(&(*val).ptr, n)
    }
}

unsafe impl<T, H> Sync for Vec<T, H> where T: Sync, H: MemPool, { }
unsafe impl<T, H: ?Sized> Send for Vec<T, H> where T: Send, H: MemPool+Send { }

impl<T, H: ?Sized> Drop for Vec<T, H>
    where H: MemPool,
{
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            for i in 0..self.len {
                unsafe { ptr::drop(self.ptr.get().add(i)); }
            }
        }
        if self.ptr.get() != empty_ptr() {
            unsafe {
                alloc::free_array(&mut self.pool, self.ptr.get(), self.cap);
            }
        }
    }
}

impl<T, H: ?Sized> Deref for Vec<T, H>
    where H: MemPool,
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_ptr(self.ptr.get(), self.len) }
    }
}

impl<T, H: ?Sized> DerefMut for Vec<T, H>
    where H: MemPool,
{
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_ptr(self.ptr.get(), self.len) }
    }
}

impl<T, H: ?Sized> Debug for Vec<T, H>
    where T: Debug,
          H: MemPool,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.deref().fmt(w)
    }
}

impl<'a, T, H: ?Sized> IntoIterator for &'a Vec<T, H>
    where H: MemPool,
{
    type Item = &'a T;
    type IntoIter = slice::Items<'a, T>;
    fn into_iter(self) -> slice::Items<'a, T> { self.iter() }
}

impl<'a, T, H: ?Sized> IntoIterator for &'a mut Vec<T, H>
    where H: MemPool,
{
    type Item = &'a mut T;
    type IntoIter = slice::MutItems<'a, T>;
    fn into_iter(self) -> slice::MutItems<'a, T> { self.iter_mut() }
}
