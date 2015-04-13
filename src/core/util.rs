// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{mem};

use cty::{c_int, };
use errno::{Errno};
use result::{Result};
use ext::{SignedInt, Int, AsByteSlice};
use c_str::{CStr};

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
        Err(Errno(-ret.to_i64() as c_int))
    } else {
        Ok(ret)
    }
}

pub fn memchr<T: AsByteSlice+?Sized>(s: &T, c: u8) -> Option<usize> {
    let s = s.as_byte_slice();
    let mut idx = 0;
    while idx < s.len() {
        if s[idx] == c {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

pub unsafe fn memmove<T>(dst: *mut T, src: *const T, num: usize) {
    memmove_u8(dst as *mut u8, src as *const u8, num * mem::size_of::<T>());
}

unsafe fn memmove_u8(mut dst: *mut u8, mut src: *const u8, num: usize) {
    let mut dst_end = dst.offset(num as isize);
    let mut src_end = src.offset(num as isize);
    if (dst as *const u8) < src {
        while src != src_end {
            *dst = *src;
            dst = dst.offset(1);
            src = src.offset(1);
        }
    } else {
        while src != src_end {
            *dst_end = *src_end;
            dst_end = dst_end.offset(-1);
            src_end = src_end.offset(-1);
        }
    }
}

pub fn empty_cstr() -> &'static CStr {
    static EMPTY: [u8; 1] = [0];
    unsafe { CStr::from_nt_slice(&EMPTY) }
}

pub fn div_rem<T: Int>(a: T, b: T) -> (T, T) {
    (a / b, a % b)
}
