// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cmp};

#[test]
fn min() {
    assert!(cmp::min(0, 1) == 0);
    assert!(cmp::min(0, 0) == 0);
}

#[test]
fn min_ref() {
    assert!(cmp::min_ref(&0, &1) == &0);
    assert!(cmp::min_ref(&0, &0) == &0);
}

#[test]
fn min_mut() {
    assert!(cmp::min_mut(&mut 0, &mut 1) == &mut 0);
    assert!(cmp::min_mut(&mut 0, &mut 0) == &mut 0);
}

#[test]
fn max() {
    assert!(cmp::max(0, 1) == 1);
    assert!(cmp::max(0, 0) == 0);
}

#[test]
fn max_ref() {
    assert!(cmp::max_ref(&0, &1) == &1);
    assert!(cmp::max_ref(&0, &0) == &0);
}

#[test]
fn max_mut() {
    assert!(cmp::max_mut(&mut 0, &mut 1) == &mut 1);
    assert!(cmp::max_mut(&mut 0, &mut 0) == &mut 0);
}
