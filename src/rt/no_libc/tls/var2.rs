// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{cmp, mem, ptr};
use super::super::{STATIC_IMAGE, Private};
use base::prelude::*;

/// Calculates the size required for the tls block.
pub fn mem_size() -> usize {
    let (tls_size, tls_align) = match unsafe { STATIC_IMAGE } {
        Some(i) => (i.mem_size, i.alignment),
        _ => (0, 1),
    };
    let max_align = cmp::max(tls_align, mem::align_of::<Private>());
    max_align - 1 + tls_size + mem::size_of::<Private>()
}

/// Installs the tls into memory.
///
/// [argument, mem]
/// The memory in which the tls will be placed.
///
/// [return_value]
/// Returns a pointer to the Private and the thread pointer.
///
/// = Remarks
///
/// `mem` does not have any alignment requirements. However, the memory pointed to must
/// have size at least `mem_size()` or the behavior is undefined.
///
/// The memory passed to this function must be zeroed, or the behavior is undefined.
pub unsafe fn place_tls(mut mem: *mut u8) -> (*mut Private, *mut u8) {
    if let Some(ref image) = STATIC_IMAGE {
        let max_align = cmp::max(image.alignment, mem::align_of::<Private>());

        let old_mem = mem as usize;
        mem = align!(mem as usize, [+] image.mem_size, [%] max_align) as *mut u8;
        assert!(mem as usize >= old_mem);
        ptr::memcpy(mem, image.addr, image.file_size);
        mem = mem.add(image.mem_size);
    } else {
        mem = align!(mem as usize, [%] mem::align_of::<Private>()) as *mut u8;
    }
    (mem as *mut _, mem)
}

pub unsafe fn get_private(mem: *mut u8) -> *mut Private {
    mem as *mut Private
}
