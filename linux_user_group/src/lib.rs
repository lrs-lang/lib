// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_user_group"]
#![crate_type = "lib"]

extern crate "linux_core" as core;
extern crate "linux_file" as file;

use std::{mem, ptr};

use core::result::{Result};
use core::errno::{self};
use core::util::{memchr};

use file::{File};

pub mod group;
pub mod user;

struct LineReader<'a> {
    start: usize,
    end: usize,
    file: File,
    err: Option<&'a mut Result<()>>,
}

impl<'a> LineReader<'a> {
    fn new(file: &str, error: Option<&'a mut Result<()>>) -> LineReader<'a> {
        match File::open_read(file) {
            Err(e) => {
                if let Some(err) = error { *err = Err(e); }
                LineReader {
                    start: 0,
                    end: 0,
                    file: File::invalid(),
                    err: None,
                }
            },
            Ok(f) => LineReader {
                start: 0,
                end: 0,
                file: f,
                err: error,
            },
        }
    }

    fn set_err(&mut self, e: errno::Errno) {
        if let Some(ref mut err) = self.err {
            **err = Err(e);
        }
    }

    fn fill<'b>(&mut self, buf: &'b mut [u8]) -> &'b [u8] {
        loop {
            {
                // Borrow checked doesn't understand that return ends the loop.
                let cur = unsafe { mem::transmute(&buf[self.start..self.end]) };
                if let Some(pos) = memchr(cur, b'\n') {
                    self.start += pos + 1;
                    return &cur[..pos];
                }
            }
            // No newline in the current buffer.
            // Move it to the left, try to read more, repeat.
            let dst = buf.as_mut_ptr();
            let src = unsafe { dst.offset(self.start as isize) };
            unsafe { ptr::copy(dst, src, self.end - self.start); }
            self.end -= self.start;
            self.start = 0;
            match self.file.read(&mut buf[self.end..]) {
                Err(e) => {
                    // This can be errno::Interrupted but only if the library was compiled
                    // without the 'retry' feature. The user wants to handle it himself.
                    self.set_err(e);
                    return &[];
                },
                Ok(0) => {
                    if self.end == buf.len() {
                        // The buffer is too small for this entry.
                        self.set_err(errno::NoMemory);
                    } else if self.end > self.start {
                        // Not at EOF but the buffer is not empty. The file is corrupted.
                        self.set_err(errno::InvalidSequence);
                    }
                    return &[];
                },
                Ok(n) => self.end += n,
            }
        }
    }
}
