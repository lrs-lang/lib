// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The lrs standard library.

#![crate_name = "lrs"]
#![crate_type = "lib"]
#![feature(plugin, no_core, macro_reexport, allow_internal_unstable)]
#![plugin(lrs_core_plugin)]
#![no_core]

#[macro_use]
#[macro_reexport(abort, assert, try, print, println, err, errln, vec, format, matches,
                 thread_local, writeln)]
extern crate lrs_core;
extern crate lrs_alloc;
extern crate lrs_arch_fns;
extern crate lrs_atomic;
extern crate lrs_base;
extern crate lrs_buf_reader;
extern crate lrs_cell;
extern crate lrs_cty;
extern crate lrs_cty_base;
extern crate lrs_box;
extern crate lrs_hashmap;
extern crate lrs_wrapping;
extern crate lrs_cfg;
extern crate lrs_fmt;
extern crate lrs_int;
extern crate lrs_io;
extern crate lrs_iter;
extern crate lrs_parse;
extern crate lrs_ringbuf;
extern crate lrs_rc;
extern crate lrs_rmo;
extern crate lrs_hash;
extern crate lrs_rv;
extern crate lrs_saturating;
extern crate lrs_getopt;
extern crate lrs_str_one;
extern crate lrs_str_three;
extern crate lrs_str_two;
extern crate lrs_time_base;
extern crate lrs_time_ext;
extern crate lrs_vec;
extern crate lrs_c_ptr_ptr;
extern crate lrs_tree;
extern crate lrs_lock;

#[cfg(not(freestanding))] extern crate lrs_r_syscall;
#[cfg(not(freestanding))] extern crate lrs_syscall;
#[cfg(not(freestanding))] extern crate lrs_kernel;
#[cfg(not(freestanding))] extern crate lrs_rt;
#[cfg(not(freestanding))] extern crate lrs_clone;
#[cfg(not(freestanding))] extern crate lrs_fd;
#[cfg(not(freestanding))] extern crate lrs_mem;
#[cfg(not(freestanding))] extern crate lrs_event;
#[cfg(not(freestanding))] extern crate lrs_signal;
#[cfg(not(freestanding))] extern crate lrs_pipe;
#[cfg(not(freestanding))] extern crate lrs_queue;
#[cfg(not(freestanding))] extern crate lrs_swap;
#[cfg(not(freestanding))] extern crate lrs_inotify;
#[cfg(not(freestanding))] extern crate lrs_env;
#[cfg(not(freestanding))] extern crate lrs_fs;
#[cfg(not(freestanding))] extern crate lrs_socket;
#[cfg(not(freestanding))] extern crate lrs_netlink;
#[cfg(not(freestanding))] extern crate lrs_sys;
#[cfg(not(freestanding))] extern crate lrs_poll;
#[cfg(not(freestanding))] extern crate lrs_dev;
#[cfg(not(freestanding))] extern crate lrs_file;
#[cfg(not(freestanding))] extern crate lrs_mqueue;
#[cfg(not(freestanding))] extern crate lrs_tty;
#[cfg(not(freestanding))] extern crate lrs_thread;
#[cfg(not(freestanding))] extern crate lrs_process;
#[cfg(not(freestanding))] extern crate lrs_dir;
#[cfg(not(freestanding))] extern crate lrs_user_group;

pub mod atomic;
pub mod alloc;
pub mod tree;
pub mod bool;
pub mod cty;
pub mod error;
pub mod fmt;
pub mod cfg;
pub mod intrinsics;
pub mod iter;
pub mod mem;
pub mod num;
pub mod getopt;
pub mod ops;
pub mod option;
pub mod io;
pub mod parse;
pub mod ringbuf;
pub mod ptr;
pub mod repr;
pub mod result;
pub mod marker;
pub mod share;
pub mod hash;
pub mod hashmap;
pub mod slice;
pub mod string;
pub mod into;
pub mod time;
pub mod util;
pub mod vec;
pub mod rc;
pub mod undef;
pub mod cmp;
pub mod bx;

#[cfg(not(freestanding))]
pub use hosted::{
    event, user, poll, sync, process, fd, file, dir, env, tty, fs, group, netlink,
    mem_map, signal, sys, thread, inotify, clone, socket, syscall, msg_queue, pipe, swap,
};

#[cfg(not(freestanding))]
#[path = ""]
mod hosted {
    pub mod event;
    pub mod user;
    pub mod poll;
    pub mod sync;
    pub mod process;
    pub mod fd;
    pub mod file;
    pub mod dir;
    pub mod env;
    pub mod tty;
    pub mod fs;
    pub mod group;
    pub mod netlink;
    pub mod mem_map;
    pub mod signal;
    pub mod sys;
    pub mod thread;
    pub mod inotify;
    pub mod clone;
    pub mod socket;
    pub mod syscall;
    pub mod msg_queue;
    pub mod pipe;
    pub mod swap;
}

/// The prelude.
pub mod prelude {
    pub mod v1 {
        pub use lrs_core::cmp::{PartialOrd, Ord};
        pub use lrs_base::prelude::*;
        pub use lrs_base::clone::{Clone};
        pub use lrs_parse::{Parse};
        pub use lrs_vec::{Vec};
        pub use lrs_rmo::{ToOwned};
        pub use lrs_io::{Read, Write, BufRead, BufWrite};
        #[cfg(not(freestanding))] pub use lrs_fd::{FDContainer};
    }
}
