// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_io"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_arch_fns as arch_fns;

use base::prelude::*;
use core::{mem, cmp};
use base::error::{DeviceFull};
use arch_fns::{memchr};

mod std { pub use base::std::*; }

/// Objects that wrap a byte-stream for reading.
pub trait Read {
    /// Reads from the byte-stream into multiple buffers.
    ///
    /// [argument, buf]
    /// The buffers that will be filled.
    ///
    /// [return_value]
    /// Returns the total number of bytes read.
    ///
    /// = Remarks
    ///
    /// The method starts at the first buffer and fills a buffer completely before moving
    /// to the next one. A `0` return value usually signals end-of-file unless the
    /// implementation documentation says something else. Many functions and structures
    /// will assume the `0` <-> end-of-file equivalence.
    fn scatter_read(&mut self, buf: &mut [&mut [d8]]) -> Result<usize>;

    /// Reads from the byte-stream into a buffer.
    ///
    /// [argument, buf]
    /// The buffer that will be filled.
    ///
    /// [return_value]
    /// Returns the total number of bytes read.
    ///
    /// = Remarks
    ///
    /// :scatter: link:lrs::io::Read::scatter_read[scatter_read]
    ///
    /// If the length of the buffer is `0`, the meaning of a `0` return value is
    /// unspecified. Otherwise a return value of `0` signals End-Of-File.
    ///
    /// The default implementation calls {scatter} with a single buffer element.
    ///
    /// = See also
    ///
    /// * {scatter}
    fn read(&mut self, buf: &mut [d8]) -> Result<usize> {
        self.scatter_read(&mut [buf])
    }

    /// Tries to read bytes until the buffer is buffer.
    ///
    /// [argument, buf]
    /// The buffer that will be filled.
    ///
    /// [return_value]
    /// Returns the total number of bytes read.
    ///
    /// = Remarks
    ///
    /// :read: link:lrs::io::Read::read[read]
    ///
    /// The default implementation calls {read} multiple times until the buffer is full or
    /// `0` is returned. If an error occurs the error is returned immediately and all
    /// bytes read up to that point are lost. This convenience method should thus not be
    /// used in reliable programs.
    ///
    /// = See also
    ///
    /// * {read}
    fn read_all(&mut self, mut buf: &mut [d8]) -> Result<usize> {
        let mut read = 0;
        while buf.len() > 0 {
            match self.read(buf) {
                e @ Err(_) => return e,
                Ok(0) => break,
                Ok(n) => {
                    read += n;
                    buf = &mut {buf}[n..];
                }
            }
        }
        Ok(read)
    }
}

/// Objects that wrap a byte-stream for reading and contain a buffer.
pub trait BufRead : Read {
    /// Copies bytes from the stream to a writer until a certain byte occurs.
    ///
    /// [argument, dst]
    /// The writer into which the stream will be piped.
    ///
    /// [argument, b]
    /// The byte at which to stop.
    ///
    /// [return_value]
    /// Returns the total number of bytes copied.
    ///
    /// = Remarks
    ///
    /// The stop-byte itself is copied to the destination. The copied bytes are no longer
    /// available for further read operations. If an error occurs, the error is returned
    /// immediately. In this case no bytes are lost since all read bytes have already been
    /// copied to the dst variable. The number of copied bytes is lost unless the dst
    /// variable has a means to obtain the number of copied bytes.
    fn copy_until<W: Write>(&mut self, dst: &mut W, b: u8) -> Result<usize>;

    /// Removes a certain number of bytes from the buffer.
    ///
    /// [argument, num]
    /// The number of bytes to remove.
    ///
    /// [return_value]
    /// Returns the actual number of bytes removed.
    ///
    /// = Remarks
    ///
    /// The returned value can be less than the num argument because there are fewer than
    /// num bytes currently buffered.
    fn consume(&mut self, num: usize) -> usize;
}

/// Objects that wrap a byte-stream for writing.
pub trait Write {
    /// Writes multiple buffers to the byte-stream.
    ///
    /// [argument, buf]
    /// The buffers to be written.
    ///
    /// [return_value]
    /// Returns the total number of bytes written.
    ///
    /// = Remarks
    ///
    /// The method starts at the first buffer and writes each buffer completely before
    /// moving to the next one.
    fn gather_write(&mut self, buf: &[&[u8]]) -> Result<usize>;

