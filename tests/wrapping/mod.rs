// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::num::{W8, W16, W32, W64, Wsize};
use std::{mem};

#[test]
fn wrapping() {
    test!(W8(255) + 1 == 0);
    test!(W8(0) - 1 == 255);
    test!(W8(128) * 2 == 0);

    test!(W8(255).wrapping_add(1) == 0);
    test!(W8(0).wrapping_sub(1) == 255);
    test!(W8(128).wrapping_mul(2) == 0);
}

#[test]
fn checked() {
    test!(W8(255).checked_add(1) == None);
    test!(W8(254).checked_add(1) == Some(W8(255)));
    test!(W8(0).checked_sub(1) == None);
    test!(W8(1).checked_sub(1) == Some(W8(0)));
    test!(W8(128).checked_mul(2) == None);
    test!(W8(127).checked_mul(2) == Some(W8(254)));
}

#[test]
fn saturating() {
    test!(W8(255).saturating_add(1) == 255);
    test!(W8(254).saturating_add(1) == 255);
    test!(W8(0).saturating_sub(1) == 0);
    test!(W8(1).saturating_sub(1) == 0);
}

#[test]
fn next_power_of_two() {
    test!(W8(0).next_power_of_two() == 1);
    test!(W8(1).next_power_of_two() == 1);
    test!(W8(2).next_power_of_two() == 2);
    test!(W8(3).next_power_of_two() == 4);
    test!(W8(128).next_power_of_two() == 128);
    test!(W8(129).next_power_of_two() == 1);

    test!(W8(0).checked_next_power_of_two() == Some(W8(1)));
    test!(W8(1).checked_next_power_of_two() == Some(W8(1)));
    test!(W8(2).checked_next_power_of_two() == Some(W8(2)));
    test!(W8(3).checked_next_power_of_two() == Some(W8(4)));
    test!(W8(128).checked_next_power_of_two() == Some(W8(128)));
    test!(W8(129).checked_next_power_of_two() == None);
}

#[test]
fn signed() {
    test!(!W8(0).signed());
    test!(!W16(0).signed());
    test!(!W32(0).signed());
    test!(!W64(0).signed());
    test!(!Wsize(0).signed());
}

#[test]
fn count_ones() {
    test!(W8(0b11110000).count_ones() == 4);
}

#[test]
fn count_zeros() {
    test!(W8(0b11110000).count_zeros() == 4);
}

#[test]
fn leading_ones() {
    test!(W8(0b11110000).leading_ones() == 4);
}

#[test]
fn leading_zeros() {
    test!(W8(0b11110000).leading_zeros() == 0);
}

#[test]
fn trailing_ones() {
    test!(W8(0b11110000).trailing_ones() == 0);
}

#[test]
fn trailing_zeros() {
    test!(W8(0b11110000).trailing_zeros() == 4);
}

#[test]
fn swap() {
    test!(W16(0xFF00).swap() == 0x00FF);
}

#[test]
fn from_be() {
    if cfg!(target_endian = "little") {
        test!(W16(0xFF00).from_be() == 0x00FF);
    } else {
        test!(W16(0xFF00).from_be() == 0xFF00);
    }
}

#[test]
fn from_le() {
    if cfg!(target_endian = "little") {
        test!(W16(0xFF00).from_le() == 0xFF00);
    } else {
        test!(W16(0xFF00).from_le() == 0x00FF);
    }
}

#[test]
fn to_be() {
    if cfg!(target_endian = "little") {
        test!(W16(0xFF00).to_be() == 0x00FF);
    } else {
        test!(W16(0xFF00).to_be() == 0xFF00);
    }
}

#[test]
fn to_le() {
    if cfg!(target_endian = "little") {
        test!(W16(0xFF00).to_le() == 0xFF00);
    } else {
        test!(W16(0xFF00).to_le() == 0x00FF);
    }
}

#[test]
fn div_rem() {
    test!(W8(3).div_rem(2) == (W8(1), W8(1)));
}

#[test]
fn rotate_right() {
    test!(W8(0b1).rotate_right(1) == 0b1000_0000);
}

#[test]
fn rotate_left() {
    test!(W8(0b1).rotate_left(1) == 0b10);
}

#[test]
fn sizes() {
    test!(mem::size_of::<W8>() == 1);
    test!(mem::size_of::<W16>() == 2);
    test!(mem::size_of::<W32>() == 4);
    test!(mem::size_of::<W64>() == 8);
    test!(mem::size_of::<Wsize>() == mem::size_of::<usize>());
}
