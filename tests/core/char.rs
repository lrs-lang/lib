// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn len() {
    assert!('a'.len() == 1);
    assert!('ä'.len() == 2);
    assert!('ᄌ'.len() == 3);
}

#[test]
fn to_utf8() {
    fn inner(c: char, s: &str) {
        let bytes = c.to_utf8();
        let len = c.len();
        assert!(&bytes[..len] == s);
    }
    inner('a', "a");
    inner('ä', "ä");
    inner('ᄌ', "ᄌ");
}

#[test]
fn from_u32() {
    assert!(char::from_u32('a' as u32) == Some('a'));
    assert!(char::from_u32('ä' as u32) == Some('ä'));
    assert!(char::from_u32(0x110000) == None);
    assert!(char::from_u32(0xd800) == None);
}

#[test]
fn max() {
    assert!(char::max() == '\u{10ffff}');
}

#[test]
fn eq() {
    assert!('a' == 'a');
    assert!('a' != 'b');
}

#[test]
fn ord() {
    assert!('a' < 'b');
    assert!('a' < 'ä');
}
