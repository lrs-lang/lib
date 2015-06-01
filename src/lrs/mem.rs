// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Memory operations.

pub use lrs_core::mem::{
    uninit, cast, zeroed, copy_as, forget, drop, copy, unsafe_copy, swap, replace,
    size_of, align_of, needs_drop, as_bytes, as_mut_bytes, align_for, is_suitable_for,
    from_bytes, from_mut_bytes, align_for_mut, addr,
};

pub use lrs_mem::{advise, protect, lock, unlock, lock_all, unlock_all};
pub use lrs_mem::flags::{MemLockFlags};

pub mod advice {
    pub use lrs_mem::adv::{
        Normal, Random, Sequential, WillNeed, DontNeed, Remove, DontFork, DoFork,
        HwPoison, SoftOffline, Mergeable, Unmergeable, HugePage, NoHugePage, DontDump,
        DoDump,
    };
}

pub mod flags {
    pub use lrs_mem::flags::{
        MLOCK_CURRENT, MLOCK_FUTURE,
    };
}
