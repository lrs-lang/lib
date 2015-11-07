// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::clone::{MaybeClone};

#[test]
fn tuple() {
    test!((0,).clone() == (0,));
    test!((0, 1).clone() == (0, 1));
    test!((0, 1, 2).clone() == (0, 1, 2));

    test!((0,).maybe_clone() == Ok((0,)));
    test!((0, 1).maybe_clone() == Ok((0, 1)));
}
