// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::ops::{Range, RangeTo, RangeFrom, RangeFull};

pub trait OutOf<T = ()> {
    fn out_of(t: T) -> Self;
}

impl<T> OutOf<T> for T {
    fn out_of(t: T) -> T {
        t
    }
}

impl<T> OutOf<Range<T>> for Range<Option<T>> {
    fn out_of(u: Range<T>) -> Self {
        Range { start: Some(u.start), end: Some(u.end) }
    }
}

impl<T> OutOf<RangeTo<T>> for Range<Option<T>> {
    fn out_of(u: RangeTo<T>) -> Self {
        Range { start: None, end: Some(u.end) }
    }
}

impl<T> OutOf<RangeFrom<T>> for Range<Option<T>> {
    fn out_of(u: RangeFrom<T>) -> Self {
        Range { start: Some(u.start), end: None }
    }
}

impl<T> OutOf<RangeFull> for Range<Option<T>> {
    fn out_of(_: RangeFull) -> Self {
        Range { start: None, end: None }
    }
}

macro_rules! as_out_of {
    ($from:ty as $to:ty) => {
        impl OutOf<$from> for $to {
            fn out_of(from: $from) -> $to { from as $to }
        }
    }
}

as_out_of!(u8  as u16);
as_out_of!(u8  as u32);
as_out_of!(u8  as u64);
as_out_of!(u8  as usize);
as_out_of!(u16 as u32);
as_out_of!(u16 as u64);
as_out_of!(u16 as usize);
as_out_of!(u32 as u64);
as_out_of!(u32 as usize);
as_out_of!(u8  as i16);
as_out_of!(u8  as i32);
as_out_of!(u8  as i64);
as_out_of!(u8  as isize);
as_out_of!(u16 as i32);
as_out_of!(u16 as i64);
as_out_of!(u16 as isize);
as_out_of!(u32 as i64);
as_out_of!(u32 as isize);
as_out_of!(i8  as i16);
as_out_of!(i8  as i32);
as_out_of!(i8  as i64);
as_out_of!(i8  as isize);
as_out_of!(i16 as i32);
as_out_of!(i16 as i64);
as_out_of!(i16 as isize);
as_out_of!(i32 as i64);
as_out_of!(i32 as isize);

macro_rules! zero {
    ($ty:ty) => {
        impl OutOf for $ty {
            fn out_of(_: ()) -> $ty {
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

impl<T> OutOf for Option<T> {
    fn out_of(_: ()) -> Option<T> {
        None
    }
}
