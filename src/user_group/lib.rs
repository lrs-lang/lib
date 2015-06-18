// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_user_group"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core       as core;
extern crate lrs_base       as base;
extern crate lrs_arch_fns   as arch_fns;
extern crate lrs_io         as io;
extern crate lrs_buf_reader as buf_reader;
extern crate lrs_fmt        as fmt;
extern crate lrs_str_one    as str_one;
extern crate lrs_str_two    as str_two;
extern crate lrs_cty        as cty;
extern crate lrs_parse      as parse;
extern crate lrs_file       as file;
extern crate lrs_vec        as vec;
extern crate lrs_alloc      as alloc;
extern crate lrs_rmo        as rmo;
extern crate lrs_iter       as iter;

#[prelude_import] use base::prelude::*;
mod lrs { pub use vec::lrs::*; }

use core::{mem};
use core::ptr::{memmove};
use base::error::{self};
use arch_fns::{memchr};

use file::{File};

pub mod group;
pub mod user;

struct LineReader<'a> {
    start: usize,
    end: usize,
    file: File,
    err: Option<&'a mut Result>,
}

impl<'a> LineReader<'a> {
    fn new(file: &str, error: Option<&'a mut Result>) -> LineReader<'a> {
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

    fn set_err(&mut self, e: error::Errno) {
        if let Some(ref mut err) = self.err {
            **err = Err(e);
        }
    }

    fn fill<'b>(&mut self, buf: &'b mut [u8]) -> &'b [u8] {
        loop {
            {
                // Borrow checked doesn't understand that return ends the loop.
                let cur: &'static [u8] = unsafe { mem::cast(&buf[self.start..self.end]) };
                if let Some(pos) = memchr(cur, b'\n') {
                    self.start += pos + 1;
                    return &cur[..pos];
                }
            }
            // No newline in the current buffer.
            // Move it to the left, try to read more, repeat.
            unsafe {
                let dst = buf.as_mut_ptr();
                let src = dst.add(self.start);
                memmove(dst, src, self.end - self.start);
            }
            self.end -= self.start;
            self.start = 0;
            match self.file.read(&mut buf[self.end..]) {
                Err(e) => {
                    // This can be error::Interrupted but only if the library was compiled
                    // without the 'retry' feature. The user wants to handle it himself.
                    self.set_err(e);
                    return &[];
                },
                Ok(0) => {
                    if self.end == buf.len() {
                        // The buffer is too small for this entry.
                        self.set_err(error::NoMemory);
                    } else if self.end > self.start {
                        // Not at EOF but the buffer is not empty. The file is corrupted.
                        self.set_err(error::InvalidSequence);
                    }
                    return &[];
                },
                Ok(n) => self.end += n,
            }
        }
    }
}
