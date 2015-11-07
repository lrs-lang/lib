// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn len() {
    test!('a'.len() == 1);
    test!('ä'.len() == 2);
    test!('ᄌ'.len() == 3);
}

#[test]
fn to_utf8() {
    fn inner(c: char, s: &str) {
        let bytes = c.to_utf8();
        let len = c.len();
        test!(&bytes[..len] == s);
    }
    inner('a', "a");
    inner('ä', "ä");
    inner('ᄌ', "ᄌ");
}

#[test]
fn from_u32() {
    test!(char::from_u32('a' as u32) == Some('a'));
    test!(char::from_u32('ä' as u32) == Some('ä'));
    test!(char::from_u32(0x110000) == None);
    test!(char::from_u32(0xd800) == None);
}

#[test]
fn max() {
    test!(char::max() == '\u{10ffff}');
}

#[test]
fn eq() {
    test!('a' == 'a');
    test!('a' != 'b');
}

#[test]
fn ord() {
    test!('a' < 'b');
    test!('a' < 'ä');
}
