// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn take() {
    assert!(Some(0).take() == Some(0));
    assert!(None::<u8>.take() == None);
    let mut x = Some(0);
    x.take();
    assert!(x == None);
}

#[test]
fn map() {
    assert!(Some(0).map(|x| x+1) == Some(1));
    assert!(None::<u8>.map(|_| abort!()) == None);
}

#[test]
fn unwrap() {
    assert!(Some(0).unwrap() == 0);
}

#[test]
#[should_panic]
fn unwrap_none() {
    None::<u8>.unwrap();
}

#[test]
fn unwrap_or() {
    assert!(Some(0).unwrap_or(1) == 0);
    assert!(None.unwrap_or(1) == 1);
}

#[test]
fn chain() {
    assert!(Some(0).chain(|x| Some(x+1)) == Some(1));
    assert!(None::<u8>.chain(|_| abort!()) == None::<u64>);
}

#[test]
fn as_ref() {
    assert!(Some(0).as_ref() == Some(&0));
}

#[test]
fn as_mut() {
    assert!(Some(0).as_mut() == Some(&mut 0));
}

#[test]
fn is_some() {
    assert!(Some(0).is_some());
    assert!(!None::<u8>.is_some());
}

#[test]
fn is_none() {
    assert!(!Some(0).is_none());
    assert!(None::<u8>.is_none());
}
