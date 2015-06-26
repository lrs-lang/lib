// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_ringbuf"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits, associated_consts)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_wrapping as wrapping;
extern crate lrs_fmt as fmt;
extern crate lrs_alloc as alloc;

mod lrs { pub use fmt::lrs::*; }

#[prelude_import] use base::prelude::*;
use core::{mem, ptr};
use core::ptr::{OwnedPtr};
use wrapping::{Wsize};
use base::clone::{MaybeClone};
use base::undef::{UndefState};
use base::{error};
use base::default::{Default};
use core::ops::{Eq, Index, IndexMut};
use core::iter::{IntoIterator};
use fmt::{Debug, Write};
use alloc::{Allocator, empty_ptr};

/// A resizable ring buffer.
pub struct DynRingBuf<T, Heap = alloc::Heap>
    where Heap: Allocator,
{
    ptr: OwnedPtr<T>,
    left: Wsize,
    right: Wsize,
    cap: usize,
    pool: Heap::Pool,
}

impl<'a, T> DynRingBuf<T, alloc::NoMem<'a>> {
    /// Creates a ring buffer which is backed by borrowed memory.
    ///
    /// [argument, buf]
    /// The buffer which will be used to store elements it.
    pub fn buffered(buf: &'a mut [u8]) -> Self {
        if mem::size_of::<T>() == 0 {
            return DynRingBuf {
                ptr: unsafe { OwnedPtr::new(empty_ptr()) },
                left: Wsize(0),
                right: Wsize(0),
                cap: 0,
                pool: (),
            };
        }

        let buf = mem::align_for_mut::<T>(buf);
        let cap = {
            let tmp = buf.len() / mem::size_of::<T>();
            let npot = tmp.next_power_of_two();
            if npot != tmp {
                npot >> 1
            } else {
                tmp
            }
        };
        DynRingBuf {
            ptr: unsafe { OwnedPtr::new(buf.as_mut_ptr() as *mut T) },
            left: Wsize(0),
            right: Wsize(0),
            cap: cap,
            pool: (),
        }
    }
}

