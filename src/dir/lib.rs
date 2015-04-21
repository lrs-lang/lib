// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_dir"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_base      as base;
extern crate linux_core      as core;
extern crate linux_cty       as cty;
extern crate linux_str_one   as str_one;
extern crate linux_str_two   as str_two;
extern crate linux_str_three as str_three;
extern crate linux_syscall   as syscall;
extern crate linux_fd        as fd;
extern crate linux_rmo       as rmo;
extern crate linux_fmt       as fmt;
extern crate linux_file      as file;
extern crate linux_vec       as vec;

#[prelude_import] use base::prelude::*;
mod linux { pub use fmt::linux::*; }

use cty::{linux_dirent64, MODE_TYPE_SHIFT, umode_t, PATH_MAX};
use str_one::{CStr};
use str_two::{ByteString};
use str_three::{ToCString};
use syscall::{getdents};
use base::error::{Errno};
use vec::{Vec};
use fd::{FDContainer};
use base::rmo::{AsRef};
use fmt::{Debug, Write};
use core::{mem};
use rmo::{ToOwned};

use file::{File, Seek};
use file::flags::{Flags};
use file::info::{Type, file_type_from_mode};

/// The default buffer size used for reading directory entries.
pub const DEFAULT_BUF_SIZE: usize = 2048;

/// Creates an iterator over the entries in the directory `path`.
///
/// Errors can optionally be stored in the `error` variable which should then be inspected
/// after the iterator returns ends the loop.
pub fn iter<'a, S>(path: S, error: Option<&'a mut Result>) -> Iter<'a>
    where S: ToCString,
{

    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    match path.rmo_cstr(&mut buf) {
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
        let mut flags = Flags::new();
        flags.set_only_directory(true);
        match File::open(path, flags) {
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
#[derive(Clone)]
pub struct Entry {
    pub inode: u64,
    pub ty:    Type,
    pub name:  ByteString,
}

impl Debug for Entry {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "Entry {{ inode: {}, ty: {:?}, name: {:?} }}",
               self.inode, self.ty, self.name)
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
            let name = CStr::from_ptr(ent.d_name.as_ptr()).as_ref();
            if name == b"." || name == b".." {
                self.next()
            } else {
                match name.to_owned() {
                    Ok(n) => Some(Entry {
                        inode: ent.d_ino,
                        ty:    ty,
                        name:  ByteString::from_vec(n),
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

//#[derive(Copy, Clone, Debug, Eq, PartialEq)]
//pub enum WalkOp {
//    Abort,
//    Continue,
//    Recurse,
//}
//
//#[derive(Clone, Debug)]
//pub struct WalkEntry<'a> {
//    pub inode: u64,
//    pub ty:    Type,
//    pub name:  &'a LinuxStr,
//}
//
//pub fn walk<S, F>(path: S, mut f: F)
//    where S: AsLinuxPath,
//          F: FnMut(&WalkEntry) -> WalkOp,
//{
//    walk_int(path.as_linux_path(), &mut f);
//}
//
//fn walk_int<F>(path: &Path, f: &mut F) 
//    where F: FnMut(&WalkEntry) -> WalkOp
//{
//    let mut iter = iter(path, None);
//    loop {
//        if iter.buf.position() as usize == iter.buf.get_ref().len() {
//            if iter.read().is_err() { break; }
//        }
//        let pos = iter.buf.position() as usize;
//        if pos == iter.buf.get_ref().len() {
//            break;
//        }
//        let entry = unsafe {
//            let ent = &*(iter.buf.get_ref()[pos..].as_ptr() as *const linux_dirent64);
//            let ent_len = ent.d_reclen as usize;
//            iter.buf.set_position((pos + ent_len) as u64);
//            let name = CStr::from_ptr(ent.d_name.as_ptr()).to_bytes();
//            let ty = file_type_from_mode((ent.d_type as umode_t) << MODE_TYPE_SHIFT);
//            WalkEntry {
//                inode: ent.d_ino,
//                ty:    ty,
//                name:  name.as_linux_str(),
//            }
//        };
//        match f(&entry) {
//            WalkOp::Abort => break,
//            WalkOp::Continue => { },
//            WalkOp::Recurse => {
//                let mut path = path.to_path_buf();
//                path.push(entry.name);
//                walk_int(&path, f);
//            },
//        }
//    }
//}