    /// Write a buffer to the byte stream.
    ///
    /// [argument, buf]
    /// The buffer to be written.
    ///
    /// [return_value]
    /// Returns the total number of bytes written.
    ///
    /// = Remarks
    ///
    /// :gather: link:lrs::io::Write::gather_write[gather_write]
    ///
    /// The default implementation calls {gather} with a single buffer.
    ///
    /// = See also
    ///
    /// * {gather}
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.gather_write(&[buf])
    }

    /// Tries to write the complete buffer to the byte-stream.
    ///
    /// [argument, buf]
    /// The buffer to be written.
    ///
    /// [return_value]
    /// Returns the total number of bytes written.
    ///
    /// = Remarks
    ///
    /// :write: link:lrs::io::Write::write[write]
    ///
    /// The default implementation calls `write` multiple times until the whole buffer has
    /// been written. If an error occurs, the error is returned immediately and the number
    /// of bytes written is lost. This method should thus not be used in reliable
    /// applications.
    fn write_all(&mut self, mut buf: &[u8]) -> Result<usize> {
        let mut written = 0;
        while buf.len() > 0 {
            match self.write(buf) {
                e @ Err(_) => return e,
                Ok(0) => return Err(DeviceFull),
                Ok(n) => {
                    written += n;
                    buf = &buf[n..];
                }
            }
        }
        Ok(written)
    }

    /// Writes a string to the byte-stream.
    ///
    /// [argument, buf]
    /// The string to be written.
    ///
    /// [return_value]
    /// Returns the total number of bytes written.
    ///
    /// = Remarks
    ///
    /// This is a convenience method that simply calls `write`.
    fn write_str(&mut self, buf: &str) -> Result<usize> {
        self.write(buf.as_bytes())
    }
}

/// Objects that wrap a byte-stream for writing and contain a buffer.
pub trait BufWrite : Write {
    /// Reads bytes from a `Read` object and writes them to the stream until end-of-file.
    ///
    /// [argument, r]
    /// The object from which to read.
    ///
    /// [return_value]
    /// Returns the number of bytes written.
    ///
    /// = Remarks
    ///
    /// If an error occurs, the error is returned immediately and the number of bytes
    /// copied is lost. No data is lost even if an error occurs.
    fn read_to_eof<R>(&mut self, r: R) -> Result<usize>
        where R: Read;
}

impl<'a> Read for &'a [u8] {
    fn read(&mut self, buf: &mut [d8]) -> Result<usize> {
        let n = mem::copy(buf, (**self).as_ref());
        *self = &self[n..];
        Ok(n)
    }

    fn scatter_read(&mut self, mut buf: &mut [&mut [d8]]) -> Result<usize> {
        let mut sum = 0;
        while self.len() > 0 && buf.len() > 0 {
            sum += self.read(&mut buf[0]).unwrap();
            let b = buf;
            buf = &mut b[1..];
        }
        Ok(sum)
    }
}

impl<'a> BufRead for &'a [u8] {
    fn copy_until<W: Write+?Sized>(&mut self, dst: &mut W, b: u8) -> Result<usize> {
        let mut len = match memchr(self, b) {
            Some(pos) => pos + 1,
            _ => self.len(),
        };
        let total = len;
        while len > 0 {
            let consumed = match try!(dst.write(&self[..len])) {
                0 => break,
                n => n,
            };
            len -= consumed;
            self.consume(consumed);
        }
        Ok(total - len)
    }

    fn consume(&mut self, num: usize) -> usize {
        let num = cmp::min(num, self.len());
        *self = &self[num..];
        num
    }
}

impl<'a> Write for &'a mut [u8] {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = mem::copy(*self, buf);
        unsafe {
            // Compiler bug.
            *self = mem::cast::<&mut [u8], &'a mut [u8]>(&mut self[n..]);
        }
        Ok(n)
    }

    fn gather_write(&mut self, mut buf: &[&[u8]]) -> Result<usize> {
        let mut sum = 0;
        while self.len() > 0 && buf.len() > 0 {
            sum += self.write(&buf[0]).unwrap();
            buf = &buf[1..];
        }
        Ok(sum)
    }
}

impl<'a> Write for &'a mut [d8] {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = mem::copy(*self, buf.as_ref());
        unsafe {
            // Compiler bug.
            *self = mem::cast::<&mut [d8], &'a mut [d8]>(&mut self[n..]);
        }
        Ok(n)
    }

    fn gather_write(&mut self, mut buf: &[&[u8]]) -> Result<usize> {
        let mut sum = 0;
        while self.len() > 0 && buf.len() > 0 {
            sum += self.write(&buf[0]).unwrap();
            buf = &buf[1..];
        }
        Ok(sum)
    }
}

impl<'a, T: Read+?Sized> Read for &'a mut T {
    fn scatter_read(&mut self, buf: &mut [&mut [d8]]) -> Result<usize> {
        (**self).scatter_read(buf)
    }

    fn read(&mut self, buf: &mut [d8]) -> Result<usize> {
        (**self).read(buf)
    }

    fn read_all(&mut self, mut buf: &mut [d8]) -> Result<usize> {
        (**self).read_all(buf)
    }
}

impl<'a, T: Write+?Sized> Write for &'a mut T {
    fn gather_write(&mut self, buf: &[&[u8]]) -> Result<usize> {
        (**self).gather_write(buf)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        (**self).write(buf)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<usize> {
        (**self).write_all(buf)
    }

    fn write_str(&mut self, buf: &str) -> Result<usize> {
        (**self).write_str(buf)
    }
}
