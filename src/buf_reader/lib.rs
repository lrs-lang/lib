// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_buf_reader"]
#![crate_type = "lib"]
#![no_std]

extern crate lrs_arch_fns as arch_fns;
extern crate lrs_base as base;
extern crate lrs_io as io;
extern crate lrs_alloc as alloc;

use base::prelude::*;
use core::{slice, cmp};
use core::ptr::{NoAliasMemPtr};
use base::{error};
use alloc::{MemPool};
use io::{Read, BufRead, Write};
use arch_fns::{memchr};

pub mod std { pub use base::std::*; }

/// A buffered reader.
pub struct BufReader<R, Heap = alloc::Heap>
    where R: Read,
          Heap: MemPool,
{
    data: NoAliasMemPtr<u8>,
    cap: usize,
    start: usize,
    end: usize,
    read: R,
    pool: Heap,
}

impl<R, H = alloc::Heap> BufReader<R, H>
    where R: Read,
          H: MemPool,
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
    pub fn new(read: R, size: usize) -> Result<Self>
        where H: OutOf,
    {
        Self::with_pool(read, size, H::out_of(()))
    }

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
    pub fn with_pool(read: R, size: usize, mut pool: H) -> Result<Self> {
        let size = match size.checked_next_power_of_two() {
            Some(n) => n,
            _ => return Err(error::NoMemory),
        };
        let ptr = unsafe { try!(alloc::alloc_array(&mut pool, size)).0 };
        let ptr = unsafe { NoAliasMemPtr::new(ptr) };
        Ok(BufReader {
            data: ptr,
            cap: size,
            start: 0,
            end: 0,
            read: read,
            pool: pool,
        })
    }

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
            try!(read.scatter_read((&mut slices[..]).as_mut()))
        };
        self.end = self.end.wrapping_add(res);
        Ok(res)
    }

    fn read_slices(&mut self) -> (&mut usize, [&[u8]; 2]) {
        let start = self.start % self.cap;
        let end = self.end % self.cap;
        let slices: [&[u8]; 2] = unsafe {
            if start <= end && self.end.wrapping_sub(self.start) != self.cap {
                [slice::from_ptr(self.data.get().add(start), end - start),
                 &[][..]]
            } else {
                [slice::from_ptr(self.data.get().add(start), self.cap - start),
                 slice::from_ptr(self.data.get(), end)]
            }
        };
        (&mut self.start, slices)
    }

    fn write_slices(&mut self) -> (&mut R, [&mut [u8]; 2]) {
        let start = self.start % self.cap;
        let end = self.end % self.cap;
        let slices = unsafe {
            if start <= end {
                [slice::from_ptr(self.data.get().add(end), self.cap - end),
                 slice::from_ptr(self.data.get(), start)]
            } else {
                [slice::from_ptr(self.data.get().add(end), start - end),
                 &mut [][..]]
            }
        };
        (&mut self.read, slices)
    }
}

impl<R, H> Read for BufReader<R, H>
    where R: Read,
          H: MemPool,
{
    fn read(&mut self, mut buf: &mut [d8]) -> Result<usize> {
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

    fn scatter_read(&mut self, mut buf: &mut [&mut [d8]]) -> Result<usize> {
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
          H: MemPool,
{
    fn drop(&mut self) {
        unsafe { alloc::free_array(&mut self.pool, self.data.get(), self.cap); }
    }
}

impl<R, H> BufRead for BufReader<R, H>
    where R: Read,
          H: MemPool,
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
