// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn unwrap() {
    test!(Ok::<_, ()>(1).unwrap() == 1);
}

#[test]
#[should_panic]
fn unwrap_err() {
    Err::<(), _>(()).unwrap();
}

#[test]
fn map() {
    test!(Ok::<_, ()>(0).map(|x| x+1) == Ok(1));
    test!(Err::<(), _>(()).map(|_| abort!()) == Err(()));
}

#[test]
fn chain() {
    test!(Ok::<_, ()>(0).chain(|x| Ok(x+1)) == Ok(1));
    test!(Err::<(), _>(()).chain::<(), _>(|_| abort!()) == Err(()));
}

#[test]
fn is_ok() {
    test!(Ok::<_, ()>(0).is_ok());
    test!(!Err::<(), _>(()).is_ok());
}

#[test]
fn is_err() {
    test!(!Ok::<_, ()>(0).is_err());
    test!(Err::<(), _>(()).is_err());
}

#[test]
fn ignore_ok() {
    test!(Ok::<_, ()>(0).ignore_ok() == Ok(()));
    test!(Err::<(), _>(0).ignore_ok() == Err(0));
}
