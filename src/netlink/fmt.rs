// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use alloc::{self};
use core::{mem};
use cty::{nlmsghdr, nlattr};
use alloc::{NoMem, MemPool, AlignAlloc};
use vec::{Vec};
use kind::{self};
use flags::{self};

macro_rules! align { ($val:expr) => { ($val + 3) & !3 } }

pub struct NlBuf<H = alloc::Heap>
    where H: MemPool
{
    buf: Vec<u8, AlignAlloc<u32, H>>,
}

impl<'a> NlBuf<NoMem<'a>> {
    pub fn buffered(buf: &'a mut [u8]) -> NlBuf<NoMem<'a>> {
        let buf = mem::align_for_mut::<u32>(buf);
        NlBuf {
            buf: unsafe {
                Vec::from_raw_parts(buf.as_mut_ptr(), 0, buf.len(), AlignAlloc::out_of(()))
            },
        }
    }
}

impl<H> NlBuf<H>
    where H: MemPool+OutOf,
{
    pub fn new() -> NlBuf<H> {
        NlBuf {
            buf: Vec::new(),
        }
    }
}

impl<H> NlBuf<H>
    where H: MemPool,
{
    pub fn new_msg<'a>(&'a mut self, ty: kind::Kind, flags: flags::NlFlags, seq: u32,
                       port: u32) -> Result<NlMsg<'a, H>> {
        let len = mem::size_of::<nlmsghdr>();
        let hdr_pos = self.buf.len();
        try!(self.buf.reserve(align!(len)));
        unsafe {
            let hdr = &mut *(self.buf.unused().as_mut_ptr() as *mut nlmsghdr);
            hdr.nlmsg_type = ty.0;
            hdr.nlmsg_flags = flags.0;
            hdr.nlmsg_seq = seq;
            hdr.nlmsg_pid = port;
            self.buf.set_len(hdr_pos + mem::size_of::<nlmsghdr>());
        }
        Ok(NlMsg {
            data: NlData { buf: &mut self.buf },
            start: hdr_pos,
        })
    }
}

impl<H> AsRef<[u8]> for NlBuf<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &[u8] {
        &self.buf
    }
}

impl<H> TryAsRef<[u8]> for NlBuf<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&[u8]> {
        Ok(&self.buf)
    }
}

pub struct NlMsg<'a, H: 'a = alloc::Heap>
    where H: MemPool+'a,
{
    data: NlData<'a, H>,
    start: usize,
}

impl<'a, H> NlMsg<'a, H>
    where H: MemPool+'a,
{
    pub fn cancel(mut self) {
        self.data.buf.truncate(self.start);
        mem::forget(self);
    }
}

impl<'a, H> Deref for NlMsg<'a, H>
    where H: MemPool+'a,
{
    type Target = NlData<'a, H>;
    fn deref(&self) -> &NlData<'a, H> {
        &self.data
    }
}

impl<'a, H> DerefMut for NlMsg<'a, H>
    where H: MemPool+'a,
{
    fn deref_mut(&mut self) -> &mut NlData<'a, H> {
        &mut self.data
    }
}

impl<'a, H> Drop for NlMsg<'a, H>
    where H: MemPool+'a,
{
    fn drop(&mut self) {
        unsafe {
            let hdr = &mut *(self.data.buf[self.start..].as_mut_ptr() as *mut nlmsghdr);
            hdr.nlmsg_len = (self.data.buf.len() - self.start) as u32;
        }
    }
}

pub struct NlAttr<'a, H = alloc::Heap>
    where H: MemPool+'a,
{
    data: NlData<'a, H>,
    start: usize,
}

impl<'a, H> NlAttr<'a, H>
    where H: MemPool+'a,
{
    pub fn cancel(mut self) {
        self.data.buf.truncate(self.start);
        mem::forget(self);
    }
}

impl<'a, H> Deref for NlAttr<'a, H>
    where H: MemPool+'a,
{
    type Target = NlData<'a, H>;
    fn deref(&self) -> &NlData<'a, H> {
        &self.data
    }
}

impl<'a, H> DerefMut for NlAttr<'a, H>
    where H: MemPool+'a,
{
    fn deref_mut(&mut self) -> &mut NlData<'a, H> {
        &mut self.data
    }
}

