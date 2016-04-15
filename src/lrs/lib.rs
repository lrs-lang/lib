// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The lrs library documentation.
//!
//! = Description
//!
//! == Definitions
//!
//! This section defines various terms that are used in the rest of the documentation.
//!
//! === the interpreter
//!
//! The machine/program interpreting the program.
//!
//! === undefined operation
//!
//! An operation for which the behavior of the interpreter has not been restricted by the
//! documentation.
//!
//! === undefined behavior
//!
//! Behavior upon an undefined operation.
//!
//! === value
//!
//! A non-empty set of finite sequences of bytes of the same length.
//!
//! === values of a type
//!
//! A set of mutually disjoint values.
//!
//! [info]
//! Hence, the values of a type define an equivalence relation on a subset of all values.
//!
//! === equivalence of two values
//!
//! Two values are equivalent if their intersection is non-empty.
//!
//! === size of a value
//!
//! The length of any of its members.
//!
//! === sized type
//!
//! A type such that all values of said type have the same size.
//!
//! === unsized type
//!
//! A type that is not sized.
//!
//! === undefined byte of a value
//!
//! A byte that varies between the byte sequences contained in a value.
//!
//! === undefined value with respect to a type
//!
//! A value that is equivalent to more than one value of said type.
//!
//! === memory representation of a value
//!
//! A sequence of values of size 1 where the `i`-th value
//!
//! * is undefined with respect to the `u8` type if the `i`-th byte of the original value
//!   is undefined;
//! * is an `u8` value containing the `i`-th byte of any element of the original value if
//!   the `i`-th byte of the original value is not undefined.
//!
//! === unspecified value
//!
//! A value of a type that is known only at runtime.
//!
//! === implementation defined
//!
//! A property of a trait specified by each implementation.
//!
//! === plain old data type
//!
//! A sized type such that the union of all of its values contains all byte sequences of
//! the types size.
//!
//! === operation constraints
//!
//! A set of properties that hold whenever an operation is invoked.
//!
//! === safe operation
//!
//! An operation without constraints.
//!
//! === implementation constraints
//!
//! A set of properties that every implementation of a trait satisfies.
//!
//! === safe trait
//!
//! A trait without implementation constraints.
//!
//! === leakable value
//!
//! A value
//!
//! * that doesn't have a destructor; or
//! * whose destructor need not run at the end of the value's lifetime.
//!
//! == General properties
//!
//! This section lists properties that apply to all interfaces.
//!
//! A value's destructor runs at the end of the values lifetime unless the value is
//! leakable.
//!
//! = Remarks
//!
//! The definition of value allows the value of padding bytes to be unspecified. That is,
//! two sequences of bytes where only the padding bytes differ correspond to the same
//! value.
//!
//! Undefined values correspond to values that have LLVM's `undef` value outside of
//! padding bytes.

#![crate_name = "lrs"]
#![crate_type = "lib"]
#![feature(no_core, macro_reexport, allow_internal_unstable)]
#![no_core]

#[macro_use]
#[macro_reexport(abort, assert, try, print, println, err, errln, vec, format, matches,
                 thread_local, writeln, impl_try_to, impl_try_from)]
extern crate lrs_core;
extern crate lrs_alloc;
extern crate lrs_arch_fns;
extern crate lrs_atomic;
extern crate lrs_base;
extern crate lrs_buf_reader;
extern crate lrs_cell;
extern crate lrs_cty;
extern crate lrs_slice;
extern crate lrs_varargs;
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
extern crate lrs_str_two;
extern crate lrs_time_base;
extern crate lrs_time_ext;
extern crate lrs_vec;
extern crate lrs_c_ptr_ptr;
extern crate lrs_tree;
extern crate lrs_lock;
extern crate lrs_rand;

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
// #[cfg(not(freestanding))] extern crate lrs_user_group;

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
pub mod varargs;
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
pub mod conv;
pub mod time;
pub mod util;
pub mod vec;
pub mod rc;
pub mod undef;
pub mod cmp;
pub mod bx;
pub mod rand;

#[cfg(not(freestanding))]
pub use hosted::{
    event, /* user, group, */ poll, sync, process, fd, file, dir, env, tty, fs, netlink,
    mem_map, signal, sys, thread, inotify, socket, syscall, msg_queue, pipe, swap,
};

#[cfg(not(freestanding))]
#[path = ""]
mod hosted {
    pub mod event;
    // pub mod user;
    pub mod poll;
    pub mod sync;
    pub mod process;
    pub mod fd;
    pub mod file;
    pub mod dir;
    pub mod env;
    pub mod tty;
    pub mod fs;
    // pub mod group;
    pub mod netlink;
    pub mod mem_map;
    pub mod signal;
    pub mod sys;
    pub mod thread;
    pub mod inotify;
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
        pub use lrs_parse::{Parse};
        pub use lrs_vec::{Vec};
        pub use lrs_io::{Read, Write, BufRead, BufWrite};
        pub use lrs_slice::{SliceExt};
        #[cfg(not(freestanding))] pub use lrs_fd::{FdContainer};
    }
}
