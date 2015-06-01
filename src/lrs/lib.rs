// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The lrs standard library.

#![crate_name = "lrs"]
#![crate_type = "lib"]
#![feature(plugin, no_std, macro_reexport)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
#[macro_reexport(abort, assert, try, println, errln, vec, format)]
extern crate lrs_core;
extern crate lrs_alloc;
extern crate lrs_arch_fns;
extern crate lrs_atomic;
extern crate lrs_base;
extern crate lrs_buf_reader;
extern crate lrs_cell;
extern crate lrs_kernel;
extern crate lrs_cty;
extern crate lrs_cty_base;
extern crate lrs_env;
extern crate lrs_box;
extern crate lrs_dev;
extern crate lrs_dir;
extern crate lrs_fd;
extern crate lrs_event;
extern crate lrs_inotify;
extern crate lrs_netlink;
extern crate lrs_file;
extern crate lrs_fmt;
extern crate lrs_fs;
extern crate lrs_thread;
extern crate lrs_int;
extern crate lrs_io;
extern crate lrs_iter;
extern crate lrs_lock;
extern crate lrs_parse;
extern crate lrs_mmap;
extern crate lrs_poll;
extern crate lrs_process;
extern crate lrs_pipe;
extern crate lrs_signal;
extern crate lrs_queue;
extern crate lrs_rc;
extern crate lrs_rmo;
extern crate lrs_r_syscall;
extern crate lrs_rt;
extern crate lrs_rv;
extern crate lrs_saturating;
extern crate lrs_socket;
extern crate lrs_getopt;
extern crate lrs_str_one;
extern crate lrs_str_three;
extern crate lrs_str_two;
extern crate lrs_sys;
extern crate lrs_syscall;
extern crate lrs_time_base;
extern crate lrs_time_ext;
extern crate lrs_user_group;
extern crate lrs_vec;
extern crate lrs_clone;
extern crate lrs_c_ptr_ptr;

pub mod atomic;
pub mod alloc;
pub mod dir;
pub mod env;
pub mod error;
pub mod fd;
pub mod file;
pub mod fmt;
pub mod fs;
pub mod group;
pub mod intrinsics;
pub mod iter;
pub mod event;
pub mod mem;
pub mod num;
pub mod getopt;
pub mod netlink;
pub mod ops;
pub mod option;
pub mod io;
pub mod parse;
pub mod poll;
pub mod process;
pub mod mem_map;
pub mod ptr;
pub mod repr;
pub mod result;
pub mod marker;
pub mod share;
pub mod signal;
pub mod slice;
pub mod string;
pub mod into;
pub mod sys;
pub mod syscall;
pub mod time;
pub mod user;
pub mod util;
pub mod vec;
pub mod thread;
pub mod inotify;
pub mod rc;
pub mod clone;
pub mod sync;
pub mod pipe;
pub mod cmp;
pub mod socket;
pub mod bx;

/// The prelude.
pub mod prelude {

    pub use lrs_core::cmp::{PartialOrd, Ord};
    pub use lrs_base::prelude::*;
    pub use lrs_base::clone::{Clone};
    pub use lrs_parse::{Parse};
    pub use lrs_vec::{Vec};
    pub use lrs_rmo::{ToOwned};
    pub use lrs_io::{Read, Write, BufRead, BufWrite};
    pub use lrs_fd::{FDContainer};
}

pub mod core {
    pub mod option {
        pub use lrs_core::option::{Option};
    }
    pub mod iter {
        pub use lrs_core::iter::{Iterator, IntoIterator};
    }
    pub mod intrinsics {
        pub use lrs_core::intrinsics::{discriminant_value};
    }
}