impl<'a, H> Drop for NlAttr<'a, H>
    where H: MemPool+'a,
{
    fn drop(&mut self) {
        unsafe {
            let hdr = &mut *(self.data.buf[self.start..].as_mut_ptr() as *mut nlattr);
            hdr.nla_len = (self.data.buf.len() - self.start) as u16;
        }
    }
}

pub struct NlData<'a, H = alloc::Heap>
    where H: MemPool+'a,
{
    buf: &'a mut Vec<u8, AlignAlloc<u32, H>>,
}

impl<'a, H> NlData<'a, H>
    where H: MemPool+'a,
{
    unsafe fn add_attr(&mut self, payload: usize) -> Result<(&mut nlattr, &mut [u8])> {
        let size = mem::size_of::<nlattr>() + align!(payload);
        try!(self.buf.reserve(size));
        let len = self.buf.len();
        self.buf.set_len(len + size);
        let head = &mut *(self.buf[len..].as_mut_ptr() as *mut nlattr);
        head.nla_len = (mem::size_of::<nlattr>() + payload) as u16;
        let tail = &mut self.buf[len + mem::size_of::<nlattr>()..];
        Ok((head, tail))
    }

    pub fn add_flag(&mut self, ty: u16) -> Result {
        unsafe {
            let (attr, _) = try!(self.add_attr(0));
            attr.nla_type = ty;
            Ok(())
        }
    }

    pub fn add_simple<T: Copy>(&mut self, ty: u16, val: T) -> Result {
        unsafe {
            let (attr, data) = try!(self.add_attr(mem::size_of::<T>()));
            attr.nla_type = ty;
            if mem::align_of::<T>() <= 4 {
                *(data.as_mut_ptr() as *mut T) = val;
            } else {
                mem::copy(data, mem::as_bytes(&val));
            }
            Ok(())
        }
    }

    pub fn add_u8  (&mut self, ty: u16, val: u8)  -> Result { self.add_simple(ty, val) }
    pub fn add_i8  (&mut self, ty: u16, val: i8)  -> Result { self.add_simple(ty, val) }
    pub fn add_u16 (&mut self, ty: u16, val: u16) -> Result { self.add_simple(ty, val) }
    pub fn add_i16 (&mut self, ty: u16, val: i16) -> Result { self.add_simple(ty, val) }
    pub fn add_u32 (&mut self, ty: u16, val: u32) -> Result { self.add_simple(ty, val) }
    pub fn add_i32 (&mut self, ty: u16, val: i32) -> Result { self.add_simple(ty, val) }
    pub fn add_u64 (&mut self, ty: u16, val: u64) -> Result { self.add_simple(ty, val) }
    pub fn add_i64 (&mut self, ty: u16, val: i64) -> Result { self.add_simple(ty, val) }

    pub fn add_string<T: ?Sized>(&mut self, ty: u16, val: &T) -> Result
        where T: AsRef<[u8]>,
    {
        let val = val.as_ref();
        let (attr, data) = unsafe { try!(self.add_attr(val.len() + 1)) };
        attr.nla_type = ty;
        mem::copy(data, val);
        data[val.len()] = 0;
        Ok(())
    }

    pub fn add_data(&mut self, ty: u16, val: &[u8]) -> Result {
        let (attr, data) = unsafe { try!(self.add_attr(val.len())) };
        attr.nla_type = ty;
        mem::copy(data, val);
        Ok(())
    }

    pub fn add_raw(&mut self, val: &[u8]) -> Result {
        let size = align!(val.len());
        try!(self.buf.reserve(size));
        let len = self.buf.len();
        unsafe { self.buf.set_len(len + size); }
        mem::copy(&mut self.buf[len..], val);
        Ok(())
    }

    pub fn add_nested<'b>(&'b mut self, ty: u16) -> Result<NlAttr<'b, H>> {
        let hdr_pos = self.buf.len();
        unsafe {
            let hdr = &mut *(self.buf.unused().as_mut_ptr() as *mut nlattr);
            hdr.nla_len = 0;
            hdr.nla_type = ty;
            self.buf.set_len(hdr_pos + mem::size_of::<nlattr>());
        }
        Ok(NlAttr {
            data: NlData { buf: &mut self.buf },
            start: hdr_pos,
        })
    }
}
