// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_core"]
#![crate_type = "lib"]
#![feature(no_std, lang_items, intrinsics, asm, plugin, unboxed_closures,
           optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
pub mod macros;

pub mod array;
pub mod clone;
pub mod cmp;
pub mod intrinsics;
pub mod iter;
pub mod marker;
pub mod mem;
pub mod num;
pub mod ops;
pub mod option;
pub mod panicking;
pub mod prelude;
pub mod ptr;
pub mod repr;
pub mod result;
pub mod slice;
pub mod str;
pub mod char;

mod core {
    pub use ::{marker, ops, clone, intrinsics};
    pub use ::{iter, option};
}

#[link(name = "c")]
extern { }

#[lang = "start"]
fn lang_start(main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    unsafe { mem::cast::<_, fn()>(main)(); }
    0
}
