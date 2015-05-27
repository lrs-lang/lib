// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_c_ptr_ptr"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_cty_base as cty_base;
extern crate lrs_base as base;
extern crate lrs_alloc as alloc;
extern crate lrs_str_one as str_one;

#[prelude_import] use base::prelude::*;
use base::{error};
use base::default::{Default};
use cty_base::types::{c_char};
use core::{slice};
use str_one::{ToCStr};
use alloc::{Allocator};

mod lrs { pub use base::lrs::*; }

const USIZE_MASK: usize = usize::bytes() - 1;
const USIZE_BYTES: usize = usize::bytes();

macro_rules! usize_align {
    ($val:expr) => { ($val + USIZE_MASK) & !USIZE_MASK }
}

pub type SCPtrPtr<Heap = alloc::Heap> = CPtrPtr<Heap>;

/// A helper type for creating `*const *const c_char` objects.
pub struct CPtrPtr<Heap = alloc::Heap>
    where Heap: Allocator,
{
    buf: *mut usize,
    pos: usize,
    cap: usize,
    num: usize,
    pool: Heap::Pool,
}

impl<'a> CPtrPtr<alloc::NoMem<'a>> {
    /// Creates a new `CPtrPtr` from borrowed memory.
    ///
    /// [argument, buf]
    /// The buffer which backs the `CPtrPtr`.
    pub fn buffered(buf: &'a mut [u8]) -> CPtrPtr<alloc::NoMem<'a>> {
        let (ptr, cap) = if buf.len() < USIZE_BYTES {
            (buf.as_mut_ptr(), 0)
        } else {
            let original = buf.as_mut_ptr();
            let ptr = usize_align!(original as usize) as *mut u8;
            let cap = (buf.len() - (ptr as usize - original as usize)) / USIZE_BYTES;
            (ptr, cap)
        };
        CPtrPtr {
            buf: ptr as *mut usize,
            pos: 0,
            cap: cap,
            num: 0,
            pool: (),
        }
    }
}

impl<Heap> CPtrPtr<Heap>
    where Heap: Allocator,
          Heap::Pool: Default,
{
    /// Allocates a new `CPtrPtr`.
    pub fn new() -> Result<CPtrPtr<Heap>> {
        const DEFAULT_CAP: usize = 32;
        let mut pool = Heap::Pool::default();
        Ok(CPtrPtr {
            buf: try!(unsafe { Heap::allocate_array(&mut pool, DEFAULT_CAP) }),
            pos: 0,
            cap: DEFAULT_CAP,
            num: 0,
            pool: pool,
        })
    }
}

impl<Heap> CPtrPtr<Heap>
    where Heap: Allocator,
{
    fn double(&mut self) -> Result {
        let new_cap = self.cap + self.cap / 2 + 1;
        self.buf = try!(unsafe {
            Heap::reallocate_array(&mut self.pool, self.buf, self.cap, new_cap)
        });
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
            Ok((self.buf, slice))
        }
    }

    fn push_int<S>(&mut self, s: S) -> Result<usize>
        where S: ToCStr,
    {
        let (next, buf) = try!(self.slot());
        let buf_addr = buf.as_ptr() as usize;
        let cstr = try!(s.to_cstr(buf));
        if cstr.as_ptr() as usize != buf_addr {
            return Err(error::InvalidArgument);
        }
        *next = 1 + usize_align!(cstr.len() + 1) / USIZE_BYTES;
        Ok(*next)
    }

    /// Adds a string to the `CPtrPtr`.
    ///
    /// [argument, s]
    /// The string to be added.
    pub fn push<S>(&mut self, s: S) -> Result
        where S: ToCStr,
    {
        loop {
            match self.push_int(&s) {
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
    where Heap: Allocator,
{
    fn drop(&mut self) {
        unsafe {
            Heap::free_array(&mut self.pool, self.buf, self.cap)
        }
    }
}
