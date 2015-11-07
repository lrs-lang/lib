// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::parse::{Parsable};

#[test]
fn binary() {
    test!(u8::parse_bytes(b"0b101010").unwrap() == 0b101010);
    test!(i8::parse_bytes(b"-0b101010").unwrap() == -0b101010);
    test!(u64::parse_bytes(b"0b101010").unwrap() == 0b101010);
    test!(i64::parse_bytes(b"-0b101010").unwrap() == -0b101010);
}

#[test]
fn octal() {
    test!(u8::parse_bytes(b"0o310").unwrap() == 0o310);
    test!(i8::parse_bytes(b"-0o110").unwrap() == -0o110);
    test!(u64::parse_bytes(b"0o101010").unwrap() == 0o101010);
    test!(i64::parse_bytes(b"-0o101010").unwrap() == -0o101010);
}

#[test]
fn decimal() {
    test!(u8::parse_bytes(b"10").unwrap() == 10);
    test!(i8::parse_bytes(b"-10").unwrap() == -10);
    test!(u64::parse_bytes(b"101010").unwrap() == 101010);
    test!(i64::parse_bytes(b"-101010").unwrap() == -101010);
}

#[test]
fn hex() {
    test!(u8::parse_bytes(b"0x10").unwrap() == 0x10);
    test!(i8::parse_bytes(b"-0x10").unwrap() == -0x10);
    test!(u64::parse_bytes(b"0x101010").unwrap() == 0x101010);
    test!(i64::parse_bytes(b"-0x101010").unwrap() == -0x101010);
}

#[test]
fn binary_init() {
    test!(u8::parse_bytes_init(b"0b101010 ").unwrap() == (0b101010, 8));
    test!(i8::parse_bytes_init(b"-0b101010 ").unwrap() == (-0b101010, 9));
    test!(u64::parse_bytes_init(b"0b101010 ").unwrap() == (0b101010, 8));
    test!(i64::parse_bytes_init(b"-0b101010 ").unwrap() == (-0b101010, 9));
}

#[test]
fn octal_init() {
    test!(u8::parse_bytes_init(b"0o310 ").unwrap() == (0o310, 5));
    test!(i8::parse_bytes_init(b"-0o110 ").unwrap() == (-0o110, 6));
    test!(u64::parse_bytes_init(b"0o101010 ").unwrap() == (0o101010, 8));
    test!(i64::parse_bytes_init(b"-0o101010 ").unwrap() == (-0o101010, 9));
}

#[test]
fn decimal_init() {
    test!(u8::parse_bytes_init(b"10 ").unwrap() == (10, 2));
    test!(i8::parse_bytes_init(b"-10 ").unwrap() == (-10, 3));
    test!(u64::parse_bytes_init(b"101010 ").unwrap() == (101010, 6));
    test!(i64::parse_bytes_init(b"-101010 ").unwrap() == (-101010, 7));
}

#[test]
fn hex_init() {
    test!(u8::parse_bytes_init(b"0x10 ").unwrap() == (0x10, 4));
    test!(i8::parse_bytes_init(b"-0x10 ").unwrap() == (-0x10, 5));
    test!(u64::parse_bytes_init(b"0x101010 ").unwrap() == (0x101010, 8));
    test!(i64::parse_bytes_init(b"-0x101010 ").unwrap() == (-0x101010, 9));
}
