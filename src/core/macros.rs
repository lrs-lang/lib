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

#[macro_export]
macro_rules! writeln {
    ($w:expr, $fmt:expr) => {
        write!($w, concat!($fmt, "\n"))
    };
    ($w:expr, $fmt:expr, $($arg:tt)*) => {
        write!($w, concat!($fmt, "\n"), $($arg)*)
    };
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

/// Formats a value into a `Vec<u8, H>`.
#[macro_export]
macro_rules! format {
    ($fmt:expr, $($arg:tt)*) => {{
        let mut vec = Vec::new();
        write!(vec, $fmt, $($arg)*).map(|_| vec)
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

#[macro_export]
macro_rules! offset_of {
    ($t:ty, $($field:ident).+) => {{
        let x: $t = unsafe { std::mem::uninitialized() };
        &x$(.$field)+ as *const _ as usize - &x as *const _ as usize
    }}
}

#[macro_export]
#[allow_internal_unstable]
macro_rules! thread_local {
    ($(static $name:ident: $t:ty = $init:expr;)+) => {
        $(
            #[thread_local] static $name: ::std::share::__ThreadLocal<$t> =
                ::std::share::__ThreadLocal::new($init);
        )+
    };
    ($(pub static $name:ident: $t:ty = $init:expr;)+) => {
        $(
            #[thread_local] pub static $name: ::std::share::__ThreadLocal<$t> =
                ::std::share::__ThreadLocal::new($init);
        )+
    }
}

#[macro_export]
macro_rules! impl_try_as_ref {
    ($target:ty, $source:ty) => {
        impl ::std::conv::TryAsRef<$target> for $source {
            fn try_as_ref(&self) -> ::std::result::Result<&$target> {
                ::std::result::Result::Ok(self.as_ref())
            }
        }
    }
}

#[macro_export]
macro_rules! impl_try_as_mut {
    ($target:ty, $source:ty) => {
        impl ::std::conv::TryAsMut<$target> for $source {
            fn try_as_mut(&mut self) -> ::std::result::Result<&mut $target> {
                ::std::result::Result::Ok(self.as_mut())
            }
        }
    }
}

#[macro_export]
macro_rules! impl_try_to {
    ($target:ty, $source:ty) => {
        impl ::std::conv::TryTo<$target> for $source {
            fn try_to(&mut self) -> ::std::result::Result<&mut $target> {
                ::std::result::Result::Ok(self.to())
            }
        }
    }
}
