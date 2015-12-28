// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn zero() {
    test!(u8::out_of(()) == 0);
    test!(u16::out_of(()) == 0);
    test!(u32::out_of(()) == 0);
    test!(u64::out_of(()) == 0);
    test!(usize::out_of(()) == 0);
    test!(i8::out_of(()) == 0);
    test!(i16::out_of(()) == 0);
    test!(i32::out_of(()) == 0);
    test!(i64::out_of(()) == 0);
    test!(isize::out_of(()) == 0);
}

#[test]
fn option() {
    test!(Option::<u8>::out_of(()) == None);
}
