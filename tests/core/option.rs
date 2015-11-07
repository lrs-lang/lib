// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn take() {
    test!(Some(0).take() == Some(0));
    test!(None::<u8>.take() == None);
    let mut x = Some(0);
    x.take();
    test!(x == None);
}

#[test]
fn map() {
    test!(Some(0).map(|x| x+1) == Some(1));
    test!(None::<u8>.map(|_| abort!()) == None);
}

#[test]
fn unwrap() {
    test!(Some(0).unwrap() == 0);
}

#[test]
#[should_panic]
fn unwrap_none() {
    None::<u8>.unwrap();
}

#[test]
fn unwrap_or() {
    test!(Some(0).unwrap_or(1) == 0);
    test!(None.unwrap_or(1) == 1);
}

#[test]
fn chain() {
    test!(Some(0).chain(|x| Some(x+1)) == Some(1));
    test!(None::<u8>.chain(|_| abort!()) == None::<u64>);
}

#[test]
fn as_ref() {
    test!(Some(0).as_ref() == Some(&0));
}

#[test]
fn as_mut() {
    test!(Some(0).as_mut() == Some(&mut 0));
}

#[test]
fn is_some() {
    test!(Some(0).is_some());
    test!(!None::<u8>.is_some());
}

#[test]
fn is_none() {
    test!(!Some(0).is_none());
    test!(None::<u8>.is_none());
}
