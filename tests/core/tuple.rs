// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn eq() {
    assert!((1, 2) == (1, 2));
    assert!((1, 2) != (1, 3));
    assert!((1, 2, 3) == (1, 2, 3));
    assert!((1, 2, 3) != (1, 3, 3));
}

#[test]
fn ord() {
    assert!((1, 2) < (1, 3));
    assert!((1, 2, 3) < (1, 3, 3));
}
