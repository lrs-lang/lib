// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_thread"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits, custom_derive)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_libc as libc;
extern crate linux_syscall as syscall;
extern crate linux_cty as cty;
extern crate linux_lock as lock;

#[prelude_import] use base::prelude::*;
use core::marker::{Leak};
use core::ops::{Drop};
use core::{mem, ptr, intrinsics};
use cty::{c_int};
use libc::{pthread_t, pthread_attr_t, PTHREAD_CREATE_DETACHED};
use lock::{LockGuard, LOCK_INIT};

mod linux { pub use base::linux::*; pub use cty; }

/// Terminates the current thread.
///
/// This is unsafe because other threads might be referencing our stack.
pub unsafe fn exit(code: c_int) -> ! {
    syscall::exit(code)
}

/// Spawns a new thread starting at the closure.
pub fn spawn<F>(f: F) -> Result
    where F: FnOnce() + Send + 'static
{
    Builder::new().chain(|b| b.spawn(f))
}

/// Spawns a new scoped thread starting at the closure.
///
/// The thread will automatically be joined when the guard's destructor runs.
pub fn scoped<'a, F>(f: F) -> Result<JoinGuard<'a>>
    where F: FnOnce() + Send + 'a
{
    Builder::new().chain(|b| b.scoped(f))
}

/// A join-guard
///
/// Note that this is `!Leak` because it allows other threads to reference objects on our
/// stack and those threads have to be joined before the end of the objects' lifetimes.
pub struct JoinGuard<'a> {
    thread: pthread_t,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Drop for JoinGuard<'a> {
    fn drop(&mut self) {
        unsafe { libc::pthread_join(self.thread, 0 as *mut _); }
    }
}

impl<'a> !Leak for JoinGuard<'a> { }

/// A thread-builder
///
/// This can be used to modify properties of the thread before spawning it.
#[derive(Pod)]
pub struct Builder {
    attr: pthread_attr_t,
}

impl Builder {
    /// Creates a new thread-builder.
    pub fn new() -> Result<Builder> {
        unsafe {
            let mut attr = mem::zeroed();
            try!(rv!(-libc::pthread_attr_init(&mut attr)));
            Ok(Builder { attr: attr })
        }
    }

    /// Sets the size of the guard page at the end of the thread's stack.
    pub fn set_guard_size(&mut self, size: usize) -> Result {
        unsafe { rv!(-libc::pthread_attr_setguardsize(&mut self.attr, size)) }
    }

    /// Like `set_guard_size`.
    pub fn guard_size(&mut self) -> Result<usize> {
        unsafe {
            let mut size = 0;
            try!(rv!(-libc::pthread_attr_getguardsize(&self.attr, &mut size)));
            Ok(size)
        }
    }

    /// Sets the size of the thread's stack.
    pub fn set_stack_size(&mut self, size: usize) -> Result {
        unsafe { rv!(-libc::pthread_attr_setstacksize(&mut self.attr, size)) }
    }

    /// Like `set_stack_size`.
    pub fn stack_size(&mut self) -> Result<usize> {
        unsafe {
            let mut size = 0;
            try!(rv!(-libc::pthread_attr_getstacksize(&mut self.attr, &mut size)));
            Ok(size)
        }
    }

    /// See the documentation of the top-level function.
    pub fn spawn<F>(mut self, f: F) -> Result
        where F: FnOnce() + Send + 'static
    {
        unsafe {
            try!(rv!(-libc::pthread_attr_setdetachstate(&mut self.attr,
                                                        PTHREAD_CREATE_DETACHED)));
            match self.spawn_inner(&f) {
                Ok(_) => {
                    intrinsics::forget(f);
                    Ok(())
                },
                Err(e) => Err(e),
            }
        }
    }

    /// See the documentation of the top-level function.
    pub fn scoped<'a, F>(self, f: F) -> Result<JoinGuard<'a>>
        where F: FnOnce() + Send + 'a
    {
        unsafe {
            match self.spawn_inner(&f) {
                Ok(thread) => {
                    intrinsics::forget(f);
                    Ok(JoinGuard { thread: thread, _marker: PhantomData })
                },
                Err(e) => Err(e),
            }
        }
    }

    unsafe fn spawn_inner<F>(self, f: &F) -> Result<pthread_t>
        where F: FnOnce() + Send,
    {
        // We use the following method to get `f` onto the other thread's stack without an
        // allocation:
        //
        // - Create a lock in the shared address space and lock it.
        // - Pass a reference to the lock-guard and `f` to the other thread.
        // - Try to lock the lock again.
        // - The other thread copies everything onto its stack and then drops the lock
        //   guard.
        // - Once it drops the lock guard our lock succeeds and we know that the other
        //   thread now has its own copy of `f`.
        // - We forget our original lock guard because it has already been dropped in the
        //   other thread and forget `f` because the other thread now owns it.

        let mut thread = mem::zeroed();
        let lock = LOCK_INIT;
        let guard = lock.lock();

        // Avoid moving f around. If we move it into the structure then the compiler
        // definitely has to copy it instead of leaving it in the user's stack and only
        // passing references to our functions. This has been observed to avoid at least
        // one copy.

        let mut payload = Payload { guard: guard, f: f };
        let start_fn: unsafe extern fn(*mut u8) -> *mut u8 = mem::cast(start::<F>);
        let rv = libc::pthread_create(&mut thread, &self.attr, start_fn,
                                      &mut payload as *mut _ as *mut _);
        try!(rv!(-rv));
        lock.lock();
        mem::forget(payload.guard);
        Ok(thread)
    }
}

/// Payload to be passed to another thread.
struct Payload<'a, F>
    where F: FnOnce() + Send,
{
    guard: LockGuard<'a>,
    f: *const F,
}

/// The function that will be called by libc. The function takes ownership of the payload
/// and the `f`.
unsafe extern fn start<'a, F>(data: *mut Payload<'a, F>) -> *mut u8
    where F: FnOnce() + Send,
{
    let Payload { guard, f } = ptr::read(data);
    let f = ptr::read(f);
    drop(guard);
    f();
    0 as *mut u8
}
