// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io::{BufRead};

#[test]
fn read_u8() {
    let mut src = &mut "Hello World".as_bytes();
    let mut dst = [0; 11];
    test!(src.read(dst.as_mut()).unwrap() == dst.len());
    test!(&dst[..] == "Hello World");
}

#[test]
fn scatter_read_u8() {
    let mut src = &mut "Hello World".as_bytes();
    let mut dst1 = [0; 6];
    let mut dst2 = [0; 5];
    test!(src.scatter_read(&mut [dst1.as_mut(), dst2.as_mut()]).unwrap() == 11);
    test!(&dst1[..] == "Hello ");
    test!(&dst2[..] == "World");
}

#[test]
fn write_u8() {
    let mut dst = *b"Hello World";
    {
        let mut dst = &mut dst[..];
        test!(dst.write(b"Arrrr").unwrap() == 5);
        test!(dst == " World");
    }
    test!(&dst[..] == "Arrrr World");
}

#[test]
fn gather_write_u8() {
    let mut dst = *b"Hello World";
    {
        let mut dst = &mut dst[..];
        test!(dst.gather_write(&[b"Ar", b"rrr"]).unwrap() == 5);
        test!(dst == " World");
    }
    test!(&dst[..] == "Arrrr World");
}

#[test]
fn copy_until_u8() {
    let mut dst = [0; 6];
    let mut src = &b"Hello World"[..];
    test!(src.copy_until(&mut &mut dst[..], b' ').unwrap() == 6);
    test!(src == "World");
    test!(&dst[..] == &b"Hello "[..]);
}

#[test]
fn copy_until_u8_fail() {
    let mut dst = [0; 2];
    let mut src = &b"Hello World"[..];
    test!(src.copy_until(&mut &mut dst[..], b' ').unwrap() == 2);
    test!(src == "llo World");
    test!(&dst == b"He");
}

#[test]
fn consume() {
    let mut src = &b"Hello World"[..];
    assert!(src.consume(6) == 6);
    assert!(src == "World");
    assert!(src.consume(6) == 5);
    assert!(src == "");
}
