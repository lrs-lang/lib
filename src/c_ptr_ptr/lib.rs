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
extern crate linux_str_one as str_one;

#[prelude_import] use base::prelude::*;
use base::{error};
use cty_base::types::{c_char};
use core::{slice};
use str_one::{ToCStr};

mod linux { pub use base::linux::*; }

const USIZE_MASK: usize = core::num::usize::BYTES - 1;
const USIZE_BYTES: usize = core::num::usize::BYTES;

macro_rules! usize_align {
    ($val:expr) => { ($val + USIZE_MASK) & !USIZE_MASK }
}

pub struct CPtrPtr<'a> {
    buf: *mut usize,
    pos: usize,
    cap: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> CPtrPtr<'a> {
    pub fn new(buf: &'a mut [u8]) -> CPtrPtr<'a> {
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
            _marker: PhantomData,
        }
    }

    fn slot(&mut self) -> Result<(&mut usize, &mut [u8])> {
        unsafe {
            if self.pos >= self.cap {
                return Err(error::NoMemory);
            }
            let usize_ptr = self.buf.add(self.pos);
            let slice_ptr = usize_ptr.add(1) as *mut u8;
            let slice = slice::from_ptr(slice_ptr,
                                        USIZE_BYTES * (self.cap - self.pos - 1));
            Ok((&mut *usize_ptr, slice))
        }
    }

    fn ptr_ptr_slice(&mut self) -> Result<(*const usize, *const usize, &mut [usize])> {
        unsafe {
            let len = match self.cap - self.pos {
                0 => return Err(error::NoMemory),
                n => n,
            };
            let slice_ptr = self.buf.add(self.pos);
            let slice = slice::from_ptr(slice_ptr, len);
            Ok((self.buf as *const usize, slice_ptr as *const usize, slice))
        }
    }

    pub fn push<S>(&mut self, s: S) -> Result
        where S: ToCStr,
    {
        self.pos += {
            let (next, buf) = try!(self.slot());
            let buf_addr = buf.as_ptr() as usize;
            let cstr = try!(s.to_cstr(buf));
            if cstr.as_ptr() as usize != buf_addr {
                return Err(error::InvalidArgument);
            }
            *next = 1 + usize_align!(cstr.len() + 1) / USIZE_BYTES;
            *next
        };
        Ok(())
    }

    pub fn finish(&mut self) -> Result<&*const c_char> {
        let (mut iter, slice_ptr, mut slice) = try!(self.ptr_ptr_slice());
        unsafe {
            while iter != slice_ptr {
                if slice.len() == 0 {
                    return Err(error::NoMemory);
                }
                slice[0] = iter.add(1) as usize;
                let tmp = slice;
                slice = &mut tmp[1..];
                iter = iter.add(*iter);
            }
            if slice.len() == 0 {
                return Err(error::NoMemory);
            }
            slice[0] = 0;
            Ok(&*(slice_ptr as *const *const c_char))
        }
    }
}
