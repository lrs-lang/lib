// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_c_ptr_ptr"]
#![crate_type = "lib"]
#![feature(no_std, const_fn)]
#![no_std]

extern crate lrs_cty_base as cty_base;
extern crate lrs_base as base;
extern crate lrs_alloc as alloc;
extern crate lrs_str_one as str_one;

use base::prelude::*;
use core::ptr::{OwnedPtr};
use base::{error};
use cty_base::types::{c_char};
use core::{slice, mem};
use alloc::{MemPool};
use str_one::{NoNullStr};

mod std { pub use base::std::*; }

const USIZE_MASK: usize = usize::bytes() - 1;
const USIZE_BYTES: usize = usize::bytes();

macro_rules! usize_align {
    ($val:expr) => { ($val + USIZE_MASK) & !USIZE_MASK }
}

pub type SCPtrPtr<Heap = alloc::Heap> = CPtrPtr<Heap>;

/// A helper type for creating `*const *const c_char` objects.
pub struct CPtrPtr<Heap = alloc::Heap>
    where Heap: MemPool,
{
    buf: OwnedPtr<usize>,
    pos: usize,
    cap: usize,
    num: usize,
    pool: Heap,
}

impl<H = alloc::Heap> CPtrPtr<H>
    where H: MemPool,
{
    /// Allocates a new `CPtrPtr`.
    pub fn new() -> Result<Self>
        where H: OutOf,
    {
        Self::with_pool(H::out_of(()))
    }

    /// Allocates a new `CPtrPtr`.
    pub fn with_pool(mut pool: H) -> Result<Self> {
        const DEFAULT_CAP: usize = 32;
        let buf = unsafe {
            OwnedPtr::new(try!(alloc::alloc_array(&mut pool, DEFAULT_CAP)).0)
        };
        Ok(CPtrPtr {
            buf: buf,
            pos: 0,
            cap: DEFAULT_CAP,
            num: 0,
            pool: pool,
        })
    }

    fn double(&mut self) -> Result {
        let new_cap = self.cap + self.cap / 2 + 1;
        self.buf = unsafe {
            OwnedPtr::new(try!(alloc::realloc_array(&mut self.pool, *self.buf, self.cap,
                                                    new_cap)).0)
        };
        self.cap = new_cap;
        Ok(())
    }

    fn slot(&mut self) -> Result<(&mut usize, &mut [u8])> {
        if self.cap == self.pos {
            try!(self.double());
        }
        unsafe {
            let usize_ptr = self.buf.add(self.pos);
            let slice_ptr = usize_ptr.add(1) as *mut u8;
            let slice = slice::from_ptr(slice_ptr,
                                        USIZE_BYTES * (self.cap - self.pos - 1));
            Ok((&mut *usize_ptr, slice))
        }
    }

    fn ptr_slice(&mut self) -> Result<(*const usize, &mut [*const c_char])> {
        if self.cap - self.pos <= self.num {
            try!(self.double());
        }
        assert!(self.cap - self.pos > self.num);
        unsafe {
            let slice_ptr = self.buf.add(self.pos) as *mut *const c_char;
            let slice = slice::from_ptr(slice_ptr, self.num + 1);
            Ok((*self.buf, slice))
        }
    }

    fn push_int<S: ?Sized>(&mut self, s: &S) -> Result<usize>
        where S: TryAsRef<NoNullStr>,
    {
        let cstr = try!(s.try_as_ref());
        let (next, buf) = try!(self.slot());
        if buf.len() < cstr.len() + 1 {
            return Err(error::NoMemory);
        }
        mem::copy(buf, cstr.as_ref());
        buf[cstr.len()] = 0;
        *next = 1 + usize_align!(cstr.len() + 1) / USIZE_BYTES;
        Ok(*next)
    }

    /// Adds a string to the `CPtrPtr`.
    ///
    /// [argument, s]
    /// The string to be added.
    pub fn push<S: ?Sized>(&mut self, s: &S) -> Result
        where S: TryAsRef<NoNullStr>,
    {
        loop {
            match self.push_int(s) {
                Ok(i) => {
                    self.pos += i;
                    self.num += 1;
                    return Ok(());
                },
                Err(error::NoMemory) => try!(self.double()),
                Err(e) => return Err(e),
            }
        }
    }

    /// Finishes the construction and returns the `*const *const c_char`.
    ///
    /// = Remarks
    ///
    /// The last element of the slice contains a null pointer. The returned value becomes
    /// invalid when another function of this object is called.
    pub fn finish(&mut self) -> Result<&mut [*const c_char]> {
        let (mut iter, mut slice) = try!(self.ptr_slice());
        unsafe {
            for i in 0..slice.len()-1 {
                slice[i] = iter.add(1) as *const c_char;
                iter = iter.add(*iter);
            }
            slice[slice.len()-1] = 0 as *const c_char;
            Ok(slice)
        }
    }

    /// Removes all strings from the `CPtrPtr`.
    pub fn clear(&mut self) {
        self.pos = 0;
        self.num = 0;
    }
}

impl<Heap> Drop for CPtrPtr<Heap>
    where Heap: MemPool,
{
    fn drop(&mut self) {
        unsafe {
            alloc::free_array(&mut self.pool, *self.buf, self.cap)
        }
    }
}
