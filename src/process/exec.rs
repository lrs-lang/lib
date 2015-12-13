// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use base::{error};
use syscall::{execveat};
use cty::{AT_FDCWD, PATH_MAX, c_char};
use str_one::{CStr};
use str_two::{CString};
use rt::{raw_env};
use rmo::{Rmo, ToRmo};
use alloc::{MemPool, FbHeap, FcPool, OncePool};
use {env, file};

type Pool<'a> = FcPool<OncePool<'a>, FbHeap>;

fn rmo_cstr<'a, S>(s: &'a S,
                   buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, CString<Pool<'a>>>>
    where S: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
{
    s.to_rmo_with(FcPool::new(OncePool::new(buf), FbHeap::out_of(())))
}

/// Executes a program in place of the current program.
///
/// [argument, path]
/// The path of the file that is going to be executed.
///
/// [argument, argv]
/// The arguments that will be passed to the new program in form of `argv` and `argc`.
///
/// [return_value]
/// On success, this function does not return.
///
/// = Remarks
///
/// :execve: link:man:execve(2)
/// :cptrptr: link:lrs::string::CPtrPtr
///
/// The user must have permission to execute the program at `path`. The environment passed
/// to the program will be the same as the environment passed to this program.
///
/// The last element of the `argv` slice must be a null pointer. The other elements must
/// be pointers to null-terminated strings. The {cptrptr}[`CPtrPtr`] structure can be used
/// to build `argv`.
///
/// If `path` is an absolute path or starts with `./` or `../`, it is passed directly to
/// {execve}. Otherwise `exec` iteratively appends `path` to every element in the
/// `PATH` environment variable and executes {execve} on the first existing file. If no
/// such file can be found, an error is returned.
///
/// = See also
///
/// * {execve}
/// * {cptrptr}
pub fn exec<P>(path: P, argv: &[*const c_char]) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let file = try!(rmo_cstr(&path, &mut buf));
    if file.len() == 0 {
        return Err(error::InvalidArgument);
    } else if file[0] == b'/' {
        return rv!(execveat(-1, &file, argv.as_ptr(), raw_env(), 0));
    } else if file.len() > 1 && file[0] == b'.' {
        if file[1] == b'/' || (file.len() > 2 && file[1] == b'.' && file[2] == b'/') {
            return rv!(execveat(AT_FDCWD, &file, argv.as_ptr(), raw_env(), 0));
        }
    }
    
    // Try first without allocating

    let mut abs_buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    match exec_rel(&file, OncePool::new(&mut abs_buf), argv) {
        Err(error::NoMemory) => { },
        x => return x,
    }

    // NoMemory can come from our stuff or execve but we can't distinguish at this point.
    // Let's just try again with dynamic allocations.

    exec_rel(&file, FbHeap::out_of(()), argv)
}

fn exec_rel<H>(rel: &CStr, pool: H, args: &[*const c_char]) -> Result
    where H: MemPool,
{
    let mut buf = try!(CString::with_pool(pool));
    for path in try!(env::path()) {
        try!(buf.set_path(path));
        try!(buf.push_file(rel));
        let cstr = try!(buf.try_as_ref());
        if file::exists(cstr) == Ok(true) {
            // Paths in PATH don't have to start with a /. We pass AT_FDCWD so that such
            // paths are interpreted relative to the cwd.
            return rv!(execveat(AT_FDCWD, cstr, args.as_ptr(), raw_env(), 0));
        }
    }
    Err(error::DoesNotExist)
}