impl<T, H> DynRingBuf<T, H>
    where H: Allocator,
{
    /// Creates a new ring buffer.
    pub fn new() -> Self
        where H::Pool: Default,
    {
        DynRingBuf {
            ptr: unsafe { OwnedPtr::new(empty_ptr()) },
            left: Wsize(0),
            right: Wsize(0),
            cap: if mem::size_of::<T>() == 0 { !0 >> 1 } else { 0 },
            pool: H::Pool::default(),
        }
    }

    /// Creates a new ring buffer and reserves a certain amount of space for it.
    pub fn with_capacity(mut cap: usize) -> Result<Self>
        where H::Pool: Default,
    {
        let mut pool = H::Pool::default();
        let size = mem::size_of::<T>();
        if cap == 0 || size == 0 {
            return Ok(DynRingBuf {
                ptr: unsafe { OwnedPtr::new(empty_ptr()) },
                left: Wsize(0),
                right: Wsize(0),
                cap: if size == 0 { !0 >> 1 } else { 0 },
                pool: pool
            });
        }
        cap = cap.checked_next_power_of_two().unwrap_or(!0);
        let ptr = unsafe { OwnedPtr::new(try!(H::allocate_array(&mut pool, cap))) };
        Ok(DynRingBuf {
            ptr: ptr,
            left: Wsize(0),
            right: Wsize(0),
            cap: cap,
            pool: pool
        })
    }

    /// Creates a new ring buffer with a memory pool.
    ///
    /// [argument, pool]
    /// The pool to draw memory from.
    pub fn with_pool(pool: H::Pool) -> Self {
        DynRingBuf {
            ptr: unsafe { OwnedPtr::new(empty_ptr()) },
            left: Wsize(0),
            right: Wsize(0),
            cap: if mem::size_of::<T>() == 0 { !0 >> 1 } else { 0 },
            pool: pool,
        }
    }

    /// Returns the capacity of the ring buffer.
    pub fn capacity(&self) -> usize {
        self.cap
    }

    fn cap_mask(&self) -> usize {
        let res = self.cap.wrapping_sub(1);
        debug_assert!(self.cap & res == 0);
        res
    }

    /// Returns the number of available but unused slots.
    pub fn available(&self) -> usize {
        self.cap - self.len()
    }

    /// Returns the number of elements in the ring buffer.
    pub fn len(&self) -> usize {
        (self.right - self.left).0
    }

    /// Reserves memory for additional elements.
    ///
    /// [argument, n]
    /// The number of elements for which memory should be reserved.
    pub fn reserve(&mut self, n: usize) -> Result {
        if self.available() >= n {
            return Ok(());
        }
        if mem::size_of::<T>() == 0 {
            return Err(error::NoMemory);
        }

        // new_cap >= 2*self.cap
        let new_cap = self.len().checked_add(n).unwrap_or(!0)
                                .checked_next_power_of_two().unwrap_or(!0);

        let ptr = if *self.ptr == empty_ptr() {
            unsafe { H::allocate_array(&mut self.pool, new_cap) }
        } else {
            unsafe { H::reallocate_array(&mut self.pool, *self.ptr, self.cap, new_cap) }
        };
        self.ptr = unsafe { OwnedPtr::new(try!(ptr)) };

        let len = self.len();
        self.left = self.left & self.cap_mask();
        self.right = self.right & self.cap_mask();
        if len > 0 && self.right <= self.left {
            unsafe { ptr::memcpy(self.ptr.add(self.cap), *self.ptr, self.right.0); }
            self.right = self.right + self.capacity();
        }

        self.cap = new_cap;
        Ok(())
    }

    unsafe fn push_right_inner(&mut self, val: T) {
        let idx = self.right & self.cap_mask();
        ptr::write(self.ptr.add(idx.0), val);
        self.right = self.right + 1;
    }

    /// Appends an element to the right end of the ring buffer.
    ///
    /// [argument, val]
    /// The element to append.
    ///
    /// = Remarks
    ///
    /// This method aborts the process if no memory is available and allocating additional
    /// memory fails. To avoid this, use `reserve` or `try_push_right`.
    pub fn push_right(&mut self, val: T) {
        self.reserve(1).unwrap();
        unsafe { self.push_right_inner(val); }
    }

    /// Tries to append an element to the right end of the ring buffer.
    ///
    /// [argument, val]
    /// The element to append.
    pub fn try_push_right(&mut self, val: T) -> Result
        where T: Copy
    {
        try!(self.reserve(1));
        unsafe { self.push_right_inner(val); }
        Ok(())
    }

    /// Removes an element from the rght end of the ring buffer.
    pub fn pop_right(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            _ => {
                self.right = self.right - 1;
                let idx = self.right & self.cap_mask();
                unsafe { Some(ptr::read(self.ptr.add(idx.0))) }
            },
        }
    }

    unsafe fn push_left_inner(&mut self, val: T) {
        self.left = self.left - 1;
        let idx = self.left & self.cap_mask();
        ptr::write(self.ptr.add(idx.0), val);
    }

    /// Appends an element to the left end of the ring buffer.
    ///
    /// [argument, val]
    /// The element to append.
    ///
    /// = Remarks
    ///
    /// This method aborts the process if no memory is available and allocating additional
    /// memory fails. To avoid this, use `reserve` or `try_push_right`.
    pub fn push_left(&mut self, val: T) {
        self.reserve(1).unwrap();
        unsafe { self.push_left_inner(val); }
    }

    /// Tries to append an element to the right end of the ring buffer.
    ///
    /// [argument, val]
    /// The element to append.
    pub fn try_push_left(&mut self, val: T) -> Result
        where T: Copy
    {
        try!(self.reserve(1));
        unsafe { self.push_left_inner(val); }
        Ok(())
    }

    /// Removes an element from the left end of the ring buffer.
    pub fn pop_left(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            _ => {
                let idx = self.left & self.cap_mask();
                self.left = self.left + 1;
                unsafe { Some(ptr::read(self.ptr.add(idx.0))) }
            },
        }
    }

    /// Creates an iterator over the elements in the ringbuffer.
    pub fn iter<'a>(&'a self) -> RingBufIter<'a, T> {
        RingBufIter {
            ptr: *self.ptr,
            cap_mask: self.cap - 1,
            left: self.left,
            right: self.right,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, H> IntoIterator for &'a DynRingBuf<T, H>
    where H: Allocator,
{
    type Item = &'a T;
    type IntoIter = RingBufIter<'a, T>;
    fn into_iter(self) -> RingBufIter<'a, T> { self.iter() }
}

unsafe impl<T, H> UndefState for DynRingBuf<T, H>
    where H: Allocator,
{
    fn num() -> usize { <OwnedPtr<T> as UndefState>::num() }

    unsafe fn set_undef(val: *mut DynRingBuf<T, H>, n: usize) {
        <OwnedPtr<T> as UndefState>::set_undef(&mut (*val).ptr, n);
    }

    unsafe fn is_undef(val: *const DynRingBuf<T, H>, n: usize) -> bool {
        <OwnedPtr<T> as UndefState>::is_undef(&(*val).ptr, n)
    }
}

unsafe impl<T, H> Sync for DynRingBuf<T, H> where T: Sync, H: Allocator, { }
unsafe impl<T, H> Send for DynRingBuf<T, H> where T: Send, H: Allocator+Send { }

impl<T, H> Drop for DynRingBuf<T, H>
    where H: Allocator,
{
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                let mut i = self.left;
                while i != self.right {
                    ptr::drop(self.ptr.add(i.0 & self.cap_mask()));
                    i = i + 1;
                }
            }
            if *self.ptr != empty_ptr() {
                H::free_array(&mut self.pool, *self.ptr, self.cap);
            }
        }
    }
}

