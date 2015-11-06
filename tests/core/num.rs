// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn wrapping() {
    assert!(255u8.wrapping_add(1) == 0);
    assert!(0u8.wrapping_sub(1) == 255);
    assert!(128u8.wrapping_mul(2) == 0);
}

#[test]
fn checked() {
    assert!(255u8.checked_add(1) == None);
    assert!(254u8.checked_add(1) == Some(255));
    assert!(0u8.checked_sub(1) == None);
    assert!(1u8.checked_sub(1) == Some(0));
    assert!(128u8.checked_mul(2) == None);
    assert!(127u8.checked_mul(2) == Some(254));
}

#[test]
fn saturating() {
    assert!(255u8.saturating_add(1) == 255);
    assert!(254u8.saturating_add(1) == 255);
    assert!(0u8.saturating_sub(1) == 0);
    assert!(1u8.saturating_sub(1) == 0);
}

#[test]
fn next_power_of_two() {
    assert!(0u8.next_power_of_two() == 1);
    assert!(1u8.next_power_of_two() == 1);
    assert!(2u8.next_power_of_two() == 2);
    assert!(3u8.next_power_of_two() == 4);
    assert!(128u8.next_power_of_two() == 128);
    assert!(129u8.next_power_of_two() == 1);

    assert!(0u8.checked_next_power_of_two() == Some(1));
    assert!(1u8.checked_next_power_of_two() == Some(1));
    assert!(2u8.checked_next_power_of_two() == Some(2));
    assert!(3u8.checked_next_power_of_two() == Some(4));
    assert!(128u8.checked_next_power_of_two() == Some(128));
    assert!(129u8.checked_next_power_of_two() == None);
}

#[test]
fn signed() {
    assert!(!0u8.signed());
    assert!(!0u16.signed());
    assert!(!0u32.signed());
    assert!(!0u64.signed());
    assert!(!0usize.signed());

    assert!(0i8.signed());
    assert!(0i16.signed());
    assert!(0i32.signed());
    assert!(0i64.signed());
    assert!(0isize.signed());
}

#[test]
fn as_signed() {
    assert!(255u8.as_signed() == -1);
    assert!((-1i8).as_unsigned() == 255);
}

#[test]
fn negative() {
    assert!((-1i8).negative());
    assert!(!1u8.negative());
}

#[test]
fn abs() {
    assert!((-1i8).abs() == 1);
}

#[test]
fn count_ones() {
    assert!(0b11110000u8.count_ones() == 4);
}

#[test]
fn count_zeros() {
    assert!(0b11110000u8.count_zeros() == 4);
}

#[test]
fn leading_ones() {
    assert!(0b11110000u8.leading_ones() == 4);
}

#[test]
fn leading_zeros() {
    assert!(0b11110000u8.leading_zeros() == 0);
}

#[test]
fn trailing_ones() {
    assert!(0b11110000u8.trailing_ones() == 0);
}

#[test]
fn trailing_zeros() {
    assert!(0b11110000u8.trailing_zeros() == 4);
}

#[test]
fn swap() {
    assert!(0xFF00u16.swap() == 0x00FF);
}

#[test]
fn from_be() {
    if cfg!(target_endian = "little") {
        assert!(0xFF00u16.from_be() == 0x00FF);
    } else {
        assert!(0xFF00u16.from_be() == 0xFF00);
    }
}

#[test]
fn from_le() {
    if cfg!(target_endian = "little") {
        assert!(0xFF00u16.from_le() == 0xFF00);
    } else {
        assert!(0xFF00u16.from_le() == 0x00FF);
    }
}

#[test]
fn to_be() {
    if cfg!(target_endian = "little") {
        assert!(0xFF00u16.to_be() == 0x00FF);
    } else {
        assert!(0xFF00u16.to_be() == 0xFF00);
    }
}

#[test]
fn to_le() {
    if cfg!(target_endian = "little") {
        assert!(0xFF00u16.to_le() == 0xFF00);
    } else {
        assert!(0xFF00u16.to_le() == 0x00FF);
    }
}

#[test]
fn div_rem() {
    assert!(3u8.div_rem(2) == (1, 1));
}

#[test]
fn rotate_right() {
    assert!(0b1u8.rotate_right(1) == 0b1000_0000);
}

#[test]
fn rotate_left() {
    assert!(0b1u8.rotate_left(1) == 0b10);
}

#[test]
fn min() {
    assert!(u8::min() == 0);
    assert!(u64::min() == 0);
    assert!(i8::min() == -128);
}

#[test]
fn max() {
    assert!(u8::max() == 255);
    assert!(u64::max() == 0xFFFFFFFFFFFFFFFF);
    assert!(i8::max() == 127);
}

#[test]
fn bits() {
    assert!(u8::bits() == 8);
    assert!(u64::bits() == 64);
}

#[test]
fn bytes() {
    assert!(u8::bytes() == 1);
    assert!(u64::bytes() == 8);
}
