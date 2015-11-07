// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::string::{AsByteStr, AsNoNullStr, ToCStr};

#[test]
fn trim() {
    let x = " abc\t ".as_byte_str();
    test!(x.trim() == "abc");
}

#[test]
fn starts_with() {
    let x = "abc".as_byte_str();
    test!(x.starts_with("ab"));
    test!(!x.starts_with("c"));
}

#[test]
fn as_no_null_str() {
    test!("abc".as_byte_str().as_no_null_str().unwrap() == "abc");
    test!("ab\0c".as_byte_str().as_no_null_str().is_err());
}

#[test]
fn to_c_str() {
    let mut buf = [0; 4];
    test!("abc".as_byte_str().to_cstr(&mut buf).unwrap() == "abc");
}

#[test]
fn debug() {
    let mut buf = [0; 32];
    let mut buf = Vec::buffered(&mut buf);
    write!(&mut buf, "{:?}", b"abc\xff".as_byte_str());
    test!(&*buf == "\"abc\\xff\"");
}
