// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_io"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

extern crate linux_core as core;
extern crate linux_ty_one as ty_one;

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::{mem};
use ty_one::error::{Errno};

pub type Result<T> = ty_one::result::Result<T, Errno>;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn read_all(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        let mut read = 0;
        while buf.len() > 0 {
            match self.read(buf) {
                e @ Err(_) => return e,
                Ok(0) => break,
                Ok(n) => {
                    read += n;
                    buf = &mut {buf}[n..];
                }
            }
        }
        Ok(read)
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<usize> {
        let mut written = 0;
        while buf.len() > 0 {
            match self.write(buf) {
                e @ Err(_) => return e,
                Ok(0) => return Ok(0),
                Ok(n) => {
                    written += n;
                    buf = &buf[n..];
                }
            }
        }
        Ok(written)
    }
}

impl<'a> Read for &'a [u8] {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = mem::copy(buf, *self);
        *self = &self[n..];
        Ok(n)
    }
}

impl<'a> Write for &'a mut [u8] {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = mem::copy(*self, buf);
        unsafe {
            // Compiler bug.
            let slf: &mut &'static mut [u8] = mem::cast::<&mut &mut [u8], _>(self);
            *slf = &mut slf[n..];
        }
        Ok(n)
    }
}

impl<'a, T: Read+?Sized> Read for &'a mut T {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        (*self).read(buf)
    }
}

impl<'a, T: Write+?Sized> Write for &'a mut T {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        (*self).write(buf)
    }
}
