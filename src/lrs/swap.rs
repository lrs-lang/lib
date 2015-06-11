// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_swap::{swap_on, swap_off};
pub use lrs_swap::flags::{SwapFlags};

pub mod flags {
    pub use lrs_swap::flags::{
        SWAP_NONE, SWAP_PREFER, SWAP_DISCARD,
    };
}
