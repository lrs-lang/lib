// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use core::{mem, ptr, cmp, slice};
use ty_one::clone::{Clone};
use core::ops::{Eq, Deref, DerefMut};
use core::iter::{IntoIterator};
use fmt::{Debug};
use io::{Write};
use alloc::{allocate_array, reallocate_array, free_array, empty_ptr};

use ty_one::{error};
use ty_one::rmo::{AsRef, AsMut};
use ty_one::byte_str::{ByteStr, AsByteStr, AsMutByteStr};
use ty_one::c_str::{CStr, AsCStr, AsMutCStr, ToCStr};
use ty_one::path::{Path, AsMutPath, AsPath};

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

    pub fn truncate(&mut self, len: usize) {
        assert!(len <= self.len);
        if mem::needs_drop::<T>() {
            for i in len..self.len {
                unsafe { ptr::read(self.ptr.add(i)); }
            }
        }
        self.len = len;
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

impl<T: Eq> Eq for Vec<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.deref().eq(other.deref())
    }
    fn ne(&self, other: &Vec<T>) -> bool {
        self.deref().ne(other.deref())
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
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.deref().fmt(w)
    }
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Result<Vec<T>> {
        let mut vec = try!(Vec::with_capacity(self.len()));
        for i in 0..self.len() {
            vec.push(try!(self[i].clone()));
        }
        Ok(vec)
    }
}

// Maybe these aren't really needed. We can just let the user manually deref.

impl AsRef<[u8]> for Vec<u8> {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}
impl AsMut<[u8]> for Vec<u8> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.deref_mut()
    }
}

impl AsByteStr for Vec<u8> {
    fn as_byte_str(&self) -> &ByteStr {
        self.deref().as_byte_str()
    }
}
impl AsMutByteStr for Vec<u8> {
    fn as_mut_byte_str(&mut self) -> &mut ByteStr {
        self.deref_mut().as_mut_byte_str()
    }
}

impl AsCStr for Vec<u8> {
    fn as_cstr(&self) -> Result<&CStr> {
        self.deref().as_cstr()
    }
}
impl AsMutCStr for Vec<u8> {
    fn as_mut_cstr(&mut self) -> Result<&mut CStr> {
        self.deref_mut().as_mut_cstr()
    }
}
impl ToCStr for Vec<u8> {
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

impl AsPath for Vec<u8> {
    fn as_path(&self) -> Result<&Path> {
        self.deref().as_path()
    }
}
impl AsMutPath for Vec<u8> {
    fn as_mut_path(&mut self) -> Result<&mut Path> {
        self.deref_mut().as_mut_path()
    }
}

impl Write for Vec<u8> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        try!(self.reserve(buf.len()));
        let len = self.len();
        unsafe { self.set_len(len + buf.len()); }
        mem::copy(&mut self[len..], buf);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<usize> {
        self.write(buf)
    }
}
