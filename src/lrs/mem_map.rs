// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_mem::{MemMap};
pub use lrs_mem::flags::{MemMapFlags, MemProtFlags, MemReMapFlags, MemSyncFlags};

pub mod flags {
    pub use lrs_mem::flags::{
        MMAP_NONE, MMAP_32BIT, MMAP_ANON, MMAP_FIXED, MMAP_HUGE_PAGES, MMAP_HUGE_2MB,
        MMAP_HUGE_1GB, MMAP_LOCKED, MMAP_DONT_BLOCK, MMAP_DONT_RESERVE, MMAP_POPULATE,
        MMAP_UNINITIALIZED, PROT_NONE, PROT_EXEC, PROT_READ, PROT_WRITE, MREMAP_NONE,
        MREMAP_MAY_MOVE, MREMAP_FIXED, MSYNC_ASYNC, MSYNC_SYNC, MSYNC_INVALIDATE,
    };
}
