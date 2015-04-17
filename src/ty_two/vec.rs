// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::{mem, ptr, cmp, slice};
use core::ops::{Deref, DerefMut};
use core::iter::{IntoIterator};
use fmt::{Debug};
use io::{Write};
use ty_one::{error};
use alloc::{allocate_array, reallocate_array, free_array, empty_ptr};

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T> {
        Vec { ptr: empty_ptr(), len: 0, cap: 0 }
    }

    pub fn with_capacity(cap: usize) -> Result<Vec<T>> {
        if cap == 0 || mem::size_of::<T>() == 0 {
            return Ok(Vec { ptr: empty_ptr(), len: 0, cap: cap });
        }
        let ptr = unsafe { allocate_array(cap) };
        if ptr.is_null() {
            return Err(error::NoMemory);
        }
        Ok(Vec { ptr: ptr, len: 0, cap: cap })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn reserve(&mut self, n: usize) -> Result {
        if self.cap - self.len >= n {
            return Ok(());
        }
        if mem::size_of::<T>() == 0 {
            self.cap = self.len + n;
            return Ok(());
        }

        let new_cap = self.len + cmp::max(n, self.cap / 2 + 1);
        let ptr = if self.ptr == empty_ptr() {
            unsafe { allocate_array(new_cap) }
        } else {
            unsafe { reallocate_array(self.ptr, self.cap, new_cap) }
        };
        if ptr.is_null() {
            Err(error::NoMemory)
        } else {
            self.ptr = ptr;
            self.cap = new_cap;
            Ok(())
        }
    }

    pub fn push(&mut self, val: T) {
        if self.cap == self.len {
            self.reserve(1).unwrap();
        }
        unsafe { ptr::write(self.ptr.add(self.len), val); }
        self.len += 1;
    }

    pub fn push_all(&mut self, vals: &[T]) where T: Copy {
        unsafe { self.unsafe_push_all(vals); }
    }

    pub unsafe fn unsafe_push_all(&mut self, vals: &[T]) {
        self.try_unsafe_push_all(vals).unwrap();
    }

    pub fn try_push_all(&mut self, vals: &[T]) -> Result where T: Copy {
        unsafe { self.try_unsafe_push_all(vals) }
    }

    pub unsafe fn try_unsafe_push_all(&mut self, vals: &[T]) -> Result {
        try!(self.reserve(vals.len()));
        let tail = slice::from_ptr(self.ptr.add(self.len), vals.len());
        mem::unsafe_copy(tail, vals);
        self.len += vals.len();
        Ok(())
    }

    pub fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            _ => {
                self.len -= 1;
                unsafe { Some(ptr::read(self.ptr.add(self.len))) }
            },
        }
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                for i in 0..self.len {
                    ptr::read(self.ptr.add(i));
                }
            }
            if self.ptr != empty_ptr() {
                free_array(self.ptr, self.cap);
            }
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl<T: Debug> Debug for Vec<T> {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        self.deref().fmt(w)
    }
}
