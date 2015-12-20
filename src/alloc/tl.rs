// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use base::{error};
use {MemPool};
use lock::{SingleThreadMutex};
use tlalc::{Cache};

/// The tlalc allocator
#[derive(Copy)]
pub struct TlAlc;

impl !Send for TlAlc { }

impl OutOf for TlAlc {
    fn out_of(_: ()) -> TlAlc {
        TlAlc
    }
}

thread_local! {
    static CACHE: SingleThreadMutex<Cache> = unsafe {
        SingleThreadMutex::new(Cache::new())
    };
}

impl MemPool for TlAlc {
    unsafe fn alloc(&mut self, size: usize, _: usize) -> Result<*mut d8> {
        if let Some(mut c) = CACHE.try_lock() {
            c.alloc(size)
        } else {
            Err(error::ResourceBusy)
        }
    }

    unsafe fn free(&mut self, ptr: *mut d8, size: usize, _: usize) {
        CACHE.lock().free(ptr, size)
    }

    unsafe fn realloc(&mut self, old_ptr: *mut d8, oldsize: usize, newsize: usize,
                      _: usize) -> Result<*mut d8> {
        CACHE.lock().realloc(old_ptr, oldsize, newsize)
    }
}
