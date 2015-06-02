// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_pipe"]
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
extern crate lrs_io as io;
extern crate lrs_saturating as saturating;

#[prelude_import] use base::prelude::*;
use syscall::{
    close, pipe2, read, write, readv, writev, fcntl_setpipe_sz, fcntl_getpipe_sz,
    ioctl_fionread, tee, splice,
};
use core::{mem};
use cty::{c_int, c_uint};
use fd::{FDContainer};
use flags::{PipeFlags, TeeFlags, SpliceFlags};
use io::{Read, Write};
use rv::{retry};
use saturating::{SaturatingCast};

mod lrs { pub use base::lrs::*; pub use cty; }

pub mod flags;

/// The maximum size of a pipe packet and atomic pipe writes.
///
/// = Remarks
///
/// Writes of at most this size are guaranteed to be atomic. When using a pipe in packet
/// mode, writes above this size are split into multiple packets. When reading from a pipe
/// in packet mode, buffers of this size are guaranteed to be able to hold all packets.
pub const PIPE_BUF: usize = 4096;

/// A kernel buffer.
pub struct Pipe {
    fd: c_int,
    owned: bool,
}

impl Pipe {
    /// Creates a new pipe.
    ///
    /// [argument, flags]
    /// Flags to use when creating the pipe.
    ///
    /// [return_value]
    /// Returns the write end (first element) and the read end (second argument) of the
    /// pipe.
    ///
    /// = See also
    ///
    /// * link:man:pipe2(2)
    pub fn new(flags: PipeFlags) -> Result<(Pipe, Pipe)> {
        let mut fds = [-1, -1];
        try!(rv!(pipe2(&mut fds, flags.0)));
        Ok((Pipe { fd: fds[1], owned: true }, Pipe { fd: fds[0], owned: true }))
    }

