// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{num, mem, slice, ptr};
use cty::{
    cmsghdr, c_int, SCM_RIGHTS, SCM_CREDENTIALS, SOL_SOCKET, user_size_t,
    SO_TIMESTAMPNS, timespec, IPPROTO_IP, IP_OPTIONS,
};
use cty::alias::{ProcessId, UserId, GroupId};
use io::{BufRead};
use fmt::{Debug, Write};
use time_base::{self, Time};
use alloc::{self, NoMem, Allocator};

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

/// Process credentials.
///
/// = Remarks
///
/// Privileged process can send any ids. Unprivileged processes must send their process
/// id and the other ids must be either the effective, real, or saved ids.
///
/// = See also
///
/// * link:man:unix(7) and SCM_CREDENTIALS therein
#[repr(C)]
#[derive(Pod, Eq)]
pub struct Credentials {
    /// The process id of a process.
    pub process_id: ProcessId,
    /// The user id of a process.
    pub user_id:    UserId,
    /// The group id of a process.
    pub group_id:   GroupId,
}

impl Debug for Credentials {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "Credentials {{ process_id: {}, user_id: {}, group_id: {} }}",
               self.process_id, self.user_id, self.group_id)
    }
}

/// A timestamp.
///
/// = Remarks
///
/// The timestamp has nanosecond precision.
///
/// = See also
///
/// * link:man:socket(7) and SO_TIMESTAMP therein
#[repr(C)]
#[derive(Pod, Eq)]
pub struct Timestamp {
    data: timespec,
}

impl Timestamp {
    /// Turns the timestamp into a `Time` object.
    pub fn to_time(&self) -> Time {
        time_base::time_from_timespec(self.data)
    }
}

impl Debug for Timestamp {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.to_time().fmt(w)
    }
}

