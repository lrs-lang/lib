// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Memory operations.

pub use lrs_core::mem::{
    uninit, cast, zeroed, copy_as, forget, drop, copy, unsafe_copy, swap, replace,
    size_of, align_of, needs_drop,
};
