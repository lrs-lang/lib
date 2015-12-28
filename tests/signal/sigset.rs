// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::signal::{Sigset};
use std::util::{all_bytes};
use std::signal::signals::{Window, Interrupted};

#[test]
fn test() {
    let mut set = Sigset::new();
    unsafe { test!(all_bytes((set.as_ref():&[d8]).as_bytes(), 0)); }
    set.fill();
    unsafe { test!(all_bytes((set.as_ref():&[d8]).as_bytes(), !0)); }
    set.clear();
    set.set(Window).unwrap();
    test!(set.is_set(Window).unwrap());
    set.set(Interrupted).unwrap();

    let mut set2 = Sigset::new();
    set2.set_all(set);
    test!(set2.all_set(set));
    test!(set2.is_set(Window).unwrap());
    test!(set2.is_set(Interrupted).unwrap());

    test!(!set2.disjoint(set));
    set.unset(Window);
    set2.unset(Interrupted);
    test!(set2.disjoint(set));

    set2.set(Interrupted);
    set2.unset_all(set);
    test!(!set2.is_set(Interrupted).unwrap());
}
