// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[allow(unused_imports)] #[prelude_import] use base::prelude::*;
use core::{mem};
use c_ptr_ptr::{CPtrPtr};
use cty::{PATH_MAX, AT_FDCWD};
use str_one::{ToCStr};
use str_three::{ToCString};
use syscall::{execveat};

pub struct Command<'a> {
    builder: CPtrPtr<'a>,
}

impl<'a> Command<'a> {
    pub fn new(buf: &'a mut [u8]) -> Command<'a> {
        Command {
            builder: CPtrPtr::new(buf),
        }
    }

    pub fn arg<A>(&mut self, arg: A) -> Result
        where A: ToCStr,
    {
        self.builder.push(arg)
    }

    pub fn exec<P>(&mut self, path: P) -> Result
        where P: ToCString,
    {
        let args = try!(self.builder.finish());
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        rv!(execveat(AT_FDCWD, &path, args, 0 as *const _, 0))
    }
}