    /// Writes to the pipe.
    ///
    /// [argument, buf]
    /// The buffer that will be written to the pipe.
    ///
    /// [return_value]
    /// Returns the number of bytes written.
    ///
    /// = Remarks
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:write(2)
    /// * link:lrs::pipe::Pipe::gather_write
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        retry(|| write(self.fd, buf)).map(|r| r as usize)
    }

    /// Writes from multiple buffers to the pipe.
    ///
    /// [argument, bufs]
    /// The buffers that will be written to the pipe.
    ///
    /// [return_value]
    /// Returns the number of bytes written.
    ///
    /// = Remarks
    ///
    /// This operation is atomic in the sense that the write operations will not be
    /// interleaved with other operations on the same file description.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:writev(2)
    /// * link:lrs::pipe::pipe::write
    pub fn gather_write(&self, bufs: &[&[u8]]) -> Result<usize> {
        retry(|| writev(self.fd, bufs)).map(|r| r as usize)
    }

    /// Reads from the pipe.
    ///
    /// [argument, buf]
    /// The buffer that will be filled by the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes read.
    ///
    /// = Remarks
    ///
    /// If the length of the buffer is `0`, the meaning of a `0` return value is
    /// unspecified. Otherwise a return value of `0` signals End-Of-File.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:read(2)
    /// * link:lrs::pipe::Pipe::scatter_read
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        retry(|| read(self.fd, buf)).map(|r| r as usize)
    }

    /// Reads from the pipe into multiple buffers.
    ///
    /// [argument, bufs]
    /// The buffers that will be filled by the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes read.
    ///
    /// = Remarks
    ///
    /// This operation is atomic in the sense that the read operations will not be
    /// interleaved with other operations on the same file description.
    ///
    /// If the length of the buffer is `0`, the meaning of a `0` return value is
    /// unspecified. Otherwise a return value of `0` signals End-Of-File.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:readv(2)
    /// * link:lrs::pipe::Pipe::read
    pub fn scatter_read(&self, bufs: &mut [&mut [u8]]) -> Result<usize> {
        retry(|| readv(self.fd, bufs)).map(|r| r as usize)
    }

    /// Reutrns the capacity of the pipe.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and F_GETPIPE_SZ therein
    pub fn capacity(&self) -> Result<usize> {
        rv!(fcntl_getpipe_sz(self.fd), -> usize)
    }

    /// Sets the capacity of the pipe.
    ///
    /// [argument, cap]
    /// The new capacity of the pipe.
    ///
    /// = Remarks
    ///
    /// The new capacity must be able to hold the bytes currently buffered in the pipe or
    /// the operation fails.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and F_SETPIPE_SZ therein
    pub fn set_capacity(&self, cap: usize) -> Result {
        let size: c_uint = cap.saturating_cast();
        rv!(fcntl_setpipe_sz(self.fd, size as c_int))
    }

    /// Returns the number of bytes currently buffered in the pipe.
    pub fn len(&self) -> Result<usize> {
        let mut len = 0;
        try!(rv!(ioctl_fionread(self.fd, &mut len)));
        Ok(len)
    }

    /// Copies bytes from this pipe to another.
    ///
    /// [argument, dst]
    /// The write-end of the destination pipe.
    ///
    /// [argument, n]
    /// The number of bytes to copy.
    ///
    /// [argument, flags]
    /// The flags to use while copying.
    ///
    /// [return_value]
    /// Returns the number of bytes copied.
    ///
    /// = Remarks
    ///
    /// `self` must be the read-end of a pipe.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:tee(2)
    pub fn copy_to(&self, dst: &Pipe, n: usize, flags: TeeFlags) -> Result<usize> {
        retry(|| tee(self.fd, dst.fd, n, flags.0)).map(|v| v as usize)
    }

    /// Reads data from a file descriptor into the pipe.
    ///
    /// [argument, src]
    /// The source from which to read.
    ///
    /// [argument, n]
    /// The number of bytes to read.
    ///
    /// [argument, flags]
    /// Flags to use while reading.
    ///
    /// [return_value]
    /// Returns the number of bytes read.
    ///
    /// = Remarks
    ///
    /// `self` must be the write-end of a pipe.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:splice(2)
    pub fn read_from<T>(&self, src: &T, n: usize, flags: SpliceFlags) -> Result<usize>
        where T: FDContainer,
    {
        retry(|| {
            splice(src.borrow(), None, self.fd, None, n, flags.0)
        }).map(|v| v as usize)
    }

    /// Reads data from a position in a file descriptor into the pipe.
    ///
    /// [argument, src]
    /// The source from which to read.
    ///
    /// [argument, at]
    /// The position at which to read.
    ///
    /// [argument, n]
    /// The number of bytes to read.
    ///
    /// [argument, flags]
    /// Flags to use while reading.
    ///
    /// [return_value]
    /// Returns the number of bytes read.
    ///
    /// = Remarks
    ///
    /// `self` must be the write-end of a pipe. The read-position in the file will not be
    /// changed.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:splice(2)
    pub fn read_from_at<T>(&self, src: &T, at: &mut u64, n: usize,
                           flags: SpliceFlags) -> Result<usize>
        where T: FDContainer,
    {
        retry(|| {
            splice(src.borrow(), Some(at), self.fd, None, n, flags.0)
        }).map(|v| v as usize)
    }

    /// Writes data from this pipe to a file descriptor.
    ///
    /// [argument, dst]
    /// The file descriptor to write to.
    ///
    /// [argument, n]
    /// The number of bytes to write.
    ///
    /// [argument, flags]
    /// Flags to use while writing.
    ///
    /// [return_value]
    /// Returns the number of bytes written.
    ///
    /// = Remarks
    ///
    /// `self` must be the read-end of a pipe.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:splice(2)
    pub fn write_to<T>(&self, dst: &T, n: usize, flags: SpliceFlags) -> Result<usize>
        where T: FDContainer,
    {
        retry(|| {
            splice(self.fd, None, dst.borrow(), None, n, flags.0)
        }).map(|v| v as usize)
    }

    /// Writes data from this pipe to a position in a file descriptor.
    ///
    /// [argument, dst]
    /// The file descriptor to write to.
    ///
    /// [argument, at]
    /// The position at which to write.
    ///
    /// [argument, n]
    /// The number of bytes to write.
    ///
    /// [argument, flags]
    /// Flags to use while writing.
    ///
    /// [return_value]
    /// Returns the number of bytes written.
    ///
    /// = Remarks
    ///
    /// `self` must be the read-end of a pipe. The read-position in the file will not be
    /// changed.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:splice(2)
    pub fn write_to_at<T>(&self, dst: &T, at: &mut u64, n: usize,
                          flags: SpliceFlags) -> Result<usize>
        where T: FDContainer,
    {
        retry(|| {
            splice(self.fd, None, dst.borrow(), Some(at), n, flags.0)
        }).map(|v| v as usize)
    }
}

impl Read for Pipe {
    fn scatter_read(&mut self, buf: &mut [&mut [u8]]) -> Result<usize> {
        Pipe::scatter_read(self, buf)
    }
}

impl Write for Pipe {
    fn gather_write(&mut self, buf: &[&[u8]]) -> Result<usize> {
        Pipe::gather_write(self, buf)
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        close(self.fd);
    }
}

impl FDContainer for Pipe {
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

    fn from_owned(fd: c_int) -> Pipe {
        Pipe { fd: fd, owned: true }
    }

    fn from_borrowed(fd: c_int) -> Pipe {
        Pipe { fd: fd, owned: false }
    }
}
