// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Aborts the process.
#[macro_export]
macro_rules! abort {
    () => { ::lrs::intrinsics::lrs_abort() }
}

/// Asserts that a condition is satisfied. Aborts the process otherwise.
#[macro_export]
macro_rules! assert {
    ($pred:expr) => { if !$pred { abort!() } }
}

/// Asserts that a condition is satisfied. Aborts the process otherwise.
#[macro_export]
macro_rules! debug_assert {
    ($pred:expr) => { if cfg!(debug_assertions) { assert!($pred) } }
}

/// Unwraps the `Ok` branch of a `Result` and returns the error from the calling function
/// otherwise.
#[cfg(not(try_abort))]
#[macro_export]
macro_rules! try {
    ($val:expr) => {
        match $val {
            ::lrs::result::Result::Ok(v) => v,
            ::lrs::result::Result::Err(e) => return ::lrs::result::Result::Err(e),
        }
    }
}

/// Unwraps the `Ok` branch of a `Result`.
#[cfg(try_abort)]
#[macro_export]
macro_rules! try {
    ($val:expr) => { $val.unwrap() }
}

/// Prints a value to stdout.
///
/// Note that stdout always refers to the file descriptor `1`.
#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        write!(::lrs::fd::STDOUT, concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        write!(::lrs::fd::STDOUT, concat!($fmt, "\n"), $($arg)*)
    };
}

/// Like `println` but write to stder.
#[macro_export]
macro_rules! errln {
    ($fmt:expr) => {
        write!(::lrs::fd::STDERR, concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        write!(::lrs::fd::STDERR, concat!($fmt, "\n"), $($arg)*)
    };
}

/// Formats a value into a `ByteString`.
#[macro_export]
macro_rules! format {
    ($fmt:expr, $($arg:tt)*) => {{
        let mut vec = Vec::new();
        write!(vec, $fmt, $($arg)*);
        ::lrs::string::ByteString::from_vec(vec)
    }};
}

//#[macro_export]
//macro_rules! matches {
//    ($pat:pat = $val:expr) => {
//        match val { $pat => true, _ => false, }
//    }
//}

/// Creates a vector out of the arguments.
#[macro_export]
macro_rules! vec {
    ($elem:expr; $n:expr) => {
        ::lrs::vec::Vec::from_elem($elem, $n)
    };
    ($($x:expr),*) => {
        {
            let base = [$($x),*];
            let mut vec = ::lrs::vec::Vec::with_capacity(base.len()).unwrap();
            unsafe {
                vec.try_unsafe_push_all(&base[..]).unwrap();
                ::lrs::mem::forget(base);
            }
            vec
        }
    };
    ($($x:expr,)*) => { vec!($($x),*) };
}

#[macro_export]
macro_rules! rv {
    ($x:expr) => {{
        let val = $x;
        if val < 0 {
            ::lrs::result::Result::Err(::lrs::error::Errno(-val as ::lrs::cty::c_int))
        } else {
            ::lrs::result::Result::Ok(())
        }
    }};
    ($x:expr, -> $t:ty) => {{
        let val = $x;
        if val < 0 {
            ::lrs::result::Result::Err(::lrs::error::Errno(-val as ::lrs::cty::c_int))
        } else {
            ::lrs::result::Result::Ok(val as $t)
        }
    }};
}
