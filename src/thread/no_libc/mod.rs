// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use base::error::{InvalidArgument};
use core::{mem, ptr};
use core::marker::{Leak};
use mmem::{MemMap};
use mmem::flags::{PROT_WRITE, PROT_READ, PROT_NONE, MMAP_NONE};
use lock::{Lock, LockGuard};
use rt::imp::tls::{self, Private};
use rt::{aux};
use syscall::{self};
use signal::{self, Sigset};
use {at_exit_};

#[cfg(target_arch = "x86_64")] #[path = "x86_64.rs"] mod arch;
#[cfg(target_arch = "x86")] #[path = "x86.rs"] mod arch;
#[cfg(target_arch = "aarch64")] #[path = "aarch64.rs"] mod arch;
#[cfg(target_arch = "arm")] #[path = "arm.rs"] mod arch;

/// Default thread state.
const NOTHING: u8 = 0;

/// Thread has entered exiting procedures.
const EXITING: u8 = 1;

/// Thread has been detached.
const DETACHED: u8 = 2;

/// A join-guard
///
/// = Remarks
///
/// Note that this is `!Leak` because it allows other threads to reference objects on our
/// stack and those threads have to be joined before the end of the objects' lifetimes.
pub struct JoinGuard<'a> {
    thread: &'a tls::Private,
}

impl JoinGuard<'static> {
    pub fn detach(self) {
        if self.thread.status.exchange(DETACHED) == NOTHING {
            // Thread has not yet entered exiting procedures and will clean up after
            // itself. Don't wait for it to exit.
            unsafe { mem::unsafe_forget(self); }
        }
    }
}

impl<'a> Drop for JoinGuard<'a> {
    fn drop(&mut self) {
        loop {
            // Wait for the kernel to signal the thread's death.

            let tid = self.thread.thread_id.load();
            if tid == 0 { break; }
            syscall::futex_wait(&self.thread.thread_id, tid, None);
        }
        unsafe {
            syscall::munmap(self.thread.mem_base as usize, self.thread.mem_size);
        }
    }
}

impl<'a> !Leak for JoinGuard<'a> { }

/// A thread-builder
///
/// = Remarks
///
/// This can be used to modify properties of the thread before spawning it.
#[derive(Pod)]
pub struct Builder {
    /// The size of the guard area in bytes. At least one page.
    guard_size: usize,
    /// The size of the user's stack excluding the guard area. At least one page.
    user_stack_size: usize,
}

impl Builder {
    /// Creates a new thread-builder.
    pub fn new() -> Result<Builder> {
        Ok(Builder {
            guard_size: aux::page_size(),
            user_stack_size: 2 << 23, // 8MB
        })
    }

    /// Sets the size of the guard page at the end of the thread's stack.
    ///
    /// [argument, size]
    /// The size of the guard page.
    pub fn set_guard_size(&mut self, size: usize) -> Result {
        if size < aux::page_size() {
            Err(InvalidArgument)
        } else {
            self.guard_size = size;
            Ok(())
        }
    }

    /// Returns the size of the guard page at the end of the thread's stack.
    pub fn guard_size(&self) -> Result<usize> {
        Ok(self.guard_size)
    }

    /// Sets the size of the thread's stack.
    ///
    /// [argument, size]
    /// The size of the thread's stack.
    pub fn set_stack_size(&mut self, size: usize) -> Result {
        if size < aux::page_size() {
            Err(InvalidArgument)
        } else {
            self.user_stack_size = size;
            Ok(())
        }
    }

    /// Returns the size of the thread's stack.
    pub fn stack_size(&self) -> Result<usize> {
        Ok(self.user_stack_size)
    }

    /// Spawns a new thread.
    ///
    /// [argument, f]
    /// The closure that will be run in the new thread.
    pub fn spawn<F>(self, f: F) -> Result
        where F: FnOnce() + Send + 'static
    {
        unsafe {
            match self.spawn_inner(&f) {
                Err(e) => Err(e),
                Ok(o) => {
                    mem::unsafe_forget(f);
                    o.detach();
                    Ok(())
                },
            }
        }
    }

    /// Spawns a new scoped thread.
    ///
    /// [argument, f]
    /// The closure that will be run in the new thread.
    ///
    /// = Remarks
    ///
    /// The thread will automatically be joined when the guard's destructor runs.
    pub fn scoped<'a, F>(self, f: F) -> Result<JoinGuard<'a>>
        where F: FnOnce() + Send + 'a
    {
        unsafe {
            match self.spawn_inner(&f) {
                Err(e) => Err(e),
                ok => {
                    mem::unsafe_forget(f);
                    ok
                },
            }
        }
    }

