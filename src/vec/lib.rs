// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_vec"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_str_one as str_one;
extern crate lrs_io as io;
extern crate lrs_fmt as fmt;
extern crate lrs_alloc as alloc;

pub mod lrs {
    pub use fmt::lrs::*;
    pub mod vec { pub use {Vec}; }
}

#[prelude_import] use base::prelude::*;
use core::{mem, ptr, cmp, slice};
use base::clone::{Clone};
use core::ops::{Eq, Deref, DerefMut};
use core::iter::{IntoIterator};
use io::{Read, Write};
use fmt::{Debug};
use alloc::{Allocator, empty_ptr};
use base::rmo::{AsRef, AsMut};
use str_one::c_str::{CStr, AsCStr, AsMutCStr, ToCStr};
use str_one::no_null_str::{NoNullStr, AsMutNoNullStr, AsNoNullStr};

pub type SVec<T, Heap = alloc::Heap> = Vec<'static, T, Heap>;

pub struct Vec<'a, T, Heap = alloc::Heap>
    where Heap: Allocator,
{
    ptr: *mut T,
    len: usize,
    cap: usize,
    _marker: PhantomData<(&'a (), Heap)>,
}

impl<'a, T> Vec<'a, T, alloc::NoMem> {
    pub fn buffered(buf: &'a mut [u8]) -> Vec<'a, T, alloc::NoMem> {
        if mem::size_of::<T>() == 0 {
            return Vec { ptr: empty_ptr(), len: 0, cap: 0, _marker: PhantomData };
        }

        let align_mask = mem::align_of::<T>() - 1;
        let mut ptr = buf.as_mut_ptr() as usize;
        let mut len = buf.len();
        if ptr & align_mask != 0 {
            let diff = (!ptr & align_mask) + 1;
            if diff > len {
                return Vec { ptr: empty_ptr(), len: 0, cap: 0, _marker: PhantomData };
            }
            ptr += diff;
            len -= diff;
        }
        let cap = len / mem::size_of::<T>();
        Vec { ptr: ptr as *mut T, len: 0, cap: cap, _marker: PhantomData }
    }
}

impl<T, H> Vec<'static, T, H>
    where H: Allocator,
{
    pub fn new() -> SVec<T, H> {
        Vec { ptr: empty_ptr(), len: 0, cap: 0, _marker: PhantomData, }
    }

    pub fn with_capacity(cap: usize) -> Result<SVec<T, H>> {
        if cap == 0 || mem::size_of::<T>() == 0 {
            return Ok(Vec { ptr: empty_ptr(), len: 0, cap: cap, _marker: PhantomData });
        }
        let ptr = unsafe { try!(H::allocate_array(cap)) };
        Ok(Vec { ptr: ptr, len: 0, cap: cap, _marker: PhantomData })
    }
}

impl<'a, T, H> Vec<'a, T, H>
    where H: Allocator,
{
    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn available(&self) -> usize {
        self.cap - self.len
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
            unsafe { H::allocate_array(new_cap) }
        } else {
            unsafe { H::reallocate_array(self.ptr, self.cap, new_cap) }
        };
        self.ptr = try!(ptr);
        self.cap = new_cap;
        Ok(())
    }

    pub fn push(&mut self, val: T) {
        if self.cap == self.len {
            self.reserve(1).unwrap();
        }
        unsafe { ptr::write(self.ptr.add(self.len), val); }
        self.len += 1;
    }

    pub fn try_push(&mut self, val: T) -> Result where T: Copy {
        if self.cap == self.len {
            try!(self.reserve(1));
        }
        unsafe { ptr::write(self.ptr.add(self.len), val); }
        self.len += 1;
        Ok(())
    }

    pub fn push_all(&mut self, vals: &[T]) -> Result where T: Copy {
        unsafe { self.try_unsafe_push_all(vals) }
    }

    pub unsafe fn unsafe_push_all(&mut self, vals: &[T]) {
        self.try_unsafe_push_all(vals).unwrap();
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
        assert!(len <= self.cap);
        self.len = len;
    }
}

impl<'a, H> Vec<'a, u8, H>
    where H: Allocator,
{
    pub fn read_to_eof<R>(&mut self, mut r: R) -> Result<usize>
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

unsafe impl<'a, T, H> Sync for Vec<'a, T, H> where T: Sync, H: Allocator, { }
unsafe impl<'a, T, H> Send for Vec<'a, T, H> where T: Send, H: Allocator+Send { }

impl<'a, T, H> Drop for Vec<'a, T, H>
    where H: Allocator,
{
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                for i in 0..self.len {
                    ptr::read(self.ptr.add(i));
                }
            }
            if self.ptr != empty_ptr() {
                H::free_array(self.ptr, self.cap);
            }
        }
    }
}

impl<'a, T, H1, H2> Eq<Vec<'a, T, H1>> for Vec<'a, T, H2>
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

impl<'a, T, H> Eq<[T]> for Vec<'a, T, H>
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

impl<'a, T, H> Deref for Vec<'a, T, H>
    where H: Allocator,
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl<'a, T, H> DerefMut for Vec<'a, T, H>
    where H: Allocator,
{
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl<'a, T, H> Debug for Vec<'a, T, H>
    where T: Debug,
          H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.deref().fmt(w)
    }
}

impl<T, H> Clone for SVec<T, H>
    where T: Clone,
          H: Allocator,
{
    fn clone(&self) -> Result<SVec<T, H>> {
        let mut vec = try!(Vec::with_capacity(self.len()));
        for i in 0..self.len() {
            vec.push(try!(self[i].clone()));
        }
        Ok(vec)
    }
}

impl<'a, T, H> IntoIterator for &'a Vec<'a, T, H>
    where H: Allocator,
{
    type Item = &'a T;
    type IntoIter = slice::Items<'a, T>;
    fn into_iter(self) -> slice::Items<'a, T> { self.iter() }
}

// Maybe these aren't really needed. We can just let the user manually deref.

impl<'a, H> AsRef<[u8]> for Vec<'a, u8, H>
    where H: Allocator,
{
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl<'a, H> AsMut<[u8]> for Vec<'a, u8, H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut [u8] {
        self.deref_mut()
    }
}

impl<'a, H> AsCStr for Vec<'a, u8, H>
    where H: Allocator,
{
    fn as_cstr(&self) -> Result<&CStr> {
        self.deref().as_cstr()
    }
}

impl<'a, H> AsMutCStr for Vec<'a, u8, H>
    where H: Allocator,
{
    fn as_mut_cstr(&mut self) -> Result<&mut CStr> {
        self.deref_mut().as_mut_cstr()
    }
}

impl<'b, H> ToCStr for Vec<'b, u8, H>
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

impl<'a, H> AsNoNullStr for Vec<'a, u8, H>
    where H: Allocator,
{
    fn as_no_null_str(&self) -> Result<&NoNullStr> {
        self.deref().as_no_null_str()
    }
}

impl<'a, H> AsMutNoNullStr for Vec<'a, u8, H>
    where H: Allocator,
{
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> {
        self.deref_mut().as_mut_no_null_str()
    }
}

impl<'a, H> Write for Vec<'a, u8, H>
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

impl<'a, H> Vec<'a, u8, H>
    where H: Allocator,
{
    pub fn unused(&mut self) -> &mut [u8] {
        unsafe { slice::from_ptr(self.ptr.add(self.len), self.cap - self.len) }
    }
}
