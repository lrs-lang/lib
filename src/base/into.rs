// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use core::ops::{Range, RangeTo, RangeFrom, RangeFull};
use core::option::{Option};
use core::option::Option::{None};

/// Objects that can be turned into other objects.
pub trait Into<T> {
    /// Turns the object into another object.
    fn into(self) -> T;
}

impl<T> Into<T> for T {
    fn into(self) -> T {
        self
    }
}

impl<T> Into<Range<Option<T>>> for RangeTo<T> {
    fn into(self) -> Range<Option<T>> {
        Range { start: None, end: Some(self.end) }
    }
}

impl<T> Into<Range<Option<T>>> for RangeFrom<T> {
    fn into(self) -> Range<Option<T>> {
        Range { start: Some(self.start), end: None }
    }
}

impl<T> Into<Range<Option<T>>> for RangeFull {
    fn into(self) -> Range<Option<T>> {
        Range { start: None, end: None }
    }
}

macro_rules! as_into {
    ($from:ty as $to:ty) => {
        impl Into<$to> for $from {
            fn into(self) -> $to { self as $to }
        }
    }
}

as_into!(u8  as u16);
as_into!(u8  as u32);
as_into!(u8  as u64);
as_into!(u8  as usize);
as_into!(u16 as u32);
as_into!(u16 as u64);
as_into!(u16 as usize);
as_into!(u32 as u64);
as_into!(u32 as usize);
as_into!(u8  as i16);
as_into!(u8  as i32);
as_into!(u8  as i64);
as_into!(u8  as isize);
as_into!(u16 as i32);
as_into!(u16 as i64);
as_into!(u16 as isize);
as_into!(u32 as i64);
as_into!(u32 as isize);
as_into!(i8  as i16);
as_into!(i8  as i32);
as_into!(i8  as i64);
as_into!(i8  as isize);
as_into!(i16 as i32);
as_into!(i16 as i64);
as_into!(i16 as isize);
as_into!(i32 as i64);
as_into!(i32 as isize);

macro_rules! zero {
    ($ty:ty) => {
        impl Into<$ty> for () {
            fn into(self) -> $ty {
                0
            }
        }
    }
}

zero!(u8    );
zero!(u16   );
zero!(u32   );
zero!(u64   );
zero!(usize );
zero!(i8    );
zero!(i16   );
zero!(i32   );
zero!(i64   );
zero!(isize );

impl<T> Into<Option<T>> for () {
    fn into(self) -> Option<T> {
        None
    }
}
