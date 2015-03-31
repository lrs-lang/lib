// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::num::{SignedInt};
use std::ffi::{CStr};

use cty::{c_int, c_void, size_t, c_char};
use errno::{Errno};
use result::{Result};

#[cfg(feature = "retry")]
pub fn retry<T: SignedInt, F: FnMut() -> T>(mut f: F) -> Result<T> {
    use errno;

    loop {
        let ret = f();
        if ret.is_negative() {
            let err = Errno(-ret.to_i64().unwrap() as c_int);
            if err != errno::Interrupted {
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
    if ret.is_negative() {
        Err(Errno(-ret.to_i64().unwrap() as c_int))
    } else {
        Ok(ret)
    }
}

pub fn memchr(s: &[u8], c: u8) -> Option<usize> {
    use cty::{memchr};

    let ptr = s.as_ptr();
    let res = unsafe { memchr(ptr as *const c_void, c as c_int, s.len() as size_t) };
    if res.is_null() {
        None
    } else {
        Some(res as usize - ptr as usize)
    }
}

pub fn empty_cstr() -> &'static CStr {
    static EMPTY: [c_char; 1] = [0];
    unsafe { CStr::from_ptr(EMPTY.as_ptr()) }
}
