// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{OncePool};
use std::string::{ByteStr, NoNullStr};

#[test]
fn trim() {
    let x: &ByteStr = " abc\t ".as_ref();
    test!(x.trim() == "abc");
}

#[test]
fn starts_with() {
    let x: &ByteStr = "abc".as_ref();
    test!(x.starts_with("ab"));
    test!(!x.starts_with("c"));
}

#[test]
fn as_no_null_str() {
    test!(("abc".as_ref():&ByteStr).try_as_ref().unwrap():&NoNullStr == "abc");
    test!((("ab\0c".as_ref():&ByteStr).try_as_ref():Result<&NoNullStr>).is_err())
}

#[test]
fn debug() {
    let mut buf = [0; 32];
    let mut buf = Vec::with_pool(OncePool::new(buf.as_mut()));
    write!(&mut buf, "{:?}", b"abc\xff"[..].as_ref():&ByteStr);
    test!(&*buf == "\"abc\\xff\"");
}
