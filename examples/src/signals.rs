// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::{mem, time, error, process};
use std::signal::{
    Sigset, Signal, signals, blocked_signals, block_signal, pending_signals,
    block_signals, Sigfd, SigfdInfo, wait, wait_timeout, set_handler, SigHandler,
    SigInfo,
};
use std::signal::flags::{SIGFD_NONE, SA_NONE, SA_RESTART};

fn main() {
    // let mut sigset = Sigset::new();
    // sigset.set(signals::Interrupted);
    // sigset.set(signals::Window);
    // block_signals(sigset);

    // let fd = Sigfd::new(sigset, SIGFD_NONE).unwrap();
    // let mut buf = [SigfdInfo::new()];
    // loop {
    //     for el in fd.read(&mut buf).unwrap() {
    //         println!("{:?}", el.signal());
    //     }
    // }

    // let info = wait_timeout(sigset, time::Time::seconds(2)).unwrap();;
    // println!("{:?}", info.signal());

    // set_handler(signals::InvalidAddress, Sigset::new(), SigHandler::Func(f), SA_NONE);
    set_handler(signals::Window, Sigset::new(), SigHandler::Func(f), SA_NONE);

    let mut sigset = Sigset::new();
    sigset.set(signals::Interrupted);
    block_signals(sigset);
    while let Err(error::Interrupted) = wait(sigset) { }
}

extern fn f(sig: Signal, info: &SigInfo, context: usize) {
    println!("yo");
}
