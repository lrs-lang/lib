// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_buf_reader"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_arch_fns as arch_fns;
extern crate linux_base as base;
extern crate linux_io as io;
extern crate linux_alloc as alloc;

#[prelude_import] use base::prelude::*;
use core::{num, slice};
use base::{error};
use alloc::{allocate_array, free_array};
use io::{Read, BufRead, Write};
use arch_fns::{memchr};

pub mod linux { pub use base::linux::*; }

pub struct BufReader<'a, R: Read> {
    data: *mut u8,
    cap: usize,
    start: usize,
    end: usize,
    read: R,
    allocated: bool,
    _marker: PhantomData<&'a ()>,
}

impl<'a, R: Read> BufReader<'a, R> {
    pub fn allocate(read: R, size: usize) -> Result<BufReader<'static, R>> {
        let size = match size.checked_next_power_of_two() {
            Some(n) => n,
            _ => return Err(error::NoMemory),
        };
        let ptr = unsafe { allocate_array(size) };
        if ptr.is_null() {
            return Err(error::NoMemory);
        }
        Ok(BufReader {
            data: ptr,
            cap: size,
            start: 0,
            end: 0,
            read: read,
            allocated: true,
            _marker: PhantomData,
        })
    }

    pub fn new(read: R, buf: &'a mut [u8]) -> BufReader<'a, R> {
        let size = match buf.len() {
            0 => 0,
            n => 1 << (num::usize::BITS - n.leading_zeros() - 1),
        };
        BufReader {
            data: buf.as_mut_ptr(),
            cap: size,
            start: 0,
            end: 0,
            read: read,
            allocated: false,
            _marker: PhantomData,
        }
    }

    pub fn available(&self) -> usize {
        self.end.wrapping_sub(self.start)
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn fill(&mut self) -> Result<usize> {
        if self.available() == self.cap {
            return Ok(0);
        }
        let res = {
            let (mut read, mut slices) = self.write_slices();
            try!(read.scatter_read(&mut slices))
        };
        self.end = self.end.wrapping_add(res);
        Ok(res)
    }

    fn read_slices(&mut self) -> (&mut usize, [&[u8]; 2]) {
        let start = self.start % self.cap;
        let end = self.end % self.cap;
        let slices: [&[u8]; 2] = unsafe {
            if start <= end {
                [slice::from_ptr(self.data.add(start), end - start),
                 &[][..]]
            } else {
                [slice::from_ptr(self.data.add(start), self.cap - start),
                 slice::from_ptr(self.data, end)]
            }
        };
        (&mut self.start, slices)
    }

    fn write_slices(&mut self) -> (&mut R, [&mut [u8]; 2]) {
        let start = self.start % self.cap;
        let end = self.end % self.cap;
        let slices = unsafe {
            if start <= end {
                [slice::from_ptr(self.data.add(end), self.cap - end),
                 slice::from_ptr(self.data, start)]
            } else {
                [slice::from_ptr(self.data.add(end), start - end),
                 &mut [][..]]
            }
        };
        (&mut self.read, slices)
    }
}

impl<'a, R: Read> Read for BufReader<'a, R> {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        if self.available() == 0 {
            self.start = 0;
            self.end = 0;
            if buf.len() >= self.cap {
                return self.read.read(buf);
            }
            if try!(self.fill()) == 0 {
                return Ok(0);
            }
        }
        let res = buf.gather_write(&self.read_slices().1).unwrap();
        self.start = self.start.wrapping_add(res);
        Ok(res)
    }

    fn scatter_read(&mut self, mut buf: &mut [&mut [u8]]) -> Result<usize> {
        if self.available() == 0 {
            self.start = 0;
            self.end = 0;
            try!(self.fill());
        }
        let mut sum = 0;
        while self.available() > 0 && buf.len() > 0 {
            sum += self.read(&mut buf[0]).unwrap();
            let b = buf;
            buf = &mut b[1..];
        }
        Ok(sum)
    }
}

impl<'a, R: Read> Drop for BufReader<'a, R> {
    fn drop(&mut self) {
        if self.allocated {
            unsafe { free_array(self.data, self.cap); }
        }
    }
}

impl<'a, R: Read> BufRead for BufReader<'a, R> {
    fn copy_until<W: Write>(&mut self, dst: &mut W, b: u8) -> Result<usize> {
        let mut len = 0;
        if self.available() == 0 {
            self.start = 0;
            self.end = 0;
            if try!(self.fill()) == 0 {
                return Ok(0);
            }
        }

        let mut done = false;

        'outer: loop {
            {
                let (start, mut bufs) = self.read_slices();

                for i in 0..2usize {
                    if let Some(pos) = memchr(&bufs[i], b) {
                        bufs[i] = &bufs[i][..pos+1];
                        done = true;
                    }

                    len += bufs[i].len();
                    while bufs[i].len() > 0 {
                        let res = try!(dst.write(&bufs[i]));
                        *start = start.wrapping_add(res);
                        bufs[i] = &bufs[i][res..];
                    }

                    if done {
                        break 'outer;
                    }
                }
            }

            if try!(self.fill()) == 0 {
                break;
            }
        }

        Ok(len)
    }
}
