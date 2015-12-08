// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_buf_reader"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_arch_fns as arch_fns;
extern crate lrs_base as base;
extern crate lrs_io as io;
extern crate lrs_alloc as alloc;

use base::prelude::*;
use core::{slice, cmp};
use core::ptr::{OwnedPtr};
use base::{error};
use base::default::{Default};
use alloc::{NoMem, Allocator};
use io::{Read, BufRead, Write};
use arch_fns::{memchr};

pub mod std { pub use base::std::*; }

/// A buffered reader.
pub struct BufReader<R, Heap = alloc::Heap>
    where R: Read,
          Heap: Allocator,
{
    data: OwnedPtr<u8>,
    cap: usize,
    start: usize,
    end: usize,
    read: R,
    pool: Heap::Pool,
}

impl<R, H> BufReader<R, H>
    where R: Read,
          H: Allocator,
          H::Pool: Default,
{
    /// Allocates a new buffered reader.
    ///
    /// [argument, read]
    /// The reader that will be wrapped in the buffered reader.
    ///
    /// [argument, size]
    /// The buffer-size of the buffered reader.
    ///
    /// = Remarks
    ///
    /// `size` will be increased to the next power of two.
    pub fn new(read: R, size: usize) -> Result<BufReader<R, H>> {
        let size = match size.checked_next_power_of_two() {
            Some(n) => n,
            _ => return Err(error::NoMemory),
        };
        let mut pool = H::Pool::default();
        let ptr = unsafe { try!(H::allocate_array(&mut pool, size)) };
        let ptr = unsafe { OwnedPtr::new(ptr) };
        Ok(BufReader {
            data: ptr,
            cap: size,
            start: 0,
            end: 0,
            read: read,
            pool: pool,
        })
    }
}

impl<'a, R> BufReader<R, NoMem<'a>>
    where R: Read,
{
    /// Creates a new buffered reader that is backed by borrowed memory.
    ///
    /// [argument, read]
    /// The reader that will be wrapped in the buffered reader.
    ///
    /// [argument, buf]
    /// The buffer that will be used to back the buffered reader.
    ///
    /// = Remarks
    ///
    /// The length of `buf` will be decreased to the previous power of two.
    pub fn buffered(read: R, buf: &'a mut [u8]) -> BufReader<R, NoMem<'a>> {
        let size = match buf.len() {
            0 => 0,
            n => 1 << (usize::bits() - n.leading_zeros() - 1),
        };
        BufReader {
            data: unsafe { OwnedPtr::new(buf.as_mut_ptr()) },
            cap: size,
            start: 0,
            end: 0,
            read: read,
            pool: (),
        }
    }
}

impl<R, H> BufReader<R, H>
    where R: Read,
          H: Allocator,
{
    /// Returns the number of currently buffered bytes.
    pub fn available(&self) -> usize {
        self.end.wrapping_sub(self.start)
    }

    /// Returns the total buffer capacity.
    pub fn capacity(&self) -> usize {
        self.cap
    }

    fn fill(&mut self) -> Result<usize> {
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
            if start <= end && self.end.wrapping_sub(self.start) != self.cap {
                [slice::from_ptr(self.data.add(start), end - start),
                 &[][..]]
            } else {
                [slice::from_ptr(self.data.add(start), self.cap - start),
                 slice::from_ptr(*self.data, end)]
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
                 slice::from_ptr(*self.data, start)]
            } else {
                [slice::from_ptr(self.data.add(end), start - end),
                 &mut [][..]]
            }
        };
        (&mut self.read, slices)
    }
}

impl<R, H> Read for BufReader<R, H>
    where R: Read,
          H: Allocator,
{
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
        let res = try!(buf.gather_write(&self.read_slices().1));
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
            sum += try!(self.read(&mut buf[0]));
            let b = buf;
            buf = &mut b[1..];
        }
        Ok(sum)
    }
}

impl<R, H> Drop for BufReader<R, H>
    where R: Read,
          H: Allocator,
{
    fn drop(&mut self) {
        unsafe { H::free_array(&mut self.pool, *self.data, self.cap); }
    }
}

impl<R, H> BufRead for BufReader<R, H>
    where R: Read,
          H: Allocator,
{
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

    fn consume(&mut self, num: usize) -> usize {
        let num = cmp::min(num, self.available());
        self.start = self.start.wrapping_add(num);
        num
    }
}
