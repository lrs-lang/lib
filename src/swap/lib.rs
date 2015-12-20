// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_swap"]
#![crate_type = "lib"]
#![feature(custom_derive)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_fmt as fmt;
extern crate lrs_alloc as alloc;
extern crate lrs_rmo as rmo;
extern crate lrs_str_one as str_one;
extern crate lrs_str_two as str_two;

use base::prelude::*;
use core::{mem};
use base::{error};
use cty::{c_int, PATH_MAX, SWAP_FLAG_PREFER};
use alloc::{FbHeap, FcPool, OncePool};
use rmo::{Rmo, ToRmo};
use str_one::{CStr};
use str_two::{CString};
use flags::{SwapFlags};
use syscall::{swapon, swapoff};

mod std { pub use fmt::std::*; pub use cty; }

pub mod flags;

pub type Pool<'a> = FcPool<OncePool<'a>, FbHeap>;

fn rmo_cstr<'a, S>(s: &'a S,
                   buf: &'a mut [d8]) -> Result<Rmo<'a, CStr, CString<Pool<'a>>>>
    where S: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
{
    s.to_rmo_with(FcPool::new(OncePool::new(buf), FbHeap::out_of(())))
}

/// Adds a swap file/device.
///
/// [argument, path]
/// A path to the file/device to be used as swap space.
///
/// [argument, flags]
/// {
/// Flags to use to modify the swap behavior.
///
/// Only the SWAP_DISCARD flag should be used here. Use the third argument instead of
/// specifying the SWAP_PREFER flag.
/// }
///
/// [argument, priority]
/// {
/// The priority of this swap space.
///
/// If set, it must be a positive number. The SWAP_PREFER flag will be set depending on
/// whether this argument is set.
/// }
///
/// = See also
///
/// * link:man:swapon(2)
pub fn swap_on<P>(path: P, flags: SwapFlags, priority: Option<i16>) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut flags = flags.0;
    match priority {
        Some(p) if p < 0 => return Err(error::InvalidArgument),
        Some(p) => flags |= p as c_int | SWAP_FLAG_PREFER,
        _ => flags &= !SWAP_FLAG_PREFER,
    }
    let mut buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
    rv!(swapon(&path, flags))
}

/// Removes a swap file/device.
///
/// [argument, path]
/// A path to the file/device.
///
/// = See also
///
/// * link:man:swapoff(2)
pub fn swap_off<P>(path: P) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
    rv!(swapoff(&path))
}
