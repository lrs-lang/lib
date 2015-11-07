// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::string::{CStr, AsCStr, ToCStr};
use std::cty::{c_char};

#[test]
fn from_ptr() {
    let x = b"abc\0";
    let s = unsafe { CStr::from_ptr(x.as_ptr() as *const c_char) };
    test!(s == "abc");
    test!(s.as_ptr() as usize == x.as_ptr() as usize);
    test!(s.bytes_with_null() == &x[..]);
    test!(s[1..] == "bc");
    test!(s[..1] == "a");
    test!(s[..] == "abc");
    test!(s[1] == b'b');
}

#[test]
fn empty() {
    test!(CStr::empty() == "");
}

#[test]
fn as_cstr() {
    test!("abc".as_cstr().is_err());
    test!("abc\0a".as_cstr().is_err());
    test!("abc\0\0".as_cstr().unwrap() == "abc");
    test!("abc\0".as_cstr().unwrap() == "abc");
}

#[test]
fn to_cstr() {
    let mut buf = [0; 4];
    test!("abc\0".as_cstr().unwrap().to_cstr(&mut buf).unwrap() == "abc");

    let mut buf = [0; 4];
    test!("abc\0".as_cstr().unwrap().to_or_as_cstr(&mut buf).unwrap() == "abc");
    test!(buf == [0; 4]);
}

#[test]
fn bytes_to_cstr() {
    let mut buf = [0; 4];
    test!("abc".to_cstr(&mut buf).unwrap() == "abc");
    test!("abc\0".to_cstr(&mut buf).unwrap() == "abc");
    test!("abc\0\0".to_cstr(&mut buf).unwrap() == "abc");
    test!("abc\0\0\0".to_cstr(&mut buf).unwrap() == "abc");

    let mut buf = [0; 3];
    test!("abc".to_cstr(&mut buf).is_err());
    test!("abc\0".to_cstr(&mut buf).is_err());
    test!("abc\0".to_or_as_cstr(&mut buf).unwrap() == "abc");
}
