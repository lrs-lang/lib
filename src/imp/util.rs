use std::num::{Int, SignedInt};

use imp::cty::{c_int};
use imp::errno::{self, Errno};
use imp::result::{Result};

#[cfg(feature = "retry")]
pub fn retry<T: SignedInt, F: FnMut() -> T>(mut f: F) -> Result<T> {
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
