// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Memory allocation

pub use linux_alloc::{
    MAX_SIZE, empty_ptr, allocate_raw, allocate_array, allocate, free_raw, free_array,
    free, reallocate_raw, reallocate_array,
};
