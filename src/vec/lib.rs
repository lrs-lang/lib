// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_vec"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_str_one as str_one;
extern crate linux_io as io;
extern crate linux_fmt as fmt;
extern crate linux_alloc as alloc;

pub mod linux {
    pub use fmt::linux::*;
    pub mod vec { pub use {Vec}; }
}

#[prelude_import] use base::prelude::*;
use core::{mem, ptr, cmp, slice};
use base::clone::{Clone};
use core::ops::{Eq, Deref, DerefMut};
use core::iter::{IntoIterator};
use io::{Read, Write};
use fmt::{Debug};
use alloc::{allocate_array, reallocate_array, free_array, empty_ptr};
use base::{error};
use base::rmo::{AsRef, AsMut};
use str_one::byte_str::{ByteStr, AsByteStr, AsMutByteStr};
use str_one::c_str::{CStr, AsCStr, AsMutCStr, ToCStr};
use str_one::no_null_str::{NoNullStr, AsMutNoNullStr, AsNoNullStr};

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

impl Vec<u8> {
    pub fn read_to_eof<R: Read>(&mut self, r: &mut R) -> Result<usize> {
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

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = slice::Items<'a, T>;
    fn into_iter(self) -> slice::Items<'a, T> { self.iter() }
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

impl AsNoNullStr for Vec<u8> {
    fn as_no_null_str(&self) -> Result<&NoNullStr> {
        self.deref().as_no_null_str()
    }
}
impl AsMutNoNullStr for Vec<u8> {
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> {
        self.deref_mut().as_mut_no_null_str()
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

    fn gather_write(&mut self, mut buf: &[&[u8]]) -> Result<usize> {
        let mut sum = 0;
        while self.len() > 0 && buf.len() > 0 {
            sum += try!(self.write(&buf[0]));
            buf = &buf[1..];
        }
        Ok(sum)
    }
}
