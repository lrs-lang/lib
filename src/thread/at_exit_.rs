// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use base::{error};
use core::{ptr, mem};
use {rt};
use rt::{AtExit};
use lock::{SingleThreadMutexGuard};
use alloc::{Bda, MemPool};

pub fn at_exit<F>(f: F) -> Result
    where F: FnOnce() + 'static,
{
    // The memory is organized as follows
    //
    //        -------------------------------------------------
    //        | size | invoke | closure | padding | size ...
    //        -------------------------------------------------
    //   ____/ \____                      _______/ \________
    //  /memory base\                    /memory base + size\
    //
    // Where `size` is the size of this section plus padding.
    // Where `invoke` is a pointer to the function that will invoke the closure.
    // Where `closure` is the data of the closure.
    // Where `padding` is enough padding to align the next `size` to an appropriate
    // boundary.
    //
    // Note that the alignment of the closure doesn't matter, since `invoke` will copy it
    // onto its stack.

    let mut at_exit = match rt::at_exit().try_lock() {
        Some(g) => g,
        _ => return Err(error::ResourceBusy),
    };

    let needed = mem::size_of::<usize>() + mem::size_of::<*mut u8>()
                        + mem::size_of::<F>();
    let needed = align!(needed, [%] mem::align_of::<Entry>());

    if at_exit.cap - at_exit.len < needed {
        let ps = rt::aux::page_size();
        let new_cap = align!(at_exit.cap + needed, [%] ps);
        unsafe {
            at_exit.ptr = if at_exit.ptr.is_null() {
                try!(Bda.alloc(new_cap, ps))
            } else {
                try!(Bda.realloc(at_exit.ptr, (*at_exit).cap, new_cap, ps))
            };
        }
        at_exit.cap = new_cap;
    }

    unsafe {
        let ptr = at_exit.ptr.add(at_exit.len) as *mut Entry;
        (*ptr).size = needed;
        (*ptr).invoke = invoke::<F>;
        ptr::write(&mut (*ptr).data as *mut _ as *mut _, f);
    }

    at_exit.len += needed;

    Ok(())
}

#[repr(C)]
struct Entry {
    size: usize,
    invoke: unsafe extern fn(SingleThreadMutexGuard<AtExit>, *mut u8),
    data: u8,
}

/// Runs all closures and deallocates the memory. This function is not reentrant.
pub unsafe fn run() {
    let at_exit = rt::at_exit();
    let mut pos = 0;

    loop {
        let mut at_exit = at_exit.lock();
        if pos == at_exit.len {
            if pos != 0 {
                Bda.free(at_exit.ptr, at_exit.cap, 1);
                at_exit.ptr = 0 as *mut u8;
                at_exit.len = 0;
                at_exit.cap = 0;
            }
            return;
        }
        let ptr = at_exit.ptr.add(pos) as *mut Entry;
        pos += (*ptr).size;
        ((*ptr).invoke)(at_exit, &mut (*ptr).data);
    }
}

/// Reads the closure onto its stack, drops the guard, and invokes the closure.
unsafe extern fn invoke<F>(guard: SingleThreadMutexGuard<AtExit>, f: *mut u8)
    where F: FnOnce() + 'static,
{
    let f = ptr::read(f as *mut F);
    drop(guard);
    f();
}
