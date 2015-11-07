// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn as_ptr() {
    unsafe {
        test!(*"a".as_ptr() == b'a');
        test!(*"b".as_ptr() == b'b');
    }
}

#[test]
fn as_bytes() {
    test!("abcd".as_bytes() == &b"abcd"[..]);
}

#[test]
fn len() {
    test!("a".len() == 1);
    test!("ä".len() == 2);
    test!("日".len() == 3);
}

#[test]
fn from_bytes() {
    test!(str::from_bytes("aä日".as_bytes()) == Some("aä日"));
    test!(str::from_bytes(&[128]) == None);
    test!(str::from_bytes(&[192, 128]) == None);
    test!(str::from_bytes(&[193, 128]) == None);
    test!(str::from_bytes(&[194, 0]) == None);
    test!(str::from_bytes(&[194, 128]) == Some("\u{80}"));
}

#[test]
fn chars_len() {
    let mut x = "aä日".chars_len();
    test!(x.next().unwrap() == ('a', 1));
    test!(x.next().unwrap() == ('ä', 2));
    test!(x.next().unwrap() == ('日', 3));
    test!(x.next() == None);
}

#[test]
fn starts_with() {
    test!("aä日".starts_with(""));
    test!("aä日".starts_with("a"));
    test!("aä日".starts_with("aä"));
    test!("aä日".starts_with("aä日"));
    test!(!"aä日".starts_with("aä日c"));
    test!(!"aä日".starts_with("c"));
}

#[test]
fn is_char_boundary() {
    test!("aä日".is_char_boundary(0));
    test!("aä日".is_char_boundary(1));
    test!(!"aä日".is_char_boundary(2));
    test!("aä日".is_char_boundary(3));
    test!(!"aä日".is_char_boundary(4));
    test!(!"aä日".is_char_boundary(5));
    test!("aä日".is_char_boundary(6));
}

#[test]
fn index() {
    test!(&"aä日"[..] == "aä日");
    test!(&"aä日"[..3] == "aä");
    test!(&"aä日"[..1] == "a");
    test!(&"aä日"[..0] == "");
    test!(&"aä日"[1..] == "ä日");
    test!(&"aä日"[1..3] == "ä");
    test!(&"aä日"[1..1] == "");
    test!(&"aä日"[3..] == "日");
    test!(&"aä日"[3..3] == "");
    test!(&"aä日"[6..] == "");
}

#[test]
fn iter() {
    let mut x = "aä日";
    test!(x.next().unwrap() == 'a');
    test!(x.next().unwrap() == 'ä');
    test!(x.next().unwrap() == '日');
    test!(x.next() == None);
}

#[test]
fn longest_sequence() {
    let x = b"a\xc3\xa4\xe6\x97\xa5\xFF";
    test!(str::longest_sequence(x) == "aä日");
}
