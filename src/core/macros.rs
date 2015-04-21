// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Aborts the process.
#[macro_export]
macro_rules! abort {
    () => { ::linux::intrinsics::linux_abort() }
}

/// Asserts that a condition is satisfied. Aborts the process otherwise.
#[macro_export]
macro_rules! assert {
    ($pred:expr) => { if !$pred { abort!() } }
}

/// Unwraps the `Ok` branch of a `Result` and returns the error from the calling function
/// otherwise.
#[macro_export]
macro_rules! try {
    ($val:expr) => {
        match $val {
            ::linux::result::Result::Ok(v) => v,
            ::linux::result::Result::Err(e) => return ::linux::result::Result::Err(e),
        }
    }
}

/// Prints a value to stdout.
///
/// Note that stdout always refers to the file descriptor `1`.
#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        write!(::linux::fd::STDOUT, concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        write!(::linux::fd::STDOUT, concat!($fmt, "\n"), $($arg)*)
    };
}

/// Formats a value into a `ByteString`.
#[macro_export]
macro_rules! format {
    ($fmt:expr, $($arg:tt)*) => {{
        let mut vec = Vec::new();
        write!(vec, $fmt, $($arg)*);
        ::linux::string::ByteString::from_vec(vec)
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
        ::linux::vec::Vec::from_elem($elem, $n)
    };
    ($($x:expr),*) => {
        {
            let base = [$($x),*];
            let mut vec = ::linux::vec::Vec::with_capacity(base.len()).unwrap();
            unsafe {
                vec.unsafe_push_all(&base[..]);
                ::linux::mem::forget(base);
            }
            vec
        }
    };
    ($($x:expr,)*) => { vec!($($x),*) };
}

#[macro_export]
macro_rules! rv {
    ($x:expr) => {
        if $x < 0 {
            ::linux::result::Result::Err(::linux::error::Errno(-$x as ::linux::cty::c_int))
        } else {
            ::linux::result::Result::Ok(())
        }
    };
    ($x:expr, -> $t:ty) => {
        if $x < 0 {
            ::linux::result::Result::Err(::linux::error::Errno(-$x as ::linux::cty::c_int))
        } else {
            ::linux::result::Result::Ok($x as $t)
        }
    };
}
