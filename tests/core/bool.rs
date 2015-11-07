// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::bool::{BoolExt};

#[test]
fn map() {
    test!(true.map(|| 1u8) == Some(1));
    test!(false.map(|| 1u8) == None);
}

#[test]
fn eq() {
    test!(true == true);
    test!(false == false);
    test!(false != true);
}
