// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This is generic code for platforms that use the Tls organization described below. It's
//! called var1 because it's the first variant described in Drepper's Tls paper.
//!
//! The memory is organized as follows:
//!
//! ----------------------------------------------------------------------
//! | private area | dtv area | padding |          tls area              |
//! ----------------------------------------------------------------------
//!       ________/ \_______
//!      /the thread pointer\
//!
//! The alignment of the thread pointer must be the maximum of the alignments of the
//! private area, the dtv area, and the tls area. The padding is calculated by rounding
//! the size of the dtv area up to a multiple of the tls area's alignment. For example, if
//! the dtv area has size 8 and the tls area has alignment 16, then the padding is 8
//! bytes.
//!
//! The dtv area is unused in statically linked programs (which is all that tls supports
//! at the moment) and is unused in tls.

use core::{mem, ptr};
use super::super::{STATIC_IMAGE, Private};
use super::{DTVR_ALIGN, DTVR_SIZE};
use base::prelude::*;

/// Calculates the maximum alignment of the private area, the dtv area, and the tls area.
///
/// [argument, tls_align]
/// The alignment of the tls area.
///
/// [return_value]
/// The calculated alignment.
fn max_align(tls_align: usize) -> usize {
    // This is the maximum of the three values, using that they are all powers of two.
    //
    // Another way to write this:
    //
    // !(!(tls_align - 1) & !(mem::align_of::<Private>() - 1) & !(DTVR_ALIGN - 1)) + 1
    //
    // The way below has 3 instructions instead of the 5 above (remember that the last two
    // numbers are known at compile time). But I'm not sure how expensive `leading_zeros`
    // is.

    (isize::min() as usize) // this is the number with only the highest bit set
        >> (tls_align | mem::align_of::<Private>() | DTVR_ALIGN).leading_zeros()
}

/// Calculates the size required for the tls block.
///
/// [return_value]
/// The calculated size.
///
/// = Remarks
///
/// The size returned is a little bit more than what is strictly required, but it allows
/// the user to pass arbitrary pointers to the function below.
pub fn mem_size() -> usize {
    let (tls_size, tls_align) = match unsafe { STATIC_IMAGE } {
        Some(i) => (i.mem_size, i.alignment),
        _ => (0, 1),
    };
    max_align(tls_align) - 1 + mem::size_of::<Private>() +
        align!(DTVR_SIZE, [%] tls_align) + tls_size
}

/// Installs the tls into memory.
///
/// [argument, mem]
/// The memory in which the tls will be placed.
///
/// [return_value]
/// Returns a pointer to the private area and the thread pointer.
///
/// = Remarks
///
/// `mem` does not have any alignment requirements. However, the memory pointed to must
/// have size at least `mem_size()` or the behavior is undefined.
///
/// The memory passed to this function must be zeroed or the behavior is undefined.
pub unsafe fn place_tls(mut mem: *mut u8) -> (*mut Private, *mut u8) {
    let (private, tp);

    if let Some(ref image) = STATIC_IMAGE {
        let max_align = max_align(image.alignment);

        mem = align!(mem as usize, [+] mem::size_of::<Private>(),
                     [%] max_align) as *mut u8;
        private = mem as *mut Private;
        mem = mem.add(mem::size_of::<Private>());
        tp = mem;
        mem = mem.add(align!(DTVR_SIZE, [%] image.alignment));
        ptr::memcpy(mem, image.addr, image.file_size);
    } else {
        let max_align = max_align(1);

        mem = align!(mem as usize, [+] mem::size_of::<Private>(),
                     [%] max_align) as *mut u8;
        private = mem as *mut Private;
        tp = mem.add(mem::size_of::<Private>());
    }

    (private, tp)
}

/// Calculates the position of the private area.
///
/// [argument, tp]
/// The thread pointer.
///
/// [return_value]
/// A pointer to the private area.
pub unsafe fn get_private(tp: *mut u8) -> *mut Private {
    tp.sub(mem::size_of::<Private>()) as *mut Private
}
