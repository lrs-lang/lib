// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[test]
fn debug() {
    let mut buf = [0; 20];
    let mut buf = Vec::buffered(&mut buf);
    write!(&mut buf, "{:?}", (1, 1));
    test!(&*buf == "(1, 1)");
}
