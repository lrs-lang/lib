// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::signal::{self, Sigset};
use std::signal::signals::{Window, Interrupted};

mod sigset;

#[test]
fn block_signal() {
    let mut set = Sigset::new();
    set.set(Window);

    let mut set2 = set;
    set2.set(Interrupted);

    test!(signal::block_signal(Window).unwrap() == Sigset::new());
    test!(signal::blocked_signals().unwrap() == set);
    test!(signal::unblock_signal(Window).unwrap() == set);
    test!(signal::block_signals(set).unwrap() == Sigset::new());
    test!(signal::blocked_signals().unwrap() == set);
    test!(signal::unblock_signals(set).unwrap() == set);
    test!(signal::blocked_signals().unwrap() == Sigset::new());

    test!(signal::block_signal(Window).unwrap() == Sigset::new());
    test!(signal::set_blocked_signals(set2).unwrap() == set);
    test!(signal::blocked_signals().unwrap() == set2);

    test!(signal::unblock_signal(Interrupted).unwrap() == set2);
    test!(signal::blocked_signals().unwrap() == set);
}
