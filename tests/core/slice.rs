// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{slice};

#[test]
fn from_ptr() {
    let x: &[u8] = &[1];
    let y: &[u8] = unsafe { slice::from_ptr(x.as_ptr(), 1) };
    test!(x == y);
}

#[test]
fn len() {
    let x: &[u8] = &[];
    test!(x.len() == 0);
    test!([0].len() == 1);
    test!([0, 0].len() == 2);
}

#[test]
fn as_ptr() {
    let mut x: &mut [u8] = &mut [1];
    test!(x.as_ptr() == &x[0]);
    test!(x.as_mut_ptr() == &mut x[0]);
}

#[test]
fn addr() {
    let x: &[u8] = &[1];
    test!(x.addr() == x.as_ptr() as usize);
}

#[test]
fn as_data() {
    let mut x = &mut [0u64; 2];
    test!(x.as_data().len() == 16);
    test!(x.as_mut_data().len() == 16);
}

#[test]
fn iter() {
    let x = [0, 1];
    let mut x = x.iter();
    test!(x.next().unwrap() == &0);
    test!(x.next().unwrap() == &1);
    test!(x.next() == None);

    let mut x = [0, 1];
    let mut x = x.iter_mut();
    test!(x.next().unwrap() == &mut 0);
    test!(x.next().unwrap() == &mut 1);
    test!(x.next() == None);
}

#[test]
fn find() {
    test!([0, 1, 2].find(|x| *x == 1) == Some(1));
    test!([0, 1, 2].find(|x| *x == 3) == None);
    test!([0, 1, 1].find_reverse(|x| *x == 1) == Some(2));
    test!([0, 1, 2, 4, 5, 6].find_binary(|x| x.cmp(&3)) == (None, 3));
}

#[test]
fn last() {
    test!([0, 1].last() == Some(&1));
}

#[test]
fn split() {
    let x = &[0, 0, 1, 0, 0, 1, 0];
    let mut x = x.split(|x| *x == 1);
    test!(x.next().unwrap() == &[0, 0][..]);
    test!(x.next().unwrap() == &[0, 0][..]);
    test!(x.next().unwrap() == &[0][..]);
    test!(x.next() == None);
}

#[test]
fn split_at() {
    test!([0, 0, 0].split_at(1) == (&[0], &[0, 0]));
}

#[test]
#[should_panic]
fn split_at_abort() {
    [0, 0, 0].split_at(4);
}

#[test]
fn split_at_mut() {
    test!([0, 0, 0].split_at_mut(1) == (&mut [0], &mut [0, 0]));
}

#[test]
#[should_panic]
fn split_at_mut_abort() {
    [0, 0, 0].split_at_mut(4);
}

#[test]
fn starts_with() {
    test!([0, 1, 2].starts_with(&[0, 1]));
    test!(![0, 1, 2].starts_with(&[0, 2]));
}

#[test]
fn sort() {
    let mut x = [3, 2, 1, 0];
    x.sort();
    test!(x == [0, 1, 2, 3]);
}

#[test]
fn sort_by() {
    let mut x = [3, 2, 1, 0];
    x.sort_by(|a, b| b.cmp(a));
    test!(x == [3, 2, 1, 0]);
}

#[test]
fn unchecked_slice() {
    let mut x = [0, 1, 2, 3];
    unsafe {
        test!(x.unchecked_slice(1, 4) == &x[1..4]);
        test!(x.unchecked_mut_slice(1, 4) == &mut [1, 2, 3][..]);

        test!(x.unchecked_slice_from(1) == &x[1..]);
        test!(x.unchecked_mut_slice_from(1) == &mut [1, 2, 3][..]);

        test!(x.unchecked_slice_to(3) == &x[..3]);
        test!(x.unchecked_mut_slice_to(3) == &mut [0, 1, 2][..]);
    }
}

#[test]
fn cmp() {
    let x: &[u8] = &[1, 2, 3];
    let y: &[u8] = &[1, 2, 3];
    test!(x == y);
    test!(x[1..] == y[1..]);
    test!(x[2..] != y[..1]);
}
