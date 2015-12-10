// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn zero() {
    test!(u8::default() == 0);
    test!(u16::default() == 0);
    test!(u32::default() == 0);
    test!(u64::default() == 0);
    test!(usize::default() == 0);
    test!(i8::default() == 0);
    test!(i16::default() == 0);
    test!(i32::default() == 0);
    test!(i64::default() == 0);
    test!(isize::default() == 0);
}

#[test]
fn option() {
    test!(Option::<u8>::default() == None);
}
