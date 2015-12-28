// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_type = "lib"]
#![crate_name = "tests"]
#![feature(custom_derive, type_ascription)]

macro_rules! test {
    ($e:expr) => {
        // if !$e { ::std::process::exit(1); }
        assert!($e);
    }
}

mod core;
mod libc;
mod int;
mod saturating;
mod wrapping;
mod cty_base;
mod arch_fns;
mod base;
mod hash;
mod rv;
mod parse;
mod io;
mod fmt;
mod cell;
mod str_one;
mod getopt;
mod rt;
mod atomic;
mod cty;
mod r_syscall;
mod syscall;
mod kernel;
mod clone;
mod fd;
mod mem;
mod lock;
mod time_base;
mod event;
mod signal;
mod pipe;
mod alloc;
mod queue;
// mod box;
mod c_ptr_ptr;
mod buf_reader;
mod rc;
mod vec;
mod ringbuf;
mod hashmap;
mod iter;
mod str_two;
mod rmo;
mod str_three;
mod swap;
mod inotify;
mod env;
mod fs;
mod socket;
mod netlink;
mod sys;
mod poll;
mod dev;
mod file;
mod mqueue;
mod tty;
mod thread;
mod process;
mod time_ext;
mod dir;
mod user_group;
