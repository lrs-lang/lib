// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_dir"]
#![crate_type = "lib"]
#![allow(trivial_numeric_casts)]

extern crate linux_core as core;
extern crate linux_file as file;

use std::path::{Path};
use std::io::{Cursor};
use std::ffi::{CStr};

use core::cty::{linux_dirent64, MODE_TYPE_SHIFT, mode_t};
use core::ext::{AsLinuxPath};
use core::string::{LinuxString, LinuxStr, AsLinuxStr};
use core::result::{Result};
use core::syscall::{getdents};
use core::errno::{Errno};
use core::fd_container::{FDContainer};

use file::{File, Seek};
use file::flags::{Flags};
use file::info::{Type, file_type_from_mode};

/// The default buffer size used for reading directory entries.
pub const DEFAULT_BUF_SIZE: usize = 2048;

/// Creates an iterator over the entries in the directory `path`.
///
/// Errors can optionally be stored in the `error` variable which should then be inspected
/// after the iterator returns ends the loop.
pub fn iter<'a, S: AsLinuxPath>(path: S, error: Option<&'a mut Result>) -> Iter<'a> {
    Iter::new(path.as_linux_path(), error)
}

/// An iterator over the entries in a directory.
pub struct Iter<'a> {
    dir: File,
    buf: Cursor<Vec<u8>>,
    err: Option<&'a mut Result>,
}

impl<'a> Iter<'a> {
    fn new(path: &Path, error: Option<&'a mut Result>) -> Iter<'a> {
        let mut flags = Flags::new();
        flags.set_only_directory(true);
        match File::open(path, flags) {
            Ok(file) => {
                Iter {
                    dir: file,
                    buf: Cursor::new(Vec::with_capacity(DEFAULT_BUF_SIZE)),
                    err: error,
                }
            },
            Err(err) => {
                if let Some(e) = error {
                    *e = Err(err);
                }
                Iter {
                    dir: File::invalid(),
                    buf: Cursor::new(vec!()),
                    err: None,
                }
            },
        }
    }

    fn set_err(&mut self, e: Errno) {
        if let Some(ref mut err) = self.err {
            **err = Err(e);
        }
    }

    /// Rewind the iterator to the first entry.
    pub fn rewind(&mut self) -> Result {
        self.buf.set_position(0);
        unsafe { self.buf.get_mut().set_len(0); }
        self.dir.seek(Seek::Start(0)).map(|_| ())
    }

    fn read(&mut self) -> Result {
        self.buf.set_position(0);
        unsafe {
            let inner = self.buf.get_mut();
            let cap = inner.capacity();
            inner.set_len(cap);
            let res = getdents(self.dir.borrow(), inner);
            if res < 0 {
                inner.set_len(0);
                Err(Errno(-res))
            } else {
                inner.set_len(res as usize);
                Ok(())
            }
        }
    }
}

/// An entry in a directory.
#[derive(Clone, Debug)]
pub struct Entry {
    pub inode: u64,
    pub ty:    Type,
    pub name:  LinuxString,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Entry;

    fn next(&mut self) -> Option<Entry> {
        if self.buf.position() as usize == self.buf.get_ref().len() {
            if let Err(e) = self.read() {
                self.set_err(e);
                return None;
            }
        }
        let pos = self.buf.position() as usize;
        if pos == self.buf.get_ref().len() {
            return None;
        }
        unsafe {
            let ent = &*(self.buf.get_ref()[pos..].as_ptr() as *const linux_dirent64);
            let ent_len = ent.d_reclen as usize;
            self.buf.set_position((pos + ent_len) as u64);
            let ty = file_type_from_mode((ent.d_types as mode_t) << MODE_TYPE_SHIFT);
            let name = CStr::from_ptr(ent.d_name.as_ptr()).to_bytes();
            if name == b"." || name == b".." {
                self.next()
            } else {
                Some(Entry {
                    inode: ent.d_ino,
                    ty:    ty,
                    name:  LinuxString::from_bytes(name),
                })
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WalkOp {
    Abort,
    Continue,
    Recurse,
}

#[derive(Clone, Debug)]
pub struct WalkEntry<'a> {
    pub inode: u64,
    pub ty:    Type,
    pub name:  &'a LinuxStr,
}

pub fn walk<S, F>(path: S, mut f: F)
    where S: AsLinuxPath,
          F: FnMut(&WalkEntry) -> WalkOp,
{
    walk_int(path.as_linux_path(), &mut f);
}

fn walk_int<F>(path: &Path, f: &mut F) 
    where F: FnMut(&WalkEntry) -> WalkOp
{
    let mut iter = iter(path, None);
    loop {
        if iter.buf.position() as usize == iter.buf.get_ref().len() {
            if iter.read().is_err() { break; }
        }
        let pos = iter.buf.position() as usize;
        if pos == iter.buf.get_ref().len() {
            break;
        }
        let entry = unsafe {
            let ent = &*(iter.buf.get_ref()[pos..].as_ptr() as *const linux_dirent64);
            let ent_len = ent.d_reclen as usize;
            iter.buf.set_position((pos + ent_len) as u64);
            let name = CStr::from_ptr(ent.d_name.as_ptr()).to_bytes();
            let ty = file_type_from_mode((ent.d_types as mode_t) << MODE_TYPE_SHIFT);
            WalkEntry {
                inode: ent.d_ino,
                ty:    ty,
                name:  name.as_linux_str(),
            }
        };
        match f(&entry) {
            WalkOp::Abort => break,
            WalkOp::Continue => { },
            WalkOp::Recurse => {
                let mut path = path.to_path_buf();
                path.push(entry.name);
                walk_int(&path, f);
            },
        }
    }
}
