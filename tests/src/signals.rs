// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs::{mem, time};
use lrs::signal::{Sigset, Signal, signals, blocked_signals, block_signal, pending_signals,
           block_signals, wait};

fn main() {
    let mut sigset = Sigset::new();
    sigset.set(signals::Interrupted);
    sigset.set(signals::Window);
    wait(sigset);
}
