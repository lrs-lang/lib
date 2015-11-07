// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::{Range};

#[test]
fn range() {
    test!(Into::<Range<Option<u8>>>::into(..) == (None..None));
    test!(Into::<Range<Option<u8>>>::into(1..) == (Some(1)..None));
    test!(Into::<Range<Option<u8>>>::into(..2) == (None..Some(2)));
}

#[test]
fn ints() {
    macro_rules! as_into {
        ($from:ty as $to:ty) => {
            test!(Into::<$to>::into(1 as $from) == 1);
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
}

#[test]
fn zero() {
    test!(Into::<u8>::into(()) == 0);
    test!(Into::<u16>::into(()) == 0);
    test!(Into::<u32>::into(()) == 0);
    test!(Into::<u64>::into(()) == 0);
    test!(Into::<usize>::into(()) == 0);
    test!(Into::<i8>::into(()) == 0);
    test!(Into::<i16>::into(()) == 0);
    test!(Into::<i32>::into(()) == 0);
    test!(Into::<i64>::into(()) == 0);
    test!(Into::<isize>::into(()) == 0);
}

#[test]
fn option() {
    test!(Into::<Option<u8>>::into(()) == None);
}
