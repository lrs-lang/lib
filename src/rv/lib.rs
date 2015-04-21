// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_rv"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_int as int;

#[prelude_import] use base::prelude::*;
use int::{SignedInt, Int};
use base::error::{Errno, c_int};

#[cfg(feature = "retry")]
pub fn retry<T: SignedInt, F: FnMut() -> T>(mut f: F) -> Result<T> {
    use base::{error};

    loop {
        let ret = f();
        if ret.negative() {
            let err = Errno(-ret.cast_i64() as c_int);
            if err != error::Interrupted {
                return Err(err);
            }
        } else {
            return Ok(ret);
        }
    }
}

#[cfg(not(feature = "retry"))]
pub fn retry<T: SignedInt, F: FnMut() -> T>(mut f: F) -> Result<T> {
    let ret = f();
    if ret.negative() {
        Err(Errno(-ret.cast_i64() as c_int))
    } else {
        Ok(ret)
    }
}
