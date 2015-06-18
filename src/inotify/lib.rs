// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_inotify"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive, associated_consts)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_fmt as fmt;
extern crate lrs_syscall as syscall;
extern crate lrs_fd as fd;
extern crate lrs_rv as rv;
extern crate lrs_str_one as str_one;
extern crate lrs_str_three as str_three;
extern crate lrs_io as io;
extern crate lrs_alloc as alloc;
extern crate lrs_rmo as rmo;

#[prelude_import] use base::prelude::*;
use syscall::{
    close, inotify_init1, inotify_add_watch, inotify_rm_watch, ioctl_fionread,
};
use base::undef::{UndefState};
use io::{Read};
use cty::{c_int, c_char, PATH_MAX};
use core::{mem};
use alloc::{FbHeap};
use rmo::{Rmo};
use fd::{FDContainer};
use event::{InodeEvents};
use flags::{InotifyFlags, WatchFlags};
use str_one::{CStr};
use str_three::{ToCString};

mod lrs { pub use fmt::lrs::*; pub use cty; }

pub mod flags;
pub mod event;

/// An inotify watch.
///
/// [field, 1]
/// The integer representing the watch.
#[repr(C)]
#[derive(Pod)]
pub struct InodeWatch(pub c_int);

/// An inotify event.
#[repr(C)]
#[derive(Copy)]
pub struct InodeData {
    /// The watch that generated the event.
    pub watch: InodeWatch,
    /// The events that occured.
    pub events: InodeEvents,
    /// The cookie of the event.
    pub cookie: u32,
    len: u32,
    name: [c_char; 0],
}

impl InodeData {
    /// Returns the name of the file that triggered the event.
    pub fn name(&self) -> &CStr {
        if self.len == 0 {
            CStr::empty()
        } else {
            unsafe { CStr::from_ptr(self.name.as_ptr()) }
        }
    }
}

/// An inotify object.
pub struct Inotify {
    fd: c_int,
    owned: bool,
}

impl Inotify {
    /// Creates a new inotify object.
    ///
    /// [argument, flags]
    /// Flags to be used when creating the object.
    ///
    /// = See also
    ///
    /// * link:man:inotify_init1(2)
    pub fn new(flags: InotifyFlags) -> Result<Inotify> {
        let fd = try!(rv!(inotify_init1(flags.0), -> c_int));
        Ok(Inotify::from_owned(fd))
    }

    /// Adds a watch or changes the watch of a path.
    ///
    /// [argument, path]
    /// The path of the watch to add or modify.
    ///
    /// [argument, events]
    /// The events to watch for.
    ///
    /// [argument, flags]
    /// Flags to use when creating or modifying the watch.
    ///
    /// [return_value]
    /// Returns the added or modified watch.
    ///
    /// = See also
    ///
    /// * link:man:inotify_add_watch(2)
    pub fn set_watch<P>(&self, path: P, events: InodeEvents,
                        flags: WatchFlags) -> Result<InodeWatch>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let link: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        let watch = try!(rv!(inotify_add_watch(self.fd, &link,
                                               events.0 | flags.0), -> c_int));
        Ok(InodeWatch(watch))
    }

    /// Removes a watch.
    ///
    /// [argument, watch]
    /// The watch to remove.
    ///
    /// = See also
    ///
    /// * link:man:inotify_rm_watch(2)
    pub fn remove_watch(&self, watch: InodeWatch) -> Result {
        rv!(inotify_rm_watch(self.fd, watch.0))
    }

    /// Reads events and creates an iterator over those events.
    ///
    /// [argument, buf]
    /// The buffer in which the events will be stored.
    ///
    /// = Remarks
    ///
    /// The buffer will be aligned for `u32` data, meaning that up to 3 bytes of buffer
    /// space are lost.
    pub fn events<'a>(&self, buf: &'a mut [u8]) -> Result<InodeDataIter<'a>> {
        let buf = mem::align_for_mut::<InodeData>(buf);
        let len = try!(self.as_fdio().read(buf));
        Ok(InodeDataIter { buf: &mut buf[..len] })
    }

    /// Returns the number of bytes available for reading.
    pub fn available(&self) -> Result<usize> {
        let mut unread = 0;
        try!(rv!(ioctl_fionread(self.fd, &mut unread)));
        Ok(unread)
    }
}

unsafe impl UndefState for Inotify {
    fn num() -> usize { bool::num() }

    unsafe fn set_undef(val: *mut Inotify, n: usize) {
        bool::set_undef(&mut (*val).owned, n);
    }

    unsafe fn is_undef(val: *const Inotify, n: usize) -> bool {
        bool::is_undef(&(*val).owned, n)
    }
}

impl Drop for Inotify {
    fn drop(&mut self) {
        close(self.fd);
    }
}

impl FDContainer for Inotify {
    fn unwrap(self) -> c_int {
        let fd = self.fd;
        mem::forget(fd);
        fd
    }

    fn is_owned(&self) -> bool {
        self.owned
    }

    fn borrow(&self) -> c_int {
        self.fd
    }

    fn from_owned(fd: c_int) -> Inotify {
        Inotify { fd: fd, owned: true }
    }

    fn from_borrowed(fd: c_int) -> Inotify {
        Inotify { fd: fd, owned: false }
    }
}

/// An iterator over inotify events.
pub struct InodeDataIter<'a> {
    buf: &'a mut [u8],
}

impl<'a> Iterator for InodeDataIter<'a> {
    type Item = &'a mut InodeData;
    fn next(&mut self) -> Option<&'a mut InodeData> {
        if !mem::is_suitable_for::<InodeData>(self.buf) {
            return None;
        }
        let inode_data: &'static mut InodeData = unsafe {
            &mut *(self.buf.as_mut_ptr() as *mut InodeData)
        };
        let len = mem::size_of::<InodeData>() + inode_data.len as usize;
        if inode_data.len != 0 {
            if self.buf.len() < len || self.buf[len - 1] != 0 {
                return None;
            }
        }
        self.buf = &mut mem::replace(&mut self.buf, &mut [])[len..];
        Some(inode_data)
    }
}
