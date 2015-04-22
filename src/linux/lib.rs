// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Experimental Linux standard library.
//!
//! This library tries to create a rust standard library on top of the Linux API. It is
//! not bound by any other standards. In particular, it does not try to create a
//! POSIX-like api or an API that can easily be ported to other platforms.
//!
//! Currently only `x86_64` is supported.

#![crate_name = "linux"]
#![crate_type = "lib"]
#![feature(plugin, no_std, macro_reexport)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
#[macro_reexport(abort, assert, try, println, vec, format)]
extern crate linux_core;
extern crate linux_alloc;
extern crate linux_arch_fns;
extern crate linux_atomic;
extern crate linux_base;
extern crate linux_buf_reader;
extern crate linux_cell;
extern crate linux_cty;
extern crate linux_cty_base;
extern crate linux_dev;
extern crate linux_dir;
extern crate linux_fd;
extern crate linux_file;
extern crate linux_fmt;
extern crate linux_fs;
extern crate linux_int;
extern crate linux_io;
extern crate linux_iter;
extern crate linux_lock;
extern crate linux_parse;
extern crate linux_poll;
extern crate linux_process;
extern crate linux_queue;
extern crate linux_rc;
extern crate linux_rmo;
extern crate linux_r_syscall;
extern crate linux_rt;
extern crate linux_rv;
extern crate linux_saturating;
extern crate linux_socket;
extern crate linux_sort;
extern crate linux_str_one;
extern crate linux_str_three;
extern crate linux_str_two;
extern crate linux_sys;
extern crate linux_syscall;
extern crate linux_time_base;
extern crate linux_time_ext;
extern crate linux_user_group;
extern crate linux_vec;
extern crate linux_clone;
extern crate linux_c_ptr_ptr;

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
pub mod mem;
pub mod num;
pub mod ops;
pub mod option;
pub mod io;
pub mod parse;
pub mod poll;
pub mod process;
pub mod ptr;
pub mod repr;
pub mod result;
pub mod share;
pub mod slice;
pub mod string;
pub mod sys;
pub mod time;
pub mod user;
pub mod util;
pub mod vec;

// Annoying that these have to be top-modules.
pub mod i8;
pub mod i16;
pub mod i32;
pub mod i64;
pub mod u8;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod isize;
pub mod usize;

pub mod prelude {
    //! The prelude.

    pub use linux_base::prelude::*;
    pub use linux_parse::{Parse};
    pub use linux_vec::{Vec};
}

pub mod core {
    //! This sad existence is needed because for loops are hard-coded to use this path.

    pub mod option {
        pub use linux_core::option::{Option};
    }
    pub mod iter {
        pub use linux_core::iter::{Iterator, IntoIterator};
    }
    pub mod intrinsics {
        pub use linux_core::intrinsics::{discriminant_value};
    }
}
