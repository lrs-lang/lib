// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;

use ty_one::error::{Errno, c_int};
use ty_one::num::{SignedInt, Int};

#[cfg(feature = "retry")]
pub fn retry<T: SignedInt, F: FnMut() -> T>(mut f: F) -> Result<T> {
    use ty_one::{error};

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
