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
extern crate lrs_str_three as str_three;
extern crate lrs_alloc     as alloc;
extern crate lrs_rmo       as rmo;

mod std { pub use fmt::std::*; pub use {cty}; }

use base::prelude::*;
use core::{mem};
use syscall::{sync, chroot, pivot_root};
use cty::{PATH_MAX};
use rmo::{Rmo};
use str_three::{ToCString};
use alloc::{FbHeap};

pub mod info;
pub mod mount;
pub mod unmount;

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
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
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
    where P: ToCString,
          Q: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let new_root: Rmo<_, FbHeap> = try!(new_root.rmo_cstr(&mut buf1));
    let put_old: Rmo<_, FbHeap> = try!(put_old.rmo_cstr(&mut buf2));
    rv!(pivot_root(&new_root, &put_old))
}
