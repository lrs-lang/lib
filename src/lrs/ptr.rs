// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Pointer manipulation.

pub use lrs_core::ptr::{
    read, write, drop, memcpy, memmove, NonZeroPtr, volatile_load, volatile_store,
    NoAliasMemPtr, AliasMemPtr, NoAliasObjPtr, AliasObjPtr, NoAliasMutObjPtr,
    AliasMutObjPtr,
};

pub use lrs_arch_fns::{
    memcpy_aligned_16_64, memcpy_aligned_16_16,
};
