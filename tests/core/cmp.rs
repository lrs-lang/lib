// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cmp};

#[test]
fn min() {
    test!(cmp::min(0, 1) == 0);
    test!(cmp::min(0, 0) == 0);
}

#[test]
fn min_ref() {
    test!(cmp::min_ref(&0, &1) == &0);
    test!(cmp::min_ref(&0, &0) == &0);
}

#[test]
fn min_mut() {
    test!(cmp::min_mut(&mut 0, &mut 1) == &mut 0);
    test!(cmp::min_mut(&mut 0, &mut 0) == &mut 0);
}

#[test]
fn max() {
    test!(cmp::max(0, 1) == 1);
    test!(cmp::max(0, 0) == 0);
}

#[test]
fn max_ref() {
    test!(cmp::max_ref(&0, &1) == &1);
    test!(cmp::max_ref(&0, &0) == &0);
}

#[test]
fn max_mut() {
    test!(cmp::max_mut(&mut 0, &mut 1) == &mut 1);
    test!(cmp::max_mut(&mut 0, &mut 0) == &mut 0);
}
