// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::share::{RefCell, RefCellStatus};

#[test]
fn test() {
    let cell = RefCell::new(1);
    test!(cell.status() == RefCellStatus::Free);
    {
        let borrow = cell.borrow();
        test!(cell.status() == RefCellStatus::Borrowed(0));
        test!(*borrow == 1);
    }
    test!(cell.status() == RefCellStatus::Free);
    {
        let mut borrow = cell.borrow_mut();
        test!(cell.status() == RefCellStatus::BorrowedMut);
        test!(*borrow == 1);
        *borrow = 2;
    }
    {
        let borrow = cell.borrow();
        cell.borrow();
        test!(cell.status() == RefCellStatus::Borrowed(0));
        test!(*borrow == 2);
    }
}

#[test]
#[should_panic]
fn fail2() {
    let cell = RefCell::new(1);
    let _b = cell.borrow();
    cell.borrow_mut();
}

#[test]
#[should_panic]
fn fail3() {
    let cell = RefCell::new(1);
    let _b = cell.borrow_mut();
    cell.borrow_mut();
}

#[test]
#[should_panic]
fn fail4() {
    let cell = RefCell::new(1);
    let _b = cell.borrow_mut();
    cell.borrow();
}
