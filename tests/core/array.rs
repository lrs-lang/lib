// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn eq() {
    assert!([0] == [0]);
    assert!([0, 1] == [0, 1]);
    assert!([0, 1, 2] == [0, 1, 2]);
    assert!([0, 1, 2, 3] != [0, 1, 2, 4]);
}
