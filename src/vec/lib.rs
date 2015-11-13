// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_vec"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits, associated_consts)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_str_one as str_one;
extern crate lrs_io as io;
extern crate lrs_fmt as fmt;
extern crate lrs_alloc as alloc;

pub mod std {
    pub use fmt::std::*;
    pub mod vec { pub use {Vec}; }
}

use base::prelude::*;
use core::{mem, ptr, cmp, slice};
use core::ptr::{OwnedPtr};
use base::clone::{MaybeClone};
use base::undef::{UndefState};
use base::default::{Default};
use core::ops::{Eq, Range};
use core::iter::{IntoIterator};
use io::{Read, Write, BufWrite};
use fmt::{Debug};
use alloc::{Allocator, empty_ptr};
use str_one::{ByteStr, CStr, AsCStr, AsMutCStr, ToCStr, NoNullStr, AsMutNoNullStr,
              AsNoNullStr};

/// A vector.
pub struct Vec<T, Heap = alloc::Heap>
    where Heap: Allocator,
{
    ptr: OwnedPtr<T>,
    len: usize,
    cap: usize,
    pool: Heap::Pool,
}

impl<'a, T> Vec<T, alloc::NoMem<'a>> {
    /// Creates a vector which is backed by borrowed memory.
    ///
    /// [argument, buf]
    /// The buffer which will be used to store elements it.
    pub fn buffered(buf: &'a mut [u8]) -> Vec<T, alloc::NoMem<'a>> {
        if mem::size_of::<T>() == 0 {
            let ptr = unsafe { OwnedPtr::new(empty_ptr()) };
            return Vec { ptr: ptr, len: 0, cap: 0, pool: () };
        }

        let buf = mem::align_for_mut::<T>(buf);
        let cap = buf.len() / mem::size_of::<T>();
        let ptr = unsafe { OwnedPtr::new(buf.as_mut_ptr() as *mut T) };
        Vec { ptr: ptr, len: 0, cap: cap, pool: () }
    }
}

impl<T, H> Vec<T, H>
    where H: Allocator,
{
    /// Creates a new allocating vector.
    pub fn new() -> Vec<T, H>
        where H::Pool: Default,
    {
        let ptr = unsafe { OwnedPtr::new(empty_ptr()) };
        Vec { ptr: ptr, len: 0, cap: 0, pool: H::Pool::default(), }
    }

    /// Creates a new allocating vector and reserves a certain amount of space for it.
    pub fn with_capacity(cap: usize) -> Result<Vec<T, H>>
        where H::Pool: Default,
    {
        let mut pool = H::Pool::default();
        if cap == 0 || mem::size_of::<T>() == 0 {
            let ptr = unsafe { OwnedPtr::new(empty_ptr()) };
            return Ok(Vec { ptr: ptr, len: 0, cap: cap, pool: pool });
        }
        let ptr = unsafe { try!(H::allocate_array(&mut pool, cap)) };
        let ptr = unsafe { OwnedPtr::new(ptr) };
        Ok(Vec { ptr: ptr, len: 0, cap: cap, pool: pool })
    }

    /// Creates a new allocating vector with a memory pool.
    ///
    /// [argument, pool]
    /// The pool to draw memory from.
    pub fn with_pool(pool: H::Pool) -> Vec<T, H> {
        let ptr = unsafe { OwnedPtr::new(empty_ptr()) };
        Vec { ptr: ptr, len: 0, cap: 0, pool: pool }
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
                                 pool: H::Pool) -> Vec<T, H> {
        Vec {
            ptr: OwnedPtr::new(ptr),
            len: len,
            cap: cap,
            pool: pool,
        }
    }

    /// Returns the capacity of the vector.
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Returns the number of available but unused slots.
    pub fn available(&self) -> usize {
        self.cap - self.len
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
        let ptr = if *self.ptr == empty_ptr() {
            unsafe { H::allocate_array(&mut self.pool, new_cap) }
        } else {
            unsafe { H::reallocate_array(&mut self.pool, *self.ptr, self.cap, new_cap) }
        };
        self.ptr = unsafe { OwnedPtr::new(try!(ptr)) };
        self.cap = new_cap;
        Ok(())
    }

    /// Minimizes the amount of used memory.
    pub fn shrink_to_fit(&mut self) -> Result {
        if self.len < self.cap {
            let ptr = unsafe { H::reallocate_array(&mut self.pool, *self.ptr, self.cap,
                                                   self.len) };
            self.ptr = unsafe { OwnedPtr::new(try!(ptr)) };
            self.cap = self.len;
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
        unsafe { ptr::write(self.ptr.add(self.len), val); }
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
        unsafe { ptr::write(self.ptr.add(self.len), val); }
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
        let tail = slice::from_ptr(self.ptr.add(self.len), vals.len());
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
                unsafe { Some(ptr::read(self.ptr.add(self.len))) }
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
                unsafe { ptr::drop(self.ptr.add(i)); }
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

    /// Removes a range from a vector, making its elements available through an iterator.
    ///
    /// [argument, range]
    /// The range to remove.
    ///
    /// [return_value]
    /// Returns an iterator over the elements of the range.
    ///
    /// = Remarks
    ///
    /// If the range is not increasing or goes beyond the bounds of the vector, the
    /// process is aborted.
    pub fn drain<'a, R>(&mut self, range: R) -> Drainer<'a, T>
        where R: Into<Range<Option<usize>>>,
    {
        let Range { start, end } = range.into();
        let (start, end) = match (start, end) {
            (Some(s), Some(e)) => (s, e),
            (Some(s), None) => (s, self.len()),
            (None, Some(e)) => (0, e),
            (None, None) => (0, self.len()),
        };
        if start > end || end > self.len() {
            abort!();
        }
        let old_len = self.len();
        self.len -= end - start;
        if mem::size_of::<T>() != 0 {
            unsafe {
                Drainer {
                    start: self.ptr.add(start),
                    cur: self.ptr.add(start),
                    end: self.ptr.add(end),
                    vec_end: self.ptr.add(old_len),
                    _data: PhantomData,
                }
            }
        } else {
            Drainer {
                start: start as *const T,
                cur: start as *const T,
                end: end as *const T,
                vec_end: old_len as *const T,
                _data: PhantomData,
            }
        }
    }
}

unsafe impl<T, H> UndefState for Vec<T, H>
    where H: Allocator,
{
    fn num() -> usize { <OwnedPtr<T> as UndefState>::num() }

    unsafe fn set_undef(val: *mut Vec<T, H>, n: usize) {
        <OwnedPtr<T> as UndefState>::set_undef(&mut (*val).ptr, n);
    }

    unsafe fn is_undef(val: *const Vec<T, H>, n: usize) -> bool {
        <OwnedPtr<T> as UndefState>::is_undef(&(*val).ptr, n)
    }
}

impl<H> BufWrite for Vec<u8, H>
    where H: Allocator,
{
    fn read_to_eof<R>(&mut self, mut r: R) -> Result<usize>
        where R: Read,
    {
        const BUF_READ_STEP_SIZE: usize = 4096;

        let mut len = 0;
        loop {
            let self_len = self.len();
            try!(self.reserve(BUF_READ_STEP_SIZE));
            unsafe { self.set_len(self_len + BUF_READ_STEP_SIZE); }
            match r.read_all(&mut self[self_len..self_len+BUF_READ_STEP_SIZE]) {
                Ok(BUF_READ_STEP_SIZE) => len += BUF_READ_STEP_SIZE,
                Ok(n) => {
                    unsafe { self.set_len(self_len + n); }
                    len += n;
                    break;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(len)
    }
}

unsafe impl<T, H> Sync for Vec<T, H> where T: Sync, H: Allocator, { }
unsafe impl<T, H> Send for Vec<T, H> where T: Send, H: Allocator+Send { }

impl<T, H> Drop for Vec<T, H>
    where H: Allocator,
{
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                for i in 0..self.len {
                    ptr::drop(self.ptr.add(i));
                }
            }
            if *self.ptr != empty_ptr() {
                H::free_array(&mut self.pool, *self.ptr, self.cap);
            }
        }
    }
}

impl<T, H1, H2> Eq<Vec<T, H1>> for Vec<T, H2>
    where T: Eq,
          H1: Allocator,
          H2: Allocator,
{
    fn eq(&self, other: &Vec<T, H1>) -> bool {
        self.deref().eq(other.deref())
    }
    fn ne(&self, other: &Vec<T, H1>) -> bool {
        self.deref().ne(other.deref())
    }
}

impl<T, H> Eq<[T]> for Vec<T, H>
    where T: Eq,
          H: Allocator,
{
    fn eq(&self, other: &[T]) -> bool {
        self.deref().eq(other)
    }
    fn ne(&self, other: &[T]) -> bool {
        self.deref().ne(other)
    }
}

impl<T, H> Deref for Vec<T, H>
    where H: Allocator,
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_ptr(*self.ptr, self.len) }
    }
}

