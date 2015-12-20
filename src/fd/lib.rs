// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_fd"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_rv as rv;
extern crate lrs_io as io;
extern crate lrs_fmt as fmt;

use base::prelude::*;
use base::error::{Errno};
use io::{Write, Read};
use cty::{c_int, FD_CLOEXEC, O_CLOEXEC};
use syscall::{
    writev, readv, read, write, fcntl_getfd, fcntl_setfd, fcntl_getfl, fcntl_setfl,
    fcntl_dupfd_cloexec, dup3,
};
use rv::{retry};
use flags::{DescriptionFlags};

mod std { pub use base::std::*; pub use cty; }

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
    fn scatter_read(&mut self, bufs: &mut [&mut [d8]]) -> Result<usize> {
        retry(|| readv(self.borrow(), bufs)).map(|r| r as usize)
    }

    fn read(&mut self, buf: &mut [d8]) -> Result<usize> {
        retry(|| read(self.borrow(), buf)).map(|r| r as usize)
    }
}

impl<'a> Read for &'a FdIo {
    fn scatter_read(&mut self, bufs: &mut [&mut [d8]]) -> Result<usize> {
        retry(|| readv(self.borrow(), bufs)).map(|r| r as usize)
    }

    fn read(&mut self, buf: &mut [d8]) -> Result<usize> {
        retry(|| read(self.borrow(), buf)).map(|r| r as usize)
    }
}

impl Write for FdIo {
    fn gather_write(&mut self, bufs: &[&[u8]]) -> Result<usize> {
        retry(|| writev(self.borrow(), bufs.as_ref())).map(|r| r as usize)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        retry(|| write(self.borrow(), buf.as_ref())).map(|r| r as usize)
    }
}

impl<'a> Write for &'a FdIo {
    fn gather_write(&mut self, bufs: &[&[u8]]) -> Result<usize> {
        retry(|| writev(self.borrow(), bufs.as_ref())).map(|r| r as usize)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        retry(|| write(self.borrow(), buf.as_ref())).map(|r| r as usize)
    }
}

/// Objects that are file descriptor wrappers.
pub trait FdContainer: Into<c_int> {
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
    /// * link:lrs::fd::FdContainer::set_close_on_exec
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
    /// * link:lrs::fd::FdContainer::set_description_flags
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
    /// * link:lrs::fd::FdContainer::description_flags
    fn set_description_flags(&self, flags: DescriptionFlags) -> Result {
        let ret = fcntl_setfl(self.borrow(), flags.0);
        rv!(ret)
    }

    /// Duplicates the file descriptor.
    ///
    /// = Remarks
    ///
    /// The `close on exec` flag will automatically be set on the new file descriptor.
    ///
    /// = See also
    ///
    /// * link:lrs::fd::FdContainer::duplicate_min
    fn duplicate(&self) -> Result<Self>
        where Self: Sized
    {
        self.duplicate_min(0)
    }

    /// Duplicates the file descriptor so that the duplicated one has a minimum value.
    ///
    /// [argument, min]
    /// The minimum value of the new file descriptor.
    ///
    /// = Remarks
    ///
    /// The `close on exec` flag will automatically be set on the new file descriptor.
    ///
    /// = See also
    ///
    /// * link:lrs::fd::FdContainer::duplicate
    /// * link:man:fcntl(2) and F_DUPFD_CLOEXEC therein
    fn duplicate_min(&self, min: c_int) -> Result<Self>
        where Self: Sized
    {
        let new = try!(rv!(fcntl_dupfd_cloexec(self.borrow(), min), -> c_int));
        Ok(Self::from_owned(new))
    }

    /// Duplicates the file descriptor, replacing an existing one.
    ///
    /// [argument, new]
    /// The file descriptor to replace.
    ///
    /// = Remarks
    ///
    /// The `new` argument can refer to an open file descriptor but does not have to. In
    /// this case, `new` will be atomically closed and replaced by a duplicate of this
    /// file descriptor.
    ///
    /// = See also
    ///
    /// * link:man:dup3(2)
    fn duplicate_as(&self, new: c_int) -> Result<Self>
        where Self: Sized
    {
        let new = try!(rv!(dup3(self.borrow(), new, O_CLOEXEC), -> c_int));
        Ok(Self::from_owned(new))
    }
}

impl Into<c_int> for FdIo {
    fn into(self) -> c_int {
        self.0
    }
}

impl FdContainer for FdIo {
    fn is_owned(&self) -> bool { false }
    fn borrow(&self) -> c_int { self.0 }
    fn from_owned(fd: c_int) -> FdIo { FdIo(fd) }
    fn from_borrowed(fd: c_int) -> FdIo { FdIo(fd) }
}

impl FdContainer for c_int {
    fn is_owned(&self) -> bool { false }
    fn borrow(&self) -> c_int { *self }
    fn from_owned(fd: c_int) -> c_int { fd }
    fn from_borrowed(fd: c_int) -> c_int { fd }
}
