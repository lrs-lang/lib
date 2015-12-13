// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_fs"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts)]

extern crate lrs_base      as base;
extern crate lrs_fmt       as fmt;
extern crate lrs_cty       as cty;
extern crate lrs_syscall   as syscall;
extern crate lrs_rv        as rv;
extern crate lrs_alloc     as alloc;
extern crate lrs_rmo       as rmo;
extern crate lrs_str_one   as str_one;
extern crate lrs_str_two   as str_two;

mod std { pub use fmt::std::*; pub use {cty}; }

use base::prelude::*;
use core::{mem};
use syscall::{sync, chroot, pivot_root};
use cty::{PATH_MAX};
use rmo::{Rmo, ToRmo};
use alloc::{FbHeap, FcPool, OncePool};
use str_one::{CStr};
use str_two::{CString};

pub mod info;
pub mod mount;
pub mod unmount;

type Pool<'a> = FcPool<OncePool<'a>, FbHeap>;

fn rmo_cstr<'a, S>(s: &'a S,
                   buf: &'a mut [u8]) -> Result<Rmo<'a, CStr, CString<Pool<'a>>>>
    where S: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
{
    s.to_rmo_with(FcPool::new(OncePool::new(buf), FbHeap::out_of(())))
}

/// Writes all buffered data and metadata to the disks.
pub fn sync_all() {
    sync()
}

/// Changes the root directory of the process.
///
/// [argument, path]
/// The new root directory.
///
/// = See also
///
/// * link:man:chroot(2)
pub fn set_root<P>(path: P) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
    rv!(chroot(&path))
}

/// Moves the current root directory and sets a new one.
///
/// [argument, new_root]
/// The path of the new root directory.
///
/// [argument, put_old]
/// Where the old root directory will me moved to.
///
/// = See also
///
/// * link:man:pivot_root(2)
pub fn move_root<P, Q>(new_root: P, put_old: Q) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          Q: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let new_root = try!(rmo_cstr(&new_root, &mut buf1));
    let put_old = try!(rmo_cstr(&put_old, &mut buf2));
    rv!(pivot_root(&new_root, &put_old))
}
