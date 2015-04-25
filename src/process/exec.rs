// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[allow(unused_imports)] #[prelude_import] use base::prelude::*;
use core::{mem};
use base::{error};
use syscall::{execveat};
use cty::{AT_FDCWD, PATH_MAX, c_char};
use str_one::{AsMutCStr, CStr};
use str_two::{NoNullString};
use str_three::{ToCString};
use rt::{raw_env};
use rmo::{Rmo};
use alloc::{Allocator, FbHeap};
use {env, file};

/// Executes the program at `path` with arguments `args`.
///
/// The `args` argument can be built with the `CPtrPtr` structure. If `path` is relative,
/// then the `PATH` will be searched for a matching file.
pub fn exec<P>(path: P, args: &[*const c_char]) -> Result
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let file: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
    if file.len() == 0 {
        return Err(error::InvalidArgument);
    } else if file[0] == b'/' {
        return rv!(execveat(-1, &file, args.as_ptr(), raw_env(), 0));
    }
    
    // Try first without allocating

    let mut abs_buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let abs_file = NoNullString::buffered(&mut abs_buf);
    match exec_rel(&file, abs_file, args) {
        Err(error::NoMemory) => { },
        x => return x,
    }

    // NoMemory can come from our stuff or execve but we can't distinguish at this point.
    // Let's just try again with dynamic allocations.

    let abs_file: NoNullString<FbHeap> = NoNullString::new();
    exec_rel(&file, abs_file, args)
}

fn exec_rel<'a, H>(rel: &CStr, mut buf: NoNullString<'a, H>,
                   args: &[*const c_char]) -> Result
    where H: Allocator,
{
    for path in try!(env::path()) {
        try!(buf.set_path(path));
        try!(buf.push_file(rel));
        let cstr: &_ = try!(buf.as_mut_cstr());
        if file::exists(cstr) == Ok(true) {
            // Paths in PATH don't have to start with a /. We pass AT_FDCWD so that such
            // paths are interpreted relative to the cwd.
            return rv!(execveat(AT_FDCWD, cstr, args.as_ptr(), raw_env(), 0));
        }
    }
    Err(error::DoesNotExist)
}
