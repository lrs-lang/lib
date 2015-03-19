use std::num::{Int, SignedInt};

use errno;
use result::{Result};

#[cfg(feature = "retry")]
pub fn retry<T: SignedInt, F: FnMut() -> T>(mut f: F) -> Result<T> {
    let minus_one = -<T as Int>::one();
    loop {
        let ret = f();
        if ret == minus_one {
            let err = errno::get();
            if err != errno ::Interrupted {
                return Err(err);
            }
        } else {
            return Ok(ret);
        }
    }
}

#[cfg(not(feature = "retry"))]
pub fn retry<T: SignedInt, F: FnMut() -> T>(mut f: F) -> Result<T> {
    let minus_one = -<T as Int>::one();
    let ret = f();
    if f() == minus_one {
        Err(errno::get())
    } else {
        Ok(ret)
    }
}
