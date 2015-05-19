// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_rc"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_fmt as fmt;
extern crate lrs_cell as cell;
extern crate lrs_alloc as alloc;
extern crate lrs_atomic as atomic;

pub use rc::{Rc, RcBuf};
pub use arc::{Arc, ArcBuf};

pub mod lrs { pub use ::fmt::lrs::*; }

pub mod rc;
pub mod arc;

// TODO: Arc is actually just an Rc with some atomic operations. Maybe Arc should just
// wrap an Rc?
