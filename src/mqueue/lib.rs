// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_mqueue"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive, associated_consts)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_fmt as fmt;
extern crate lrs_syscall as syscall;
extern crate lrs_fd as fd;
extern crate lrs_rv as rv;
extern crate lrs_file as file;
extern crate lrs_rmo as rmo;
extern crate lrs_time_base as time_base;
extern crate lrs_alloc as alloc;
extern crate lrs_str_two as str_two;
extern crate lrs_str_one as str_one;

use base::prelude::*;
use core::{mem};
use base::undef::{UndefState};
use fd::{FdContainer};
use file::flags::{FileFlags, Mode};
use fmt::{Debug, Write};
use cty::{mq_attr, NAME_MAX, c_int, c_uint, k_long};
use str_one::{CStr};
use str_two::{CString};
use rmo::{Rmo, ToRmo};
use rv::{retry};
use alloc::{FbHeap, FcPool, OncePool};
use syscall::{close, mq_open, mq_timedsend, mq_timedreceive, mq_getsetattr, mq_unlink};
use time_base::{time_to_timespec, Time};
use flags::{MqFlags};

mod std { pub use fmt::std::*; pub use cty; }

pub mod flags;

type Pool<'a> = FcPool<OncePool<'a>, FbHeap>;

fn rmo_cstr<'a, S>(s: &'a S,
                   buf: &'a mut [d8]) -> Result<Rmo<'a, CStr, CString<Pool<'a>>>>
    where S: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
{
    s.to_rmo_with(FcPool::new(OncePool::new(buf), FbHeap::out_of(())))
}

/// Attributes of a message queue.
pub struct MqAttr {
    /// Flags of the queue.
    pub flags: MqFlags,
    /// Maximum number of messages stored in the queue.
    pub max_msgs: usize,
    /// Number of currently stored messages.
    pub cur_msgs: usize,
    /// The maximum size of a message in the queue.
    pub msg_size: usize,
}

impl MqAttr {
    fn to_native(&self) -> mq_attr {
        mq_attr {
            mq_flags: self.flags.0,
            mq_maxmsg: self.max_msgs as k_long,
            mq_curmsgs: self.cur_msgs as k_long,
            mq_msgsize: self.msg_size as k_long,
            __reserved: [0; 4],
        }
    }

    fn from_native(attr: mq_attr) -> MqAttr {
        MqAttr {
            flags: MqFlags(attr.mq_flags),
            max_msgs: attr.mq_maxmsg as usize,
            cur_msgs: attr.mq_curmsgs as usize,
            msg_size: attr.mq_msgsize as usize,
        }
    }
}

impl Debug for MqAttr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "MqAttr {{ flags: {:?}, max_msgs: {}, cur_msgs: {}, msg_size: {} }}",
               self.flags, self.max_msgs, self.cur_msgs, self.msg_size)
    }
}

/// A kernel message queue.
pub struct MsgQueue {
    fd: c_int,
    owned: bool,
}

impl MsgQueue {
    /// Opens or creates a message queue.
    ///
    /// [argument, path]
    /// The name of the message queue.
    ///
    /// [argument, flags]
    /// Flags to be used when opening the message queue.
    ///
    /// [argument, mode]
    /// The mode of a newly created message queue.
    ///
    /// [argument, attr]
    /// The attributes of a newly created message queue.
    ///
    /// = Remarks
    ///
    /// The path should not contain any '/'. The mode and attr arguments only matter if
    /// the flags argument contains the `FILE_CREATE` flag. See the manual page for more
    /// details.
    ///
    /// :unlink: link:lrs::msg_queue::remove[remove]
    ///
    /// When the queue is no longer needed, it should be deleted with {unlink}.
    ///
    /// = See also
    ///
    /// * link:man:mq_open(2)
    /// * {unlink}
    pub fn open<P>(path: P, flags: FileFlags, mode: Mode,
                   attr: Option<MqAttr>) -> Result<MsgQueue>
        where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
    {
        let mut buf: [d8; NAME_MAX] = unsafe { mem::uninit() };
        let path = try!(rmo_cstr(&path, &mut buf));
        let attr = attr.map(|a| a.to_native());
        let fd = try!(rv!(mq_open(&path, flags.0, mode.0, attr.as_ref()), -> c_int));
        Ok(MsgQueue::from_owned(fd))
    }

    /// Sends a message over the message queue.
    ///
    /// [argument, msg]
    /// The message to be sent.
    ///
    /// [argument, priority]
    /// The priority of the message.
    ///
    /// = Remarks
    ///
    /// The priority should be between 0 and `i16::max()`.
    ///
    /// = See also
    ///
    /// * link:man:mq_timedsend(2)
    pub fn send(&self, msg: &[u8], priority: u16) -> Result {
        retry(|| mq_timedsend(self.fd, msg.as_ref(), priority as c_uint, None)).ignore_ok()
    }