/// A generic control message.
#[derive(Copy, Eq)]
pub enum CMsg<'a> {
    /// An unknown control message.
    ///
    /// [field, 1]
    /// The data-part of the control message.
    ///
    /// = Remarks
    ///
    /// This will always be used if no wrapper for the control message type has been
    /// written yet.
    Unknown(&'a [u8]),

    /// A set of file descriptors.
    ///
    /// [field, 1]
    /// The file descriptors sent with the message.
    ///
    /// = See also
    ///
    /// * link:man:unix(7) and SCM_RIGHTS therein
    Fds(&'a [c_int]),

    /// Process credentials.
    ///
    /// [field, 1]
    /// The credentials sent by the process.
    Credentials(&'a Credentials),

    /// A timestamp.
    ///
    /// [field, 1]
    /// The timestamp.
    Timestamp(&'a Timestamp),

    /// Ipv4 options.
    ///
    /// [field, 1]
    /// The options.
    Ipv4Options(&'a [u8]),
}

impl<'a> Debug for CMsg<'a> {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        match *self {
            CMsg::Unknown(_) => write!(w, "Unknown"),
            CMsg::Fds(f) => write!(w, "Fds({:?})", f),
            CMsg::Credentials(c) => write!(w, "{:?}", c),
            CMsg::Timestamp(c) => write!(w, "{:?}", c),
            CMsg::Ipv4Options(c) => write!(w, "{:?}", c),
        }
    }
}

/// A buffer for creating control messages.
pub struct CMsgBuf<Heap = alloc::Heap>
    where Heap: Allocator,
{
    data: *mut u8,
    len: usize,
    cap: usize,
    _marker: PhantomData<Heap>,
}

impl<'a> CMsgBuf<NoMem<'a>> {
    /// Creates a new `CMsgBuf` backed by borrowed memory.
    ///
    /// [argument, buf]
    /// The buffer in which the control messages will be created.
    pub fn buffered(buf: &'a mut [u64]) -> CMsgBuf<NoMem<'a>> {
        CMsgBuf {
            data: buf.as_mut_ptr() as *mut u8,
            len: 0,
            cap: buf.len() * 8,
            _marker: PhantomData,
        }
    }
}

impl<H> CMsgBuf<H>
    where H: Allocator,
{
    /// Creates a new `CMsgBuf` backed by allocated memory.
    ///
    /// [return_value]
    /// Returns an allocated `CMsgBuf`.
    ///
    /// = Remarks
    ///
    /// The buffer will be resized dynamically. This constructor fails if no memory can be
    /// allocated.
    pub fn new() -> Result<CMsgBuf<H>> {
        let ptr: *mut usize = unsafe { try!(H::allocate_array(1)) };
        Ok(CMsgBuf {
            data: ptr as *mut u8,
            len: 0,
            cap: num::usize::BYTES,
            _marker: PhantomData,
        })
    }
}

impl<H> CMsgBuf<H>
    where H: Allocator,
{
    /// Returns the size currently occupied by the create messages.
    pub fn len(&self) -> usize {
        self.len
    }

    fn reserve(&mut self, n: usize) -> Result {
        if self.cap - self.len < n {
            let cap = self.cap / num::usize::BYTES;
            let new_cap = pad_ptr!(self.cap * 2 + n) / num::usize::BYTES;
            let ptr = unsafe {
                try!(H::reallocate_array(self.data as *mut usize, cap, new_cap))
            };
            self.data = ptr as *mut u8;
            self.cap = new_cap * num::usize::BYTES;
        }
        Ok(())
    }

    fn bytes(&mut self, bytes: &[u8], level: c_int, ty: c_int) -> Result {
        let msg_space = msg_space!(bytes.len());
        try!(self.reserve(msg_space));
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

    /// Adds a message containing file descriptors to the buffer.
    ///
    /// [argument, fds]
    /// The file descriptors to be added.
    pub fn fds(&mut self, fds: &[c_int]) -> Result {
        self.bytes(fds.as_ref(), SOL_SOCKET, SCM_RIGHTS)
    }

    /// Adds process credentials to the buffer.
    ///
    /// [argument, creds]
    /// The credentials to be added.
    pub fn credentials(&mut self, creds: Credentials) -> Result {
        self.bytes(mem::as_bytes(&creds), SOL_SOCKET, SCM_CREDENTIALS)
    }

    /// Creates an iterator over the messages.
    pub fn iter<'b>(&'b self) -> CMsgIter<'b> {
        CMsgIter { data: unsafe { slice::from_ptr(self.data, self.len) } }
    }
}

impl<H> Drop for CMsgBuf<H>
    where H: Allocator,
{
    fn drop(&mut self) {
        unsafe {
            H::free_array(self.data as *mut usize, self.cap / num::usize::BYTES);
        }
    }
}

impl<H> AsRef<[u8]> for CMsgBuf<H>
    where H: Allocator,
{
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_ptr(self.data, self.len) }
    }
}

/// An iterator over control messages.
pub struct CMsgIter<'a> {
    data: &'a [u8],
}

impl<'a> CMsgIter<'a> {
    /// Creates a new iterator.
    ///
    /// [argument, buf]
    /// The buffer which contains the control messages.
    ///
    /// = Remarks
    ///
    /// This operation fails if the slice is not properly aligned.
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

    fn timestamp(&self, len: usize) -> Option<CMsg<'a>> {
        if len != mem::size_of::<Timestamp>() {
            return None;
        }
        unsafe {
            let ptr = self.data.as_ptr() as *const Timestamp;
            Some(CMsg::Timestamp(&*ptr))
        }
    }

    fn ipv4_options(&self, len: usize) -> Option<CMsg<'a>> {
        Some(CMsg::Ipv4Options(&self.data[..len]))
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
            (SOL_SOCKET, SO_TIMESTAMPNS) => self.timestamp(data_len),
            (IPPROTO_IP, IP_OPTIONS) => self.ipv4_options(data_len),
            _ => Some(CMsg::Unknown(&self.data[..len])),
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
