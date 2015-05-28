// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_fd"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_rv as rv;
extern crate lrs_io as io;
extern crate lrs_fmt as fmt;

#[prelude_import] use base::prelude::*;
use base::error::{Errno};
use io::{Write, Read};
use cty::{c_int, FD_CLOEXEC};
use syscall::{
    writev, readv, read, write, fcntl_getfd, fcntl_setfd, fcntl_getfl, fcntl_setfl
};
use rv::{retry};
use flags::{DescriptionFlags};

mod lrs { pub use base::lrs::*; pub use cty; }

pub mod flags;

/// FdIo wrapping `0`.
pub const STDIN: FdIo = FdIo(0);

/// FdIo wrapping `1`.
pub const STDOUT: FdIo = FdIo(1);

/// FdIo wrapping `2`.
pub const STDERR: FdIo = FdIo(2);

/// A read/write wrapper for raw file descriptors.
///
/// [field, 1]
/// The numeric value of the file descriptor.
#[derive(Pod, Eq)]
pub struct FdIo(pub c_int);

impl Read for FdIo {
    fn scatter_read(&mut self, bufs: &mut [&mut [u8]]) -> Result<usize> {
        retry(|| readv(self.borrow(), bufs)).map(|r| r as usize)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        retry(|| read(self.borrow(), buf)).map(|r| r as usize)
    }
}

impl Write for FdIo {
    fn gather_write(&mut self, bufs: &[&[u8]]) -> Result<usize> {
        retry(|| writev(self.borrow(), bufs)).map(|r| r as usize)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        retry(|| write(self.borrow(), buf)).map(|r| r as usize)
    }
}

/// Objects that are file descriptor wrappers.
pub trait FDContainer {
    /// Consumes the object and returns the file descriptor without closing it.
    fn unwrap(self) -> c_int;

    /// Returns whether the object owns the file descriptor, i.e., whether it closes it
    /// when it goes out of scope.
    fn is_owned(&self) -> bool;

    /// Returns the contained file descriptor.
    fn borrow(&self) -> c_int;

    /// Creates a new owned object from a file descriptor.
    ///
    /// [argument, fd]
    /// The value of the file descriptor.
    ///
    /// = Remarks
    ///
    /// Not all objects support owned file descriptors. Check with `is_owned` if this
    /// matters.
    fn from_owned(fd: c_int) -> Self;

    /// Creates a new borrowed object from a file descriptor.
    ///
    /// [argument, fd]
    /// The value of the file descriptor.
    fn from_borrowed(fd: c_int) -> Self;

    /// Borrows the file descriptor as an `FdIo`.
    fn as_fdio(&self) -> FdIo {
        FdIo(self.borrow())
    }

    /// Retrieves the status of the `close on exec` flag.
    ///
    /// [return_value]
    /// Returns whether the `close on exec` flag is set.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and the description of `F_GETFD` therein.
    /// * link:lrs::fd::FDContainer::set_close_on_exec
    fn is_close_on_exec(&self) -> Result<bool> {
        let ret = fcntl_getfd(self.borrow());
        if ret < 0 {
            Err(Errno(-ret as c_int))
        } else {
            Ok(ret & FD_CLOEXEC != 0)
        }
    }

    /// Enables or disables the `close on exec` flag.
    ///
    /// [argument, val]
    /// Whether the flag is set.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and the description of `F_SETFD` therein.
    /// * link:lrs::file::File::io_close_on_exec
    fn set_close_on_exec(&self, val: bool) -> Result {
        let mut ret = fcntl_getfd(self.borrow());
        if ret >= 0 {
            ret = (ret & !FD_CLOEXEC) | (FD_CLOEXEC * val as c_int);
            ret = fcntl_setfd(self.borrow(), ret);
        }
        rv!(ret)
    }

    /// Retrieves the file description flags.
    ///
    /// [return_value]
    /// Returns the description flags.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and the description of `F_GETFL` therein.
    /// * link:lrs::fd::FDContainer::set_description_flags
    fn description_flags(&self) -> Result<DescriptionFlags> {
        let ret = fcntl_getfl(self.borrow());
        if ret < 0 {
            Err(Errno(-ret as c_int))
        } else {
            Ok(DescriptionFlags(ret))
        }
    }

    /// Sets the file description flags.
    ///
    /// [argument, flags]
    /// The modified flags.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and the description of `F_SETFL` therein.
    /// * link:lrs::fd::FDContainer::description_flags
    fn set_description_flags(&self, flags: DescriptionFlags) -> Result {
        let ret = fcntl_setfl(self.borrow(), flags.0);
        rv!(ret)
    }
}

impl FDContainer for FdIo {
    fn unwrap(self) -> c_int { self.0 }
    fn is_owned(&self) -> bool { false }
    fn borrow(&self) -> c_int { self.0 }
    fn from_owned(fd: c_int) -> FdIo { FdIo(fd) }
    fn from_borrowed(fd: c_int) -> FdIo { FdIo(fd) }
}

impl FDContainer for c_int {
    fn unwrap(self) -> c_int { self }
    fn is_owned(&self) -> bool { false }
    fn borrow(&self) -> c_int { *self }
    fn from_owned(fd: c_int) -> c_int { fd }
    fn from_borrowed(fd: c_int) -> c_int { fd }
}