impl<T, H> DerefMut for Vec<T, H>
    where H: Allocator,
{
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_ptr(*self.ptr, self.len) }
    }
}

impl<T, H> Debug for Vec<T, H>
    where T: Debug,
          H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.deref().fmt(w)
    }
}

impl<T, H> MaybeClone for Vec<T, H>
    where T: MaybeClone,
          H: Allocator,
          H::Pool: Default,
{
    fn maybe_clone(&self) -> Result<Vec<T, H>> {
        let mut vec = try!(Vec::with_capacity(self.len()));
        for i in 0..self.len() {
            vec.push(try!(self[i].maybe_clone()));
        }
        Ok(vec)
    }
}

impl<'a, T, H> IntoIterator for &'a Vec<T, H>
    where H: Allocator,
{
    type Item = &'a T;
    type IntoIter = slice::Items<'a, T>;
    fn into_iter(self) -> slice::Items<'a, T> { self.iter() }
}

impl<'a, T, H> IntoIterator for &'a mut Vec<T, H>
    where H: Allocator,
{
    type Item = &'a mut T;
    type IntoIter = slice::MutItems<'a, T>;
    fn into_iter(self) -> slice::MutItems<'a, T> { self.iter_mut() }
}

impl<T, H> AsRef<[T]> for Vec<T, H>
    where H: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.deref()
    }
}

