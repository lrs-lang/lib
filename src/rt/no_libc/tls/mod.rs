// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {aux, cty, AtExit};
use core::{ptr};
use base::prelude::*;
use atomic::{AtomicCInt, AtomicU8};
use lock::{SingleThreadMutex};

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
    /// Architecture-specific elements. NOTE: This MUST be the first element.
    pub arch: arch::ArchPrivate,

    pub thread_id: AtomicCInt,
    pub mem_base: *mut u8,
    pub mem_size: usize,

    /// Values defined in the thread crate.
    pub status: AtomicU8,

    pub at_exit: SingleThreadMutex<AtExit>,
}

impl !Send for Private { }
impl !Sync for Private { }

pub static mut STATIC_IMAGE: Option<TlsImage> = None;

pub unsafe fn init() {
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
    let (private, tp) = arch::place_tls(mem);
    ptr::write(&mut (*private).at_exit, SingleThreadMutex::new(AtExit::new()));

    arch::set_tp(tp).unwrap();
}

pub fn size() -> usize {
    arch::mem_size()
}

pub unsafe fn place(mem: *mut u8) -> (*mut Private, *mut u8) {
    arch::place_tls(mem)
}

pub unsafe fn private() -> &'static Private {
    &*arch::get_private()
}

pub fn at_exit() -> &'static SingleThreadMutex<AtExit> {
    unsafe { &(*arch::get_private()).at_exit }
}
