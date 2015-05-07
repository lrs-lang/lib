// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{num, mem, slice, ptr};
use base::{error};
use cty::{
    cmsghdr, c_int, SCM_RIGHTS, SCM_CREDENTIALS, SOL_SOCKET, user_size_t,
};
use cty::alias::{ProcessId, UserId, GroupId};
use io::{BufRead};
use fmt::{Debug, Write};

const PTR_MASK: usize = num::usize::BYTES - 1;

// pads an integer to a multiple of the pointer alignment
macro_rules! pad_ptr { ($val:expr) => { ($val + PTR_MASK) & !PTR_MASK } }

// the space occupied by a cmsg header
macro_rules! hdr_space { () => { pad_ptr!(mem::size_of::<cmsghdr>()) } }

// the space occupied by a message with data length $val
macro_rules! msg_space { ($val:expr) => { hdr_space!() + pad_ptr!($val) } }

// the value of the cmsg_len field for data length $val
macro_rules! msg_len { ($val:expr) => { hdr_space!() + $val } }

// pointer to the current cmsg header in a CMsgBuf
macro_rules! hdr_ptr { ($slf:expr) => { $slf.data.add($slf.len) as *mut cmsghdr } }

// pointer to the current data section in a CMsgBuf
macro_rules! data_ptr { ($slf:expr) => { $slf.data.add($slf.len + hdr_space!()) } }

#[repr(C)]
#[derive(Pod, Eq)]
pub struct Credentials {
    pub process_id: ProcessId,
    pub user_id:    UserId,
    pub group_id:   GroupId,
}

impl Debug for Credentials {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "Credentials {{ process_id: {}, user_id: {}, group_id: {} }}",
               self.process_id, self.user_id, self.group_id)
    }
}

#[derive(Copy, Eq)]
pub enum CMsg<'a> {
    Unknown,
    Fds(&'a [c_int]),
    Credentials(&'a Credentials),
}

impl<'a> Debug for CMsg<'a> {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        match *self {
            CMsg::Unknown => write!(w, "Unknown"),
            CMsg::Fds(f) => write!(w, "Fds({:?})", f),
            CMsg::Credentials(c) => write!(w, "{:?}", c),
        }
    }
}

pub struct CMsgBuf<'a> {
    data: *mut u8,
    len: usize,
    cap: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> CMsgBuf<'a> {
    pub fn new(buf: &'a mut [u8]) -> CMsgBuf<'a> {
        let ptr = buf.as_ptr() as usize;
        let pad_ptr = pad_ptr!(ptr);
        let cap = buf.len().saturating_sub(pad_ptr - ptr);
        let real_ptr = match cap {
            0 => ptr,
            _ => pad_ptr,
        };
        CMsgBuf {
            data: real_ptr as *mut u8,
            len: 0,
            cap: cap,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn bytes(&mut self, bytes: &[u8], level: c_int, ty: c_int) -> Result {
        let msg_space = msg_space!(bytes.len());
        if self.cap - self.len < msg_space {
            return Err(error::NoMemory);
        }
        unsafe {
            let hdr = hdr_ptr!(self);
            (*hdr).cmsg_len = msg_len!(bytes.len()) as user_size_t;
            (*hdr).cmsg_level = level;
            (*hdr).cmsg_type = ty;
            let data = data_ptr!(self);
            ptr::memcpy(data, bytes.as_ptr(), bytes.len());
            self.len += msg_space;
        }
        Ok(())
    }

    pub fn fds(&mut self, fds: &[c_int]) -> Result {
        self.bytes(fds.as_ref(), SOL_SOCKET, SCM_RIGHTS)
    }

    pub fn credentials(&mut self, creds: Credentials) -> Result {
        self.bytes(mem::as_bytes(&creds), SOL_SOCKET, SCM_CREDENTIALS)
    }

    pub fn iter(&self) -> CMsgIter<'a> {
        CMsgIter { data: unsafe { slice::from_ptr(self.data, self.len) } }
    }
}

impl<'a> AsRef<[u8]> for CMsgBuf<'a> {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_ptr(self.data, self.len) }
    }
}

pub struct CMsgIter<'a> {
    data: &'a [u8],
}

impl<'a> CMsgIter<'a> {
    pub fn new(buf: &'a [u8]) -> Option<CMsgIter<'a>> {
        if buf.as_ptr() as usize & PTR_MASK != 0 {
            None
        } else {
            Some(CMsgIter { data: buf })
        }
    }

    fn fds(&self, len: usize) -> Option<CMsg<'a>> {
        if len % mem::size_of::<c_int>() != 0 {
            return None;
        }
        unsafe {
            let fd_ptr = self.data.as_ptr() as *const c_int;
            let num = len / mem::size_of::<c_int>();
            Some(CMsg::Fds(slice::from_ptr(fd_ptr, num)))
        }
    }

    fn credentials(&self, len: usize) -> Option<CMsg<'a>> {
        if len != mem::size_of::<Credentials>() {
            return None;
        }
        unsafe {
            let ptr = self.data.as_ptr() as *const Credentials;
            Some(CMsg::Credentials(&*ptr))
        }
    }
}

impl<'a> Iterator for CMsgIter<'a> {
    type Item = CMsg<'a>;
    fn next(&mut self) -> Option<CMsg<'a>> {
        // Check that there is a header we can read info from.
        if self.data.len() < hdr_space!() {
            return None;
        }

        let (len, level, ty) = unsafe {
            let hdr = self.data.as_ptr() as *const cmsghdr;
            ((*hdr).cmsg_len as usize, (*hdr).cmsg_level, (*hdr).cmsg_type)
        };

        // Check that the full length of the message can be accessed and that the length
        // is not corrupted.
        if self.data.len() < pad_ptr!(len) || len < hdr_space!() {
            self.data.consume(self.data.len());
            return None;
        }

        // Strip the header so that the utility functions have less work to do.
        let data_len = len - hdr_space!();
        self.data.consume(hdr_space!());

        // Return `None` if the message is corrupted.
        let rv = match (level, ty) {
            (SOL_SOCKET, SCM_RIGHTS) => self.fds(data_len),
            (SOL_SOCKET, SCM_CREDENTIALS) => self.credentials(data_len),
            _ => Some(CMsg::Unknown),
        };
        if rv.is_none() {
            self.data.consume(self.data.len());
            return None;
        }

        // Strip the rest of the message.
        self.data.consume(pad_ptr!(data_len));

        rv
    }
}

impl<'a> AsRef<[u8]> for CMsgIter<'a> {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}
