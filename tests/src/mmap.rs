// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::file::{File};
use std::mem_map::{MemMap};
use std::mem_map::{MemMapFlags, MemProtFlags, MemReMapFlags};
use std::mem_map::flags::{MMAP_NONE, MREMAP_MAY_MOVE, PROT_READ, PROT_WRITE};
use std::string::{AsByteStr};

fn main() {
    let file = File::open_read("/etc/fstab").unwrap();
    let len = file.info().unwrap().size();
    let mmap = MemMap::file(&file, 0, len as usize, PROT_READ, false, MMAP_NONE).unwrap();
    println!("{:?}", mmap.as_byte_str());
}
