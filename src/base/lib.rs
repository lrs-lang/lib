// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_base"]
#![crate_type = "lib"]
#![feature(no_std, custom_derive, default_type_parameter_fallback)]
#![no_std]

extern crate lrs_wrapping as wrapping;
extern crate lrs_cty_base as cty_base;

pub mod std {
    pub use core::*;
    pub use {result, error};
    pub mod share {
        pub use core::thread_local::*;
    }
    pub mod conv {
        pub use default::{Default};
        pub use into::{Into};
        pub use to::{To, TryTo};
        pub use as_ref::{AsRef, AsMut};
    }
}

pub mod result;
pub mod error;
pub mod as_ref;
pub mod into;
pub mod to;
pub mod undef;
pub mod default;

pub mod prelude {
    pub use core::prelude::v1::*;
    pub use core::bool::{BoolExt};
    pub use result::{Result};
    pub use result::Result::{Ok, Err};
    pub use as_ref::{AsRef, AsMut};
    pub use into::{Into};
    pub use to::{To, TryTo};
    pub use default::{Default};
}
