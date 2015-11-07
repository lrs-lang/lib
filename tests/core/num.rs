// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn wrapping() {
    test!(255u8.wrapping_add(1) == 0);
    test!(0u8.wrapping_sub(1) == 255);
    test!(128u8.wrapping_mul(2) == 0);
}

#[test]
fn checked() {
    test!(255u8.checked_add(1) == None);
    test!(254u8.checked_add(1) == Some(255));
    test!(0u8.checked_sub(1) == None);
    test!(1u8.checked_sub(1) == Some(0));
    test!(128u8.checked_mul(2) == None);
    test!(127u8.checked_mul(2) == Some(254));
}

#[test]
fn saturating() {
    test!(255u8.saturating_add(1) == 255);
    test!(254u8.saturating_add(1) == 255);
    test!(0u8.saturating_sub(1) == 0);
    test!(1u8.saturating_sub(1) == 0);
}

#[test]
fn next_power_of_two() {
    test!(0u8.next_power_of_two() == 1);
    test!(1u8.next_power_of_two() == 1);
    test!(2u8.next_power_of_two() == 2);
    test!(3u8.next_power_of_two() == 4);
    test!(128u8.next_power_of_two() == 128);
    test!(129u8.next_power_of_two() == 1);

    test!(0u8.checked_next_power_of_two() == Some(1));
    test!(1u8.checked_next_power_of_two() == Some(1));
    test!(2u8.checked_next_power_of_two() == Some(2));
    test!(3u8.checked_next_power_of_two() == Some(4));
    test!(128u8.checked_next_power_of_two() == Some(128));
    test!(129u8.checked_next_power_of_two() == None);
}

#[test]
fn signed() {
    test!(!0u8.signed());
    test!(!0u16.signed());
    test!(!0u32.signed());
    test!(!0u64.signed());
    test!(!0usize.signed());

    test!(0i8.signed());
    test!(0i16.signed());
    test!(0i32.signed());
    test!(0i64.signed());
    test!(0isize.signed());
}

#[test]
fn as_signed() {
    test!(255u8.as_signed() == -1);
    test!((-1i8).as_unsigned() == 255);
}

#[test]
fn negative() {
    test!((-1i8).negative());
    test!(!1u8.negative());
}

#[test]
fn abs() {
    test!((-1i8).abs() == 1);
}

#[test]
fn count_ones() {
    test!(0b11110000u8.count_ones() == 4);
}

#[test]
fn count_zeros() {
    test!(0b11110000u8.count_zeros() == 4);
}

#[test]
fn leading_ones() {
    test!(0b11110000u8.leading_ones() == 4);
}

#[test]
fn leading_zeros() {
    test!(0b11110000u8.leading_zeros() == 0);
}

#[test]
fn trailing_ones() {
    test!(0b11110000u8.trailing_ones() == 0);
}

#[test]
fn trailing_zeros() {
    test!(0b11110000u8.trailing_zeros() == 4);
}

#[test]
fn swap() {
    test!(0xFF00u16.swap() == 0x00FF);
}

#[test]
fn from_be() {
    if cfg!(target_endian = "little") {
        test!(0xFF00u16.from_be() == 0x00FF);
    } else {
        test!(0xFF00u16.from_be() == 0xFF00);
    }
}

#[test]
fn from_le() {
    if cfg!(target_endian = "little") {
        test!(0xFF00u16.from_le() == 0xFF00);
    } else {
        test!(0xFF00u16.from_le() == 0x00FF);
    }
}

#[test]
fn to_be() {
    if cfg!(target_endian = "little") {
        test!(0xFF00u16.to_be() == 0x00FF);
    } else {
        test!(0xFF00u16.to_be() == 0xFF00);
    }
}

#[test]
fn to_le() {
    if cfg!(target_endian = "little") {
        test!(0xFF00u16.to_le() == 0xFF00);
    } else {
        test!(0xFF00u16.to_le() == 0x00FF);
    }
}

#[test]
fn div_rem() {
    test!(3u8.div_rem(2) == (1, 1));
}

#[test]
fn rotate_right() {
    test!(0b1u8.rotate_right(1) == 0b1000_0000);
}

#[test]
fn rotate_left() {
    test!(0b1u8.rotate_left(1) == 0b10);
}

#[test]
fn min() {
    test!(u8::min() == 0);
    test!(u64::min() == 0);
    test!(i8::min() == -128);
}

#[test]
fn max() {
    test!(u8::max() == 255);
    test!(u64::max() == 0xFFFFFFFFFFFFFFFF);
    test!(i8::max() == 127);
}

#[test]
fn bits() {
    test!(u8::bits() == 8);
    test!(u64::bits() == 64);
}

#[test]
fn bytes() {
    test!(u8::bytes() == 1);
    test!(u64::bytes() == 8);
}
