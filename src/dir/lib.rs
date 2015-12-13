// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_dir"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base      as base;
extern crate lrs_cty       as cty;
extern crate lrs_str_one   as str_one;
extern crate lrs_str_two   as str_two;
extern crate lrs_syscall   as syscall;
extern crate lrs_fd        as fd;
extern crate lrs_rmo       as rmo;
extern crate lrs_fmt       as fmt;
extern crate lrs_file      as file;
extern crate lrs_alloc     as alloc;
extern crate lrs_vec       as vec;

use base::prelude::*;
mod std { pub use fmt::std::*; }

use cty::{linux_dirent64, MODE_TYPE_SHIFT, umode_t, PATH_MAX};
use str_one::{CStr, ByteStr};
use str_two::{CString};
use syscall::{getdents};
use base::error::{Errno};
use vec::{Vec};
use fd::{FdContainer};
use fmt::{Debug, Write};
use core::{mem};
use rmo::{Rmo, ToRmo};
use alloc::{FbHeap, FcPool, OncePool};

use file::{File, Seek};
use file::flags::{FILE_ONLY_DIRECTORY, Mode};
use file::info::{Type, file_type_from_mode};

/// The default buffer size used for reading directory entries.
pub const DEFAULT_BUF_SIZE: usize = 2048;

type Pool<'a> = FcPool<OncePool<'a>, FbHeap>;

fn rmo_cstr<'a, S>(s: &'a S,
                   buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, CString<Pool<'a>>>>
    where S: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
{
    s.to_rmo_with(FcPool::new(OncePool::new(buf), FbHeap::out_of(())))
}

/// Creates an iterator over the entries in a directory.
///
/// [argument, path]
/// The path of the directory to be inspected.
///
/// [argument, error]
/// Optional storage space for an error that occurs during the iteation.
///
/// = Remarks
///
/// If the error argument is not `None`, an error that occurs during the iteration will be
/// stored in its place. After the iteration the error variable should be inspected for an
/// error.
pub fn iter<'a, S>(path: S, error: Option<&'a mut Result>) -> Iter<'a>
    where S: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = rmo_cstr(&path, &mut buf);
    match path {
        Ok(p) => Iter::new(&p, error),
        Err(e) => Iter::error_dummy(e, error),
    }
}

/// An iterator over the entries in a directory.
pub struct Iter<'a> {
    dir: File,
    buf: Vec<u8>,
    buf_pos: usize,
    err: Option<&'a mut Result>,
}

impl<'a> Iter<'a> {
    fn new(path: &CStr, error: Option<&'a mut Result>) -> Iter<'a> {
        match File::open(path, FILE_ONLY_DIRECTORY, Mode(0)) {
            Ok(file) => {
                match Vec::with_capacity(DEFAULT_BUF_SIZE) {
                    Ok(v) => Iter {
                        dir: file,
                        buf: v,
                        buf_pos: 0,
                        err: error,
                    },
                    Err(err) => Iter::error_dummy(err, error),
                }
            },
            Err(err) => Iter::error_dummy(err, error),
        }
    }

    fn error_dummy(err: Errno, error: Option<&'a mut Result>) -> Iter<'a> {
        if let Some(e) = error {
            *e = Err(err);
        }
        Iter {
            dir: File::invalid(),
            buf: Vec::new(),
            buf_pos: 0,
            err: None,
        }
    }

    fn set_err(&mut self, e: Errno) {
        if let Some(ref mut err) = self.err {
            **err = Err(e);
        }
    }

    /// Rewind the iterator to the first entry.
    pub fn rewind(&mut self) -> Result {
        self.buf_pos = 0;
        self.buf.truncate(0);
        self.dir.seek(Seek::Start(0)).ignore_ok()
    }

    fn read(&mut self) -> Result {
        self.buf_pos = 0;
        let cap = self.buf.capacity();
        let res = unsafe {
            self.buf.set_len(cap);
            getdents(self.dir.borrow(), &mut self.buf)
        };
        if res < 0 {
            self.buf.truncate(0);
            Err(Errno(-res))
        } else {
            self.buf.truncate(res as usize);
            Ok(())
        }
    }
}

/// An entry in a directory.
#[derive(TryTo)]
pub struct Entry {
    /// The inode of the entry.
    pub inode: u64,
    /// The type of the entry.
    pub ty:    Type,
    /// The name of the entry.
    pub name:  Vec<u8>,
}

impl Debug for Entry {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "Entry {{ inode: {}, ty: {:?}, name: {:?} }}",
               self.inode, self.ty, self.name.as_str())
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Entry;

    fn next(&mut self) -> Option<Entry> {
        if self.buf_pos as usize == self.buf.len() {
            if let Err(e) = self.read() {
                self.set_err(e);
                return None;
            }
        }
        if self.buf_pos == self.buf.len() {
            return None;
        }
        unsafe {
            let ent = &*(self.buf[self.buf_pos..].as_ptr() as *const linux_dirent64);
            let ent_len = ent.d_reclen as usize;
            self.buf_pos += ent_len;
            let ty = file_type_from_mode((ent.d_type as umode_t) << MODE_TYPE_SHIFT);
            let name = CStr::from_ptr(ent.d_name.as_ptr());
            if name == "." || name == ".." {
                self.next()
            } else {
                match AsRef::<ByteStr>::as_ref(name).try_to() {
                    Ok(n) => Some(Entry {
                        inode: ent.d_ino,
                        ty:    ty,
                        name:  n,
                    }),
                    Err(e) => {
                        self.set_err(e);
                        None
                    },
                }
            }
        }
    }
}