impl<T, H> Index<usize> for DynRingBuf<T, H>
    where H: Allocator,
{
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        assert!(idx < self.len());
        let idx = (self.left + idx).0 & self.cap_mask();
        unsafe { &*(self.ptr.add(idx)) }
    }
}

impl<T, H> IndexMut<usize> for DynRingBuf<T, H>
    where H: Allocator,
{
    fn index_mut(&mut self, idx: usize) -> &mut T {
        assert!(idx < self.len());
        let idx = (self.left + idx).0 & self.cap_mask();
        unsafe { &mut *(self.ptr.add(idx)) }
    }
}

impl<T, H1, H2> Eq<DynRingBuf<T, H1>> for DynRingBuf<T, H2>
    where T: Eq,
          H1: Allocator,
          H2: Allocator,
{
    fn eq(&self, other: &DynRingBuf<T, H1>) -> bool {
        if self.ptr == other.ptr {
            return true;
        }
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }

    fn ne(&self, other: &DynRingBuf<T, H1>) -> bool {
        !self.eq(other)
    }
}

impl<T, H> Eq<[T]> for DynRingBuf<T, H>
    where T: Eq,
          H: Allocator,
{
    fn eq(&self, other: &[T]) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }

    fn ne(&self, other: &[T]) -> bool {
        !self.eq(other)
    }
}

impl<T, H> Debug for DynRingBuf<T, H>
    where T: Debug,
          H: Allocator,
{
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        try!(write!(w, "["));
        if self.len() > 0 {
            let mut iter = self.iter();
            iter.right = iter.right - 1;
            for el in iter {
                try!(write!(w, "{:?}, ", el));
            }
            try!(write!(w, "{:?}", &self[self.len() - 1]));
        }
        write!(w, "]");
        Ok(())
    }
}

impl<T, H> MaybeClone for DynRingBuf<T, H>
    where T: MaybeClone,
          H: Allocator,
          H::Pool: Default,
{
    fn maybe_clone(&self) -> Result<DynRingBuf<T, H>> {
        let mut vec = try!(DynRingBuf::with_capacity(self.len()));
        for i in 0..self.len() {
            vec.push_right(try!(self[i].maybe_clone()));
        }
        Ok(vec)
    }
}

/// An iterator over the elements in a ringbuffer.
pub struct RingBufIter<'a, T> {
    ptr: *const T,
    cap_mask: usize,
    left: Wsize,
    right: Wsize,
    _marker: PhantomData<&'a ()>,
}

impl<'a, T> Iterator for RingBufIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.left == self.right {
            None
        } else {
            self.left = self.left + 1;
            let idx = (self.left - 1).0 & self.cap_mask;
            unsafe { Some(&*(self.ptr.add(idx))) }
        }
    }
}
