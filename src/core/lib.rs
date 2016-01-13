// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_core"]
#![crate_type = "lib"]
#![feature(no_core, lang_items, intrinsics, asm, plugin, unboxed_closures,
           optin_builtin_traits, const_fn, fundamental, associated_type_defaults,
           allow_internal_unstable, on_unimplemented)]
#![plugin(lrs_core_plugin)]
#![no_core]

#[macro_use]
pub mod macros;

pub mod array;
pub mod bool;
pub mod char;
pub mod cmp;
pub mod intrinsics;
pub mod iter;
pub mod marker;
pub mod tuple;
pub mod mem;
pub mod int;
pub mod ops;
pub mod option;
pub mod panicking;
pub mod ptr;
pub mod repr;
pub mod slice;
pub mod str;
pub mod thread_local;
pub mod non_zero;
pub mod data;
pub mod float;

mod sort;

pub mod std {
    pub use ::{marker, ops, intrinsics, option, mem};
}

mod core {
    pub use ::{iter, option, intrinsics};
}

pub mod prelude {
    pub mod v1 {
        pub use marker::{
            Sized, Copy, Pod, Send, Sync, NoSend, NoSync, Interrupt, NoInterrupt,
            PhantomData,
        };
        pub use option::{Option};
        pub use option::Option::{Some, None};
        pub use ops::{Fn, FnOnce, FnMut, Drop, Deref, DerefMut};
        pub use mem::{drop};
        pub use iter::{Iterator};
        pub use data::{d8, DataSlice};
    }
}
