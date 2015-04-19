// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_ty_two"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_ty_one as ty_one;
extern crate linux_arch as arch;
extern crate linux_lock as lock;
extern crate linux_io as io;
extern crate linux_fmt as fmt;
extern crate linux_alloc as alloc;

pub mod linux {
    pub use ::fmt::linux::*;
    pub mod vec {
        pub use ::vec::{Vec};
    }
}

pub mod arc;
pub mod byte_string;
pub mod c_string;
pub mod io_ext;
pub mod iter_ext;
pub mod path_buf;
pub mod queue;
pub mod rc;
pub mod rmo;
pub mod string;
pub mod vec;

pub mod prelude {
    pub use string::{String};
    pub use vec::{Vec};
    pub use rmo::{ToOwned};
}
