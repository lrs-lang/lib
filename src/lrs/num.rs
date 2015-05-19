// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Number types

pub use lrs_int::{
    Int, UnsignedInt, SignedInt,
};
pub use lrs_saturating::{
    SaturatingCast,
};

/// Constants for the i8 type.
pub mod i8 {
    pub const MIN: i8 = ::lrs_core::num::i8::MIN;
    pub const MAX: i8 = ::lrs_core::num::i8::MAX;
}

/// Constants for the i16 type.
pub mod i16 {
    pub const MIN: i16 = ::lrs_core::num::i16::MIN;
    pub const MAX: i16 = ::lrs_core::num::i16::MAX;
}

/// Constants for the i32 type.
pub mod i32 {
    pub const MIN: i32 = ::lrs_core::num::i32::MIN;
    pub const MAX: i32 = ::lrs_core::num::i32::MAX;
}

/// Constants for the i64 type.
pub mod i64 {
    pub const MIN: i64 = ::lrs_core::num::i64::MIN;
    pub const MAX: i64 = ::lrs_core::num::i64::MAX;
}

/// Constants for the isize type.
pub mod isize {
    pub const BITS:  usize = ::lrs_core::num::isize::BITS;
    pub const BYTES: usize = ::lrs_core::num::isize::BYTES;
    pub const MIN:   isize = ::lrs_core::num::isize::MIN;
    pub const MAX:   isize = ::lrs_core::num::isize::MAX;
}

/// Constants for the u8 type.
pub mod u8 {
    pub const MIN: u8 = ::lrs_core::num::u8::MIN;
    pub const MAX: u8 = ::lrs_core::num::u8::MAX;
}

/// Constants for the u16 type.
pub mod u16 {
    pub const MIN: u16 = ::lrs_core::num::u16::MIN;
    pub const MAX: u16 = ::lrs_core::num::u16::MAX;
}

/// Constants for the u32 type.
pub mod u32 {
    pub const MIN: u32 = ::lrs_core::num::u32::MIN;
    pub const MAX: u32 = ::lrs_core::num::u32::MAX;
}

/// Constants for the u32 type.
pub mod u64 {
    pub const MIN: u64 = ::lrs_core::num::u64::MIN;
    pub const MAX: u64 = ::lrs_core::num::u64::MAX;
}

/// Constants for the usize type.
pub mod usize {
    pub const BITS:  usize = ::lrs_core::num::usize::BITS;
    pub const BYTES: usize = ::lrs_core::num::usize::BYTES;
    pub const MIN:   usize = ::lrs_core::num::usize::MIN;
    pub const MAX:   usize = ::lrs_core::num::usize::MAX;
}
