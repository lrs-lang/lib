// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_fd"]
#![crate_type = "lib"]
#![feature(plugin, no_std, macro_reexport)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_cty as cty;
extern crate linux_syscall as syscall;
extern crate linux_rv as rv;
extern crate linux_io as io;

#[prelude_import] use base::prelude::*;
use io::{Write, Read};
use cty::{c_int};
use syscall::{writev, readv};
use rv::{retry};

pub struct FdIo(pub c_int);

pub const STDOUT: FdIo = FdIo(1);
pub const STDERR: FdIo = FdIo(2);

impl Write for FdIo {
    fn gather_write(&mut self, buf: &[&[u8]]) -> Result<usize> {
        retry(|| writev(self.0, buf)).map(|r| r as usize)
    }
}

impl Read for FdIo {
    fn scatter_read(&mut self, buf: &mut [&mut [u8]]) -> Result<usize> {
        retry(|| readv(self.0, buf)).map(|r| r as usize)
    }
}

pub type FD = c_int;

pub trait FDContainer {
    fn unwrap(self) -> FD;
    fn is_owned(&self) -> bool;
    fn borrow(&self) -> FD;
    fn from_owned(fd: FD) -> Self;
    fn from_borrowed(fd: FD) -> Self;
}
