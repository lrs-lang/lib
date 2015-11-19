// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Aborts the process.
#[macro_export]
macro_rules! abort {
    () => { ::std::intrinsics::lrs_abort() }
}

/// Asserts that a condition is satisfied. Aborts the process otherwise.
#[macro_export]
macro_rules! assert {
    ($pred:expr) => { if !$pred { ::std::intrinsics::lrs_abort() } }
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
            ::std::result::Result::Ok(v) => v,
            ::std::result::Result::Err(e) => return ::std::result::Result::Err(e),
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
macro_rules! print {
    ($fmt:expr) => {
        write!(::std::fd::STDOUT, $fmt)
    };
    ($fmt:expr, $($arg:tt)*) => {
        write!(::std::fd::STDOUT, $fmt, $($arg)*)
    };
}

/// Prints a value to stdout.
///
/// Note that stdout always refers to the file descriptor `1`.
#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        write!(::std::fd::STDOUT, concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        write!(::std::fd::STDOUT, concat!($fmt, "\n"), $($arg)*)
    };
}

/// Like `print` but write to stder.
#[macro_export]
macro_rules! err {
    ($fmt:expr) => {
        write!(::std::fd::STDERR, $fmt)
    };
    ($fmt:expr, $($arg:tt)*) => {
        write!(::std::fd::STDERR, $fmt, $($arg)*)
    };
}

/// Like `println` but write to stder.
#[macro_export]
macro_rules! errln {
    ($fmt:expr) => {
        write!(::std::fd::STDERR, concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        write!(::std::fd::STDERR, concat!($fmt, "\n"), $($arg)*)
    };
}

/// Formats a value into a `ByteString`.
#[macro_export]
macro_rules! format {
    ($fmt:expr, $($arg:tt)*) => {{
        let mut vec = Vec::new();
        write!(vec, $fmt, $($arg)*);
        ::std::string::ByteString::from_vec(vec)
    }};
}

#[macro_export]
macro_rules! matches {
    ($val:expr => $($pat:tt)+) => {
        match $val { $($pat)+ => true, _ => false, }
    }
}

/// Creates a vector out of the arguments.
#[macro_export]
macro_rules! vec {
    ($elem:expr; $n:expr) => {
        ::std::vec::Vec::from_elem($elem, $n)
    };
    ($($x:expr),*) => {
        {
            let base = [$($x),*];
            let mut vec = ::std::vec::Vec::with_capacity(base.len()).unwrap();
            unsafe {
                vec.try_unsafe_push_all(&base[..]).unwrap();
                ::std::mem::unsafe_forget(base);
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
            ::std::result::Result::Err(::std::error::Errno(-val as ::std::cty::c_int))
        } else {
            ::std::result::Result::Ok(())
        }
    }};
    ($x:expr, -> $t:ty) => {{
        let val = $x;
        if val < 0 {
            ::std::result::Result::Err(::std::error::Errno(-val as ::std::cty::c_int))
        } else {
            ::std::result::Result::Ok(val as $t)
        }
    }};
}

#[macro_export]
macro_rules! align {
    // Rounds $val up so that align_up!($val, [%] $to) has $to alignment. The rv is in the
    // range [$val, $val+$to).
    ($val:expr, [%] $to:expr) => {{
        let val = $val;
        let mask = $to - 1;
        (val + mask) & !mask
    }};
    // Rounds $val up so that align_up!($val, [+] $with, [%] $to) + $with has $to
    // alignment. The rv is in the range [$val, $val+$to).
    ($val:expr, [+] $with:expr, [%] $to:expr) => {{
        let val = $val;
        let with = $with;
        let to = $to;
        let mask = to - 1;
        align!(val + (with & mask), [%] to) - (with & mask)
    }}
}
