// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{c_int, ElfPhdr, AUX_CNT, AT_PHDR, AT_EXECFD, AT_PHNUM, AT_PAGESZ};
use core::{slice};

static mut AUXV: [usize; AUX_CNT] = [0; AUX_CNT];

/// Initializes the AUXV.
///
/// [argument, auxv]
/// A pointer to the environment variable passed by the OS.
pub fn init(mut auxv: *const usize) {
    unsafe {
        while *auxv != 0 {
            auxv = auxv.add(1);
        }
        auxv = auxv.add(1);

        while *auxv != 0 {
            if *auxv < AUX_CNT {
                AUXV[*auxv] = *auxv.add(1);
            }
            auxv = auxv.add(2);
        }
    }
}

/// The program to be interpreted.
///
/// [return_value]
/// Returns a file descriptor to the object file to be interpreted.
///
/// = Remarks
///
/// Only set if this program is an interpreter. Even then, `AT_PHDR` might be set instead.
pub fn program_fd() -> Option<c_int> {
    match unsafe { AUXV[AT_EXECFD] } {
        0 => None,
        n => Some(n as c_int),
    }
}

/// The program header table.
///
/// [return_value]
/// Said table.
pub fn program_header_table() -> Option<&'static [ElfPhdr]> {
    match unsafe { AUXV[AT_PHDR] } {
        0 => None,
        n => Some(unsafe { slice::from_ptr(n as *const _, AUXV[AT_PHNUM]) }),
    }
}

/// The page size of the process.
///
/// [return_value]
/// Said size.
pub fn page_size() -> Option<usize> {
    match unsafe { AUXV[AT_PAGESZ] } {
        0 => None,
        n => Some(n),
    }
}
