// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
#[cfg(target_endian = "little")]
fn as_ref() {
    test!(1u8.as_ref() == &[1][..]);
    test!(1u16.as_ref() == &[1, 0][..]);
    test!(1u32.as_ref() == &[1, 0, 0, 0][..]);
    test!(1u64.as_ref() == &[1, 0, 0, 0, 0, 0, 0, 0][..]);
}

#[test]
#[cfg(target_endian = "big")]
fn as_ref() {
    test!(1u8.as_ref() == &[1][..]);
    test!(1u16.as_ref() == &[0, 1][..]);
    test!(1u32.as_ref() == &[0, 0, 0, 1][..]);
    test!(1u64.as_ref() == &[0, 0, 0, 0, 0, 0, 0, 1][..]);
}

#[test]
#[cfg(target_endian = "little")]
fn as_mut() {
    test!(1u8.as_mut() == &[1][..]);
    test!(1u16.as_mut() == &[1, 0][..]);
    test!(1u32.as_mut() == &[1, 0, 0, 0][..]);
    test!(1u64.as_mut() == &[1, 0, 0, 0, 0, 0, 0, 0][..]);
}

#[test]
#[cfg(target_endian = "big")]
fn as_mut() {
    test!(1u8.as_mut() == &[1][..]);
    test!(1u16.as_mut() == &[0, 1][..]);
    test!(1u32.as_mut() == &[0, 0, 0, 1][..]);
    test!(1u64.as_mut() == &[0, 0, 0, 0, 0, 0, 0, 1][..]);
}
