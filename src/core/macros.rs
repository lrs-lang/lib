// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! abort {
    () => { unsafe { $crate::intrinsics::abort() } }
}

#[macro_export]
macro_rules! assert {
    ($pred:expr) => { if !$pred { abort!() } }
}

#[macro_export]
macro_rules! try {
    ($val:expr) => {
        match $val {
            $crate::result::Result::Ok(v) => v,
            $crate::result::Result::Err(e) => return $crate::result::Result::Err(e),
        }
    }
}
