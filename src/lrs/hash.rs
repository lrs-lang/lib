// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_hash::{Hash, Hasher};

pub mod xx_hash {
    pub use lrs_hash::xx_hash::{
        u32hash_bytes, u64hash_bytes, u32hash_u8, u32hash_u16, u32hash_u32, u32hash_u64,
        u32hash_usize, u64hash_u8, u64hash_u16, u64hash_u32, u64hash_u64, u64hash_usize,
        XxHash32, XxHash64,
    };
}
