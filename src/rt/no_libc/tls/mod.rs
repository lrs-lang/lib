// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {aux, cty};
use base::prelude::*;

macro_rules! align {
    // Rounds $val up so that align_up!($val, [%] $to) has $to alignment. The rv is in the
    // range [$val, $val+$to).
    ($val:expr, [%] $to:expr) => {{
        let val = $val;
        let mask = $to - 1;
        (val + mask) & !mask
    }};
    // Rounds $val up so that align_up!($val, [+] $with, [%] $to) + $with has $to
    // alignment. The rv is in the range [$val, $val+$to).
    ($val:expr, [+] $with:expr, [%] $to:expr) => {{
        let val = $val;
        let with = $with;
        let to = $to;
        let mask = to - 1;
        align!(val + (with & mask), [%] to) - (with & mask)
    }}
}

#[cfg(target_arch = "x86_64")] #[path = "x86_64.rs"] pub mod arch;
#[cfg(target_arch = "x86")] #[path = "x86.rs"] pub mod arch;
#[cfg(target_arch = "arm")] #[path = "arm.rs"] pub mod arch;
#[cfg(target_arch = "aarch64")] #[path = "aarch64.rs"] pub mod arch;

#[derive(Copy)]
pub struct TlsImage {
    addr: *const u8,
    /// The size of the memory pointed to by addr.
    file_size: usize,
    /// The size of the tls block each thread uses. If mem_size > file_size, the remainder
    /// contains zeroed memory. This is a multiple of alignment.
    mem_size: usize,
    alignment: usize,
}

#[repr(C)]
pub struct Private {
    arch: arch::ArchPrivate,
}

pub static mut STATIC_IMAGE: Option<TlsImage> = None;

pub unsafe fn init_tls() {
    let phdrs = aux::program_header_table().unwrap();

    if let Some(tls) = phdrs.iter().find(|h| h.p_type == cty::PT_TLS) {
        let image = TlsImage {
            addr: tls.p_vaddr as *const _,
            file_size: tls.p_filesz as usize,
            mem_size: align!(tls.p_memsz as usize, [%] tls.p_align as usize),
            alignment: tls.p_align as usize,
        };

        STATIC_IMAGE = Some(image);
    }

    set_static_image();
}

pub unsafe fn set_static_image() {
    use syscall::{mmap};
    use cty::{PROT_READ, PROT_WRITE, MAP_ANONYMOUS, MAP_PRIVATE};

    let size = align!(arch::mem_size(), [%] aux::page_size().unwrap());
    let mem = mmap(0, size, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, -1,
                   0) as *mut u8;
    let (_, tp) = arch::place_tls(mem);
    arch::set_tp(tp).unwrap();
}
