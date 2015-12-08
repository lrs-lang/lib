// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
#[cfg(target_endian = "little")]
fn as_ref() {
    test!(AsRef::<[u8]>::as_ref(&1u8)  == &[1][..]);
    test!(AsRef::<[u8]>::as_ref(&1u16) == &[1, 0][..]);
    test!(AsRef::<[u8]>::as_ref(&1u32) == &[1, 0, 0, 0][..]);
    test!(AsRef::<[u8]>::as_ref(&1u64) == &[1, 0, 0, 0, 0, 0, 0, 0][..]);
}

#[test]
#[cfg(target_endian = "big")]
fn as_ref() {
    test!(AsRef::<[u8]>::as_ref(&1u8)  == &[1][..]);
    test!(AsRef::<[u8]>::as_ref(&1u16) == &[0, 1][..]);
    test!(AsRef::<[u8]>::as_ref(&1u32) == &[0, 0, 0, 1][..]);
    test!(AsRef::<[u8]>::as_ref(&1u64) == &[0, 0, 0, 0, 0, 0, 0, 1][..]);
}

#[test]
#[cfg(target_endian = "little")]
fn as_mut() {
    test!(AsMut::<[u8]>::as_mut(&mut 1u8)  == &[1][..]);
    test!(AsMut::<[u8]>::as_mut(&mut 1u16) == &[1, 0][..]);
    test!(AsMut::<[u8]>::as_mut(&mut 1u32) == &[1, 0, 0, 0][..]);
    test!(AsMut::<[u8]>::as_mut(&mut 1u64) == &[1, 0, 0, 0, 0, 0, 0, 0][..]);
}

#[test]
#[cfg(target_endian = "big")]
fn as_mut() {
    test!(AsMut::<[u8]>::as_mut(&mut 1u8)  == &[1][..]);
    test!(AsMut::<[u8]>::as_mut(&mut 1u16) == &[0, 1][..]);
    test!(AsMut::<[u8]>::as_mut(&mut 1u32) == &[0, 0, 0, 1][..]);
    test!(AsMut::<[u8]>::as_mut(&mut 1u64) == &[0, 0, 0, 0, 0, 0, 0, 1][..]);
}
