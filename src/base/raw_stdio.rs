// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use linux_ty_one::prelude::*;
use linux_io::{Write, Read};
use linux_arch::cty::{c_int};
use linux_arch::syscall::{writev, readv};
use util::{retry};

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
