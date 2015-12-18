// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{slice, mem};
use core::marker::{Leak};
use {Vec};
use alloc::{MemPool};
use io::{Write, BufWrite, Read};
use str_one::{ByteStr};

impl<H: ?Sized> Vec<u8, H>
    where H: MemPool,
{
    pub fn as_str(&self) -> &ByteStr {
        self.deref().as_ref()
    }

    pub fn as_mut_str(&mut self) -> &mut ByteStr {
        self.deref_mut().as_mut()
    }

    pub unsafe fn unused(&mut self) -> &mut [u8] {
        slice::from_ptr(self.ptr.get().add(self.len), self.cap - self.len)
    }

    pub fn leak<'a>(mut self) -> &'a mut ByteStr
        where Self: Leak+Sized,
              H: 'a,
    {
        let p = unsafe { mem::cast(self.as_mut_str()) };
        mem::forget(self);
        p
    }
}

impl<H: ?Sized> Write for Vec<u8, H>
    where H: MemPool,
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

impl<H: ?Sized> BufWrite for Vec<u8, H>
    where H: MemPool,
{
    fn read_to_eof<R>(&mut self, mut r: R) -> Result<usize>
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
