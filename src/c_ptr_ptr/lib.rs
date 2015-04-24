// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_c_ptr_ptr"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_cty_base as cty_base;
extern crate linux_base as base;
extern crate linux_alloc as alloc;
extern crate linux_str_one as str_one;

#[prelude_import] use base::prelude::*;
use base::{error};
use cty_base::types::{c_char};
use core::{slice};
use str_one::{ToCStr};
use alloc::{Allocator};

mod linux { pub use base::linux::*; }

const USIZE_MASK: usize = core::num::usize::BYTES - 1;
const USIZE_BYTES: usize = core::num::usize::BYTES;

macro_rules! usize_align {
    ($val:expr) => { ($val + USIZE_MASK) & !USIZE_MASK }
}

pub type SCPtrPtr<Heap = alloc::Heap> = CPtrPtr<'static, Heap>;

pub struct CPtrPtr<'a, Heap = alloc::Heap>
    where Heap: Allocator,
{
    buf: *mut usize,
    pos: usize,
    cap: usize,
    num: usize,
    _marker: PhantomData<(&'a (), Heap)>,
}

impl<'a> CPtrPtr<'a, alloc::NoHeap> {
    pub fn buffered(buf: &'a mut [u8]) -> CPtrPtr<'a, alloc::NoHeap> {
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
            _marker: PhantomData,
        }
    }
}

impl<Heap> CPtrPtr<'static, Heap>
    where Heap: Allocator,
{
    pub fn new() -> Result<CPtrPtr<'static, Heap>> {
        const DEFAULT_CAP: usize = 32;
        Ok(CPtrPtr {
            buf: try!(unsafe { Heap::allocate_array(DEFAULT_CAP) }),
            pos: 0,
            cap: DEFAULT_CAP,
            num: 0,
            _marker: PhantomData,
        })
    }
}

impl<'a, Heap> CPtrPtr<'a, Heap>
    where Heap: Allocator,
{
    fn double(&mut self) -> Result {
        let new_cap = self.cap + self.cap / 2 + 1;
        self.buf = try!(unsafe { Heap::reallocate_array(self.buf, self.cap, new_cap) });
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

    pub fn push<S>(&mut self, s: S) -> Result
        where S: ToCStr,
    {
        self.pos += {
            let mut inc = None;
            while inc.is_none() {
                match self.push_int(&s) {
                    Ok(i) => inc = Some(i),
                    Err(error::NoMemory) => try!(self.double()),
                    Err(e) => return Err(e),
                }
            }
            inc.unwrap()
        };
        self.num += 1;
        Ok(())
    }

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

    pub fn truncate(&mut self) {
        self.pos = 0;
        self.num = 0;
    }
}