    /// Sends a message over the message queue with a timeout.
    ///
    /// [argument, msg]
    /// The message to be sent.
    ///
    /// [argument, priority]
    /// The priority of the message.
    ///
    /// [argument, timeout]
    /// Until when this function should block.
    ///
    /// = Remarks
    ///
    /// The priority should be between 0 and `i16::max()`. The timeout argument is
    /// absolute. See the example.
    ///
    /// = Examples
    ///
    /// ----
    /// let flags = FILE_READ_WRITE | FILE_CREATE;
    /// let queue = MsgQueue::open("test", flags, MODE_FILE, None).unwrap();
    /// let timeout = REAL.get_time().unwrap() + Time::seconds(1);
    /// queue.send_timeout("hello, world".as_ref(), 1, timeout));
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:mq_timedsend(2)
    pub fn send_timeout(&self, msg: &[u8], priority: u16, timeout: Time) -> Result {
        let timeout = time_to_timespec(timeout);
        retry(|| mq_timedsend(self.fd, msg.as_ref(), priority as c_uint,
                              Some(&timeout))).ignore_ok()
    }

    /// Receives a message over the message queue.
    ///
    /// [argument, buf]
    /// A buffer in which the message will be stored.
    ///
    /// [return_value]
    /// Returns the message and its priority.
    ///
    /// = See also
    ///
    /// * link:man:mq_timedreceive(2)
    pub fn recv<'a>(&self, buf: &'a mut [u8]) -> Result<(&'a mut [u8], u16)> {
        let mut prio = 0;
        let len = try!(retry(|| mq_timedreceive(self.fd, buf.as_mut(), Some(&mut prio),
                                                None))) as usize;
        Ok((&mut buf[..len], prio as u16))
    }

    /// Receives a message over the message queue with a timeout.
    ///
    /// [argument, buf]
    /// A buffer in which the message will be stored.
    ///
    /// [argument, timeout]
    /// Until when this function should block.
    ///
    /// [return_value]
    /// Returns the message and its priority.
    ///
    /// = Remarks
    ///
    /// :send_timeout: link:lrs::msg_queue::MsgQueue::send_timeout[send_timeout]
    ///
    /// The timeout argument is absolute. See the example in {send_timeout}.
    ///
    /// = See also
    ///
    /// * link:man:mq_timedreceive(2)
    /// * {send_timeout}
    pub fn recv_timeout<'a>(&self, buf: &'a mut [u8],
                            timeout: Time) -> Result<(&'a mut [u8], u16)> {
        let timeout = time_to_timespec(timeout);
        let mut prio = 0;
        let len = try!(retry(|| mq_timedreceive(self.fd, buf.as_mut(), Some(&mut prio),
                                                Some(&timeout)))) as usize;
        Ok((&mut buf[..len], prio as u16))
    }

    /// Returns the attributes of the message queue.
    ///
    /// = See also
    ///
    /// * link:man:mq_getattr(3)
    pub fn attributes(&self) -> Result<MqAttr> {
        let mut attr = mem::zeroed();
        try!(rv!(mq_getsetattr(self.fd, None, Some(&mut attr))));
        Ok(MqAttr::from_native(attr))
    }

    /// Sets the attributes of the message queue.
    ///
    /// [return_value]
    /// Returns the previous attributes of the message queue.
    ///
    /// = See also
    ///
    /// * link:man:mq_setattr(3)
    pub fn set_attributes(&self, attr: MqAttr) -> Result<MqAttr> {
        let attr = attr.to_native();
        let mut new = mem::zeroed();
        try!(rv!(mq_getsetattr(self.fd, Some(&attr), Some(&mut new))));
        Ok(MqAttr::from_native(new))
    }
}

unsafe impl UndefState for MsgQueue {
    fn num() -> usize { bool::num() }

    unsafe fn set_undef(val: *mut MsgQueue, n: usize) {
        bool::set_undef(&mut (*val).owned, n);
    }

    unsafe fn is_undef(val: *const MsgQueue, n: usize) -> bool {
        bool::is_undef(&(*val).owned, n)
    }
}

impl Drop for MsgQueue {
    fn drop(&mut self) {
        if self.owned {
            close(self.fd);
        }
    }
}

impl Into<c_int> for MsgQueue {
    fn into(self) -> c_int {
        let fd = self.fd;
        mem::forget(fd);
        fd
    }
}

impl FdContainer for MsgQueue {
    fn is_owned(&self) -> bool {
        self.owned
    }

    fn borrow(&self) -> c_int {
        self.fd
    }

    fn from_owned(fd: c_int) -> MsgQueue {
        MsgQueue { fd: fd, owned: true }
    }

    fn from_borrowed(fd: c_int) -> MsgQueue {
        MsgQueue { fd: fd, owned: false }
    }
}

/// Removes a message queue.
///
/// [argument, path]
/// The name of the queue.
///
/// = See also
///
/// * link:man:mq_unlink(2)
pub fn remove<P>(path: P) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [d8; NAME_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
    rv!(mq_unlink(&path))
}
