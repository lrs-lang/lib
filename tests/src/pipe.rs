// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs::signal::{set_handler, signals, SigHandler, Sigset, Signal, SigInfo};
use lrs::signal::flags::{SA_NONE};
use lrs::pipe::{Pipe};
use lrs::pipe::flags::{PIPE_NONE, TEE_NONE};

fn main() {
    set_handler(signals::Window, Sigset::new(), SigHandler::Func(window), SA_NONE);
    let (write1, read1) = Pipe::new(PIPE_NONE).unwrap();
    let (write2, read2) = Pipe::new(PIPE_NONE).unwrap();

    write2.set_capacity(4096).unwrap();
    let buf = [0; 4096];
    write2.write(&buf);
    write1.write(b"Hello World");

    println!("{:?}", read1.copy_to(&write2, 100, TEE_NONE));
}

extern fn window(signal: Signal, info: &SigInfo, context: usize) {
    println!("window");
}
