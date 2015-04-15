// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! linux_shim {
    () => { mod linux { pub use ::core::linux::*; } }
}

#[macro_export]
macro_rules! abort {
    () => { unsafe { ::linux::intrinsics::abort() } }
}

#[macro_export]
macro_rules! assert {
    ($pred:expr) => { if !$pred { abort!() } }
}

#[macro_export]
macro_rules! try {
    ($val:expr) => {
        match $val {
            ::linux::result::Result::Ok(v) => v,
            ::linux::result::Result::Err(e) => return ::linux::result::Result::Err(e),
        }
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        write!(Stdout, concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        write!(Stdout, concat!($fmt, "\n"), $($arg)*)
    };
}

#[macro_export]
macro_rules! matches {
    ($pat:pat = $val:expr) => {
        match val { $pat => true, _ => false, }
    }
}