    unsafe fn spawn_inner<'a, F>(self, f: &F) -> Result<JoinGuard<'a>>
        where F: FnOnce() + Send + 'a,
    {
        // The memory will be organized as follows:
        //
        // -------------------------------------------------
        // | guard pages |         stack        | tls area |
        // -------------------------------------------------
        //         _____/ \______        ______/ \______
        //        /page alignment\      /stack alignment\

        const STACK_ALIGNMENT: usize = 16;

        let page_size = aux::page_size();
        let guard_size = align!(self.guard_size, [%] page_size);
        let stack_size = align!(self.user_stack_size, [%] STACK_ALIGNMENT);
        let map_size = align!(guard_size + stack_size + tls::size(), [%] page_size);

        // Don't forget to mem::forget this below in the success case.
        let mut map = try!(MemMap::anon(map_size, PROT_NONE, false, MMAP_NONE));
        try!(map.protect(guard_size.., PROT_READ | PROT_WRITE));

        let stack = map.as_mut_ptr().add(guard_size + stack_size);
        let (private, tp) = tls::place(stack);

        (*private).mem_base = map.as_mut_ptr();
        (*private).mem_size = map_size;
        (*private).status.store(NOTHING);

        // We have to block all signals so that the cloned thread doesn't get interrupted
        // before it had time to set up its stack.
        let set = signal::block_all().unwrap();

        let lock = Lock::new();
        let guard = lock.lock();
        let mut payload = Payload {
            guard: guard,
            f: f,
            sigs: set,
        };

        try!(start_thread(stack, &mut payload, tp, private));
        lock.lock();

        signal::set_blocked_signals(set);

        mem::forget(payload.guard);
        mem::forget(map);

        Ok(JoinGuard { thread: &mut *private })
    }
}

/// Payload to be passed to another thread.
struct Payload<'a, F>
    where F: FnOnce() + Send,
{
    guard: LockGuard<'a>,
    f: *const F,
    sigs: Sigset,
}

/// The function that will be called by start_thread. The function takes ownership of the
/// payload and the `f`.
unsafe extern fn start<'a, F>(data: *mut Payload<'a, F>) -> !
    where F: FnOnce() + Send,
{
    let Payload { guard, f, sigs } = ptr::read(data);
    let f = ptr::read(f);
    drop(guard);
    signal::set_blocked_signals(sigs);
    f();

    at_exit_::run();

    let private = tls::private();

    if private.status.exchange(EXITING) == DETACHED {
        // We have already been detached which means that we have to clean up after
        // ourselves. This includes unmapping our own stack. Since working without a stack
        // isn't so easy, we temporarily swap out our own stack for a globally shared
        // stack. This stack is protected by a lock.

        static LOCK: Lock = Lock::new();
        static mut STACK: [u8; 256] = [0; 256];

        let _guard = LOCK.lock();

        // We use the stack up to the point where this thread exits, so we don't have a
        // chance to unlock the lock ourselves. Luckily, the kernel will store a `0` in
        // and call `futex_wake` on the `tid` address once this thread exits. We set this
        // address to the address of the lock. Since `0` means unlocked, this will unlock
        // the lock for us.
        syscall::set_tid_address(Some(LOCK.as_atomic()));

        // Block all signals so that we don't get interrupted after we've destroyed our
        // stack.
        signal::block_all().unwrap();

        stop_thread(private.mem_base, private.mem_size,
                    STACK.as_mut_ptr().add(mem::size_of_val(&STACK)))
    }

    syscall::exit(0);
}

/// Starts a new thread.
///
/// [argument, stack]
/// The stack of the new thread.
///
/// [argument, payload]
/// The payload to be passed ot the start function.
///
/// [argument, tp]
/// The thread pointer.
///
/// [argument, private]
/// The private area of the thread.
unsafe fn start_thread<F>(stack: *mut u8, payload: &mut Payload<F>, tp: *mut u8,
                          private: *mut Private) -> Result
    where F: FnOnce() + Send,
{
    use cty::{
        c_int,
        CLONE_VM, CLONE_FS, CLONE_FILES, CLONE_SIGHAND, CLONE_THREAD, CLONE_SYSVSEM,
        CLONE_SETTLS, CLONE_CHILD_SETTID, CLONE_CHILD_CLEARTID,
    };

    const FLAGS: c_int = CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND |
                            CLONE_THREAD | CLONE_SYSVSEM | CLONE_SETTLS |
                            CLONE_CHILD_SETTID | CLONE_CHILD_CLEARTID;

    let start_fn: unsafe extern fn(*mut u8) -> ! = mem::cast(start::<F>);
    let arg = payload as *mut _ as *mut _;

    rv!(arch::start_thread(start_fn, arg, FLAGS, stack, &(*private).thread_id, tp))
}

/// Unmaps this thread's stack and exits.
///
/// [argument, stack_base]
/// The base of this thread's stack.
///
/// [argument, stack_size]
/// The size of this thread's stack.
///
/// [argument, tmp_stack]
/// A temporary stack that we can use after unmapping our stack.
unsafe fn stop_thread(stack_base: *mut u8, stack_size: usize, tmp_stack: *mut u8) -> ! {
    arch::stop_thread(stack_base, stack_size, tmp_stack)
}

pub fn at_exit<F>(f: F) -> Result
    where F: FnOnce() + 'static,
{
    at_exit_::at_exit(f)
}
