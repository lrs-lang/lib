// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_rc"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_fmt as fmt;
extern crate linux_cell as cell;
extern crate linux_alloc as alloc;
extern crate linux_atomic as atomic;

pub use rc::{Rc};
pub use arc::{Arc};

pub mod linux { pub use ::fmt::linux::*; }

pub mod rc;
pub mod arc;