impl<T, H> AsMut<[T]> for Vec<T, H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.deref_mut()
    }
}

impl<H> AsRef<ByteStr> for Vec<u8, H>
    where H: Allocator,
{
    fn as_ref(&self) -> &ByteStr {
        self.deref().as_ref()
    }
}

impl<H> AsMut<ByteStr> for Vec<u8, H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut ByteStr {
        self.deref_mut().as_mut()
    }
}

impl<H> AsCStr for Vec<u8, H>
    where H: Allocator,
{
    fn as_cstr(&self) -> Result<&CStr> {
        self.deref().as_cstr()
    }
}

impl<H> AsMutCStr for Vec<u8, H>
    where H: Allocator,
{
    fn as_mut_cstr(&mut self) -> Result<&mut CStr> {
        self.deref_mut().as_mut_cstr()
    }
}

impl<H> ToCStr for Vec<u8, H>
    where H: Allocator,
{
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.deref().to_cstr(buf)
    }

    fn to_or_as_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<&'a CStr> {
        self.deref().to_or_as_cstr(buf)
    }

    fn to_or_as_mut_cstr<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.deref_mut().to_or_as_mut_cstr(buf)
    }
}

impl<H> AsNoNullStr for Vec<u8, H>
    where H: Allocator,
{
    fn as_no_null_str(&self) -> Result<&NoNullStr> {
        self.deref().as_no_null_str()
    }
}

impl<H> AsMutNoNullStr for Vec<u8, H>
    where H: Allocator,
{
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> {
        self.deref_mut().as_mut_no_null_str()
    }
}

impl<H> Write for Vec<u8, H>
    where H: Allocator,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        try!(self.reserve(buf.len()));
        let len = self.len();
        unsafe { self.set_len(len + buf.len()); }
        mem::copy(&mut self[len..], buf);
        Ok(buf.len())
    }

    fn gather_write(&mut self, mut buf: &[&[u8]]) -> Result<usize> {
        let mut sum = 0;
        while buf.len() > 0 {
            sum += try!(self.write(&buf[0]));
            buf = &buf[1..];
        }
        Ok(sum)
    }
}

impl<H> Vec<u8, H>
    where H: Allocator,
{
    pub fn unused(&mut self) -> &mut [u8] {
        unsafe { slice::from_ptr(self.ptr.add(self.len), self.cap - self.len) }
    }
}

/// An iterator over the elements of a subrange of the vector.
pub struct Drainer<'a, T> {
    start: *const T,
    cur: *const T,
    end: *const T,
    vec_end: *const T,
    _data: PhantomData<(&'a (), T)>,
}

impl<'a, T> Iterator for Drainer<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.cur != self.end {
            unsafe {
                if mem::size_of::<T>() != 0 {
                    let t = ptr::read(self.cur);
                    self.cur = self.cur.add(1);
                    Some(t)
                } else {
                    let t = mem::unsafe_zeroed();
                    self.cur = (self.cur as usize + 1) as *const _;
                    Some(t)
                }
            }
        } else {
            None
        }
    }
}

impl<'a, T> Drop for Drainer<'a, T> {
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            unsafe {
                if mem::size_of::<T>() != 0 {
                    while self.cur != self.end {
                        ptr::drop(self.cur as *mut T);
                        self.cur = self.cur.add(1);
                    }
                } else {
                    drop(mem::unsafe_zeroed::<T>())
                }
            }
        }

        if mem::size_of::<T>() != 0 {
            let len = (self.vec_end as usize - self.end as usize) / mem::size_of::<T>();
            if len != 0 {
                unsafe { ptr::memmove(self.start as *mut _, self.end, len); }
            }
        }
    }
}
