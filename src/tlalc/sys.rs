// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use cty::{PROT_READ, PROT_WRITE, MAP_ANONYMOUS, MAP_PRIVATE, c_int};
use syscall::{mmap, munmap};
use base::error::{Errno};

use chunk::{Chunk};
use {CHUNK_SIZE, CHUNK_MASK};

pub unsafe fn map<T>(size: usize) -> Result<*mut T> {
    let ptr = mmap(0, size, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    if ptr < 0 && -ptr < 4096 {
        Err(Errno(-ptr as c_int))
    } else {
        Ok(ptr as usize as *mut T)
    }
}

pub unsafe fn map_chunk() -> Result<*mut Chunk> {
    let mut ptr: *mut u8 = try!(map(CHUNK_SIZE));
    if ptr as usize & CHUNK_MASK == 0 {
        return Ok(ptr as *mut Chunk)
    }
    unmap(ptr, CHUNK_SIZE);

    ptr = try!(map(2 * CHUNK_SIZE));

    let addr = ptr as usize;
    let real_addr = align!(addr, [%] CHUNK_SIZE);

    let low_lost = real_addr - addr;
    let high_lost = CHUNK_SIZE - low_lost;

    if low_lost > 0 {
        unmap(ptr, low_lost);
    }

    if high_lost > 0 {
        unmap(ptr.add(CHUNK_SIZE + low_lost), high_lost);
    }

    Ok(real_addr as *mut Chunk)
}

pub unsafe fn unmap<T>(ptr: *const T, size: usize) -> Result {
    let res = munmap(ptr as usize, size);
    if res < 0 {
        Err(Errno(-res as c_int))
    } else {
        Ok(())
    }
}
