// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_inotify"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
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
extern crate lrs_io as io;

#[prelude_import] use base::prelude::*;
use syscall::{
    close, inotify_init1,
};
use io::{Read};
use cty::{c_int, c_char};
use core::{mem};
use fd::{FDContainer};
use event::{InodeEvents};
use flags::{InotifyFlags};
use str_one::{CStr};

mod lrs { pub use fmt::lrs::*; pub use cty; }

pub mod flags;
pub mod event;

#[repr(C)]
#[derive(Pod)]
pub struct InodeWatch(pub c_int);

#[repr(C)]
#[derive(Pod)]
pub struct InodeData {
    pub watch: InodeWatch,
    pub events: InodeEvents,
    pub cookie: u32,
    len: u32,
    name: [c_char; 0],
}

impl InodeData {
    pub fn name(&self) -> &CStr {
        if self.len == 0 {
            CStr::empty()
        } else {
            unsafe { CStr::from_ptr(self.name.as_ptr()) }
        }
    }
}

pub struct Inotify {
    fd: c_int,
    owned: bool,
}

impl Inotify {
    pub fn new(flags: InotifyFlags) -> Result<Inotify> {
        let fd = try!(rv!(inotify_init1(flags.0), -> c_int));
        Ok(Inotify::from_owned(fd))
    }

    pub fn events<'a>(&self, buf: &'a mut [u8]) -> Result<InodeDataIter<'a>> {
        let buf = mem::align_for_mut::<InodeData>(buf);
        let len = try!(self.as_fdio().read(buf));
        Ok(InodeDataIter { buf: &mut buf[..len] })
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

pub struct InodeDataIter<'a> {
    buf: &'a mut [u8],
}

impl<'a> Iterator for InodeDataIter<'a> {
    type Item = &'a mut InodeData;
    fn next(&mut self) -> Option<&'a mut InodeData> {
        if self.buf.len() == 0 {
            return None;
        }
        let inode_data: &'static mut InodeData = unsafe {
            mem::cast(mem::from_mut_bytes::<InodeData>(self.buf).unwrap())
        };
        let len = mem::size_of::<InodeData>() + inode_data.len as usize;
        self.buf = &mut mem::replace(&mut self.buf, &mut [])[len..];
        Some(inode_data)
    }
}
