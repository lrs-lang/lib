// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs::string::{CStr, AsByteStr};
use lrs::inotify::{Inotify, InodeWatch, InodeData, InodeDataIter};
use lrs::inotify::{WatchFlags, InotifyFlags};
use lrs::inotify::{InodeEvents};
use lrs::inotify::events::{INEV_ALL};
use lrs::inotify::flags::{WATCH_NONE, INOTIFY_NONE};

fn main() {
    let inotify = Inotify::new(INOTIFY_NONE).unwrap();
    inotify.set_watch(".", INEV_ALL, WATCH_NONE).unwrap();

    let mut buf = [0; 1024];

    loop {
        for event in inotify.events(&mut buf).unwrap() {
            println!("{:?}: {:?}", event.events, event.name());
        }
    }
}

