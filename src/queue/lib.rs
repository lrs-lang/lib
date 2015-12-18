// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_queue"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cell as cell;
extern crate lrs_arch_fns as arch_fns;
extern crate lrs_atomic as atomic;
extern crate lrs_lock as lock;
extern crate lrs_alloc as alloc;

use base::prelude::*;
use core::ptr::{NoAliasMemPtr};
use base::{error};
use alloc::{MemPool, empty_ptr};
use arch_fns::{spin};
use atomic::{AtomicUsize};
use cell::cell::{Cell};
use core::{ptr, mem};
use lock::{Lock, LockGuard, RawCondvar, LOCK_INIT, RAW_CONDVAR_INIT};

pub mod std { pub use base::std::*; }

/// A multi-threaded queue.
///
/// = Remarks
///
/// This queue can be used for sending messages between threads.
pub struct Queue<T, Heap = alloc::Heap>
    where Heap: MemPool,
{
    // The buffer we store the massages in.
    buf: NoAliasMemPtr<Cell<T>>,
    // One less than the capacity of the channel. Note that the capacity is a power of
    // two.
    cap_mask: usize,

    // The place before which all elements in the buffer have been read.
    read_start: AtomicUsize,
    // The next place that's free for writing.
    next_write: AtomicUsize,

    // The place before which all elements in the buffer have been written.
    write_end:   AtomicUsize,
    // The next place that's free for reading.
    next_read:   AtomicUsize,

    // Number of senders that are currently sleeping.
    sleeping_senders: AtomicUsize,
    // Condvar the senders are sleeping on.
    send_condvar:     RawCondvar,

    // Number of receivers that are currently sleeping.
    sleeping_receivers: AtomicUsize,
    // Condvar the senders are sleeping on.
    recv_condvar:       RawCondvar,

    // Mutex that protects the two atomic variables above.
    sleep_lock: Lock,

    pool: Heap,

    _marker: PhantomData<Cell<T>>,
}

impl<T, H> Queue<T, H>
    where H: MemPool,
{
    /// Creates a new queue with allocated memory.
    ///
    /// [argument, cap]
    /// The number of elements that can be stored in the queue.
    ///
    /// = Remarks
    ///
    /// The capacity will be increased to the next power of two.
    pub fn new(cap: usize) -> Result<Queue<T, H>>
        where H: OutOf,
    {
        Self::with_pool(cap, H::out_of(()))
    }

    /// Creates a new queue with allocated memory.
    ///
    /// [argument, cap]
    /// The number of elements that can be stored in the queue.
    ///
    /// [argument, pool]
    /// The memory pool from which the queue will be allocated.
    ///
    /// = Remarks
    ///
    /// The capacity will be increased to the next power of two.
    pub fn with_pool(cap: usize, mut pool: H) -> Result<Queue<T, H>> {
        let cap = match cap.checked_next_power_of_two() {
            Some(c) => c,
            _ => return Err(error::NoMemory),
        };
        let buf = match mem::size_of::<T>() {
            0 => empty_ptr(),
            _ => unsafe { try!(alloc::alloc_array(&mut pool, cap)).0 },
        };
        Ok(Queue {
            buf: unsafe { NoAliasMemPtr::new(buf) },
            cap_mask: cap - 1,

            read_start: AtomicUsize::new(0),
            next_write: AtomicUsize::new(0),

            write_end: AtomicUsize::new(0),
            next_read: AtomicUsize::new(0),

            sleeping_senders: AtomicUsize::new(0),
            send_condvar:     RAW_CONDVAR_INIT,

            sleeping_receivers: AtomicUsize::new(0),
            recv_condvar:       RAW_CONDVAR_INIT,

            sleep_lock: LOCK_INIT,

            pool: pool,

            _marker: PhantomData,
        })
    }

    /// Get a position to write to if the queue isn't full
    fn get_write_pos(&self) -> Option<usize> {
        loop {
            let read_start = self.read_start.load();
            let next_write = self.next_write.load();
            if next_write.wrapping_sub(read_start) > self.cap_mask {
                return None;
            }
            let next_next_write = next_write.wrapping_add(1);
            if self.next_write.compare_exchange(next_write,
                                                next_next_write) == next_write {
                if cfg!(target_pointer_width = "32") {
                    let read_start = self.read_start.load();
                    if next_write.wrapping_sub(read_start) > self.cap_mask {
                        abort!();
                    }
                }
                return Some(next_write);
            }
            spin();
        }
    }

    /// `pos` is the position we've written to
    fn set_write_end(&self, pos: usize) {
        let mut write_end = self.write_end.load();
        while write_end != pos {
            spin();
            write_end = self.write_end.load();
        }

        self.write_end.store(pos.wrapping_add(1));
    }

    fn set_mem(&self, pos: usize, val: T) {
        unsafe {
            ptr::write(self.buf.get().add(pos & self.cap_mask), Cell::new(val));
        }
    }

    fn push_int(&self, val: T, guard: Option<&LockGuard>) -> Option<T> {
        let write_pos = match self.get_write_pos() {
            Some(w) => w,
            _ => return Some(val),
        };
        self.set_mem(write_pos, val);
        self.set_write_end(write_pos);

        if self.sleeping_receivers.load() > 0 {
            let _guard = match guard.is_some() {
                true => None,
                _ => Some(self.sleep_lock.lock()),
            };
            self.recv_condvar.signal(1);
        }
        None
    }

    /// Tries to add an element to the queue.
    ///
    /// [argument, val]
    /// The element to be added.
    ///
    /// [return_value]
    /// Returns the element if the queue is full.
    pub fn push(&self, val: T) -> Option<T> {
        self.push_int(val, None)
    }

    /// Blocks until it can add the element to the queue.
    ///
    /// [argument, val]
    /// The element to be added.
    pub fn push_wait(&self, mut val: T) {
        val = match self.push_int(val, None) {
            Some(v) => v,
            _ => return,
        };

        let mut guard = self.sleep_lock.lock();
        self.sleeping_senders.add(1);
        loop {
            val = match self.push_int(val, Some(&guard)) {
                Some(v) => v,
                _ => break,
            };
            guard = self.send_condvar.wait(guard);
        }
        self.sleeping_senders.sub(1);
    }

    /// Get a position to read from if the queue isn't empty
    fn get_read_pos(&self) -> Option<usize> {
        loop {
            let write_end = self.write_end.load();
            let next_read = self.next_read.load();
            let next_next_read = next_read.wrapping_add(1);
            if write_end.wrapping_sub(next_next_read) > self.cap_mask {
                return None;
            }
            if self.next_read.compare_exchange(next_read, next_next_read) == next_read {
                if cfg!(target_pointer_width = "32") {
                    let write_end = self.write_end.load();
                    if write_end.wrapping_sub(next_next_read) > self.cap_mask {
                        abort!();
                    }
                }
                return Some(next_read);
            }
            spin();
        }
    }

    /// `pos` is the position we've read from
    fn set_read_start(&self, pos: usize) {
        let mut read_start = self.read_start.load();
        while read_start != pos {
            spin();
            read_start = self.read_start.load();
        }

        self.read_start.store(pos.wrapping_add(1));
    }

    fn get_mem(&self, pos: usize) -> T {
        unsafe {
            // old code left here as a warning to future generations
            //
            // ptr::read(ptr::read(self.buf.add(pos & self.cap_mask)).ptr())
            //
            // wew, lad
            ptr::read(self.buf.get().add(pos & self.cap_mask)).into()
        }
    }

    fn pop_int(&self, guard: Option<&LockGuard>) -> Option<T> {
        let read_pos = match self.get_read_pos() {
            Some(r) => r,
            _ => return None,
        };
        let val = self.get_mem(read_pos);
        self.set_read_start(read_pos);

        if self.sleeping_senders.load() > 0 {
            let _guard = match guard.is_some() {
                true => None,
                false => Some(self.sleep_lock.lock()),
            };
            self.send_condvar.signal(1);
        }

        Some(val)
    }

    /// Removes an element to the queue.
    ///
    /// [return_value]
    /// Returns the removed element.
    pub fn pop(&self) -> Option<T> {
        self.pop_int(None)
    }

    /// Blocks until there is an element in the queue.
    ///
    /// [return_value]
    /// Returns the removed element.
    pub fn pop_wait(&self) -> T {
        let mut rv = self.pop_int(None);

        if rv.is_none() {
            let mut guard = self.sleep_lock.lock();
            self.sleeping_receivers.add(1);
            loop {
                rv = self.pop_int(Some(&guard));
                if rv.is_some() {
                    break;
                }
                guard = self.recv_condvar.wait(guard);
            }
            self.sleeping_receivers.sub(1);
        }

        rv.unwrap()
    }
}

unsafe impl<T, H> Send for Queue<T, H> where T: Send, H: MemPool+Send { }
unsafe impl<T, H> Sync for Queue<T, H> where T: Send, H: MemPool { }

impl<T, H> Drop for Queue<T, H>
    where H: MemPool,
{
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                let write_end = self.write_end.load();
                let read_start = self.read_start.load();
                for i in 0..write_end-read_start {
                    self.get_mem(read_start + i);
                }
            }

            if mem::size_of::<T>() > 0 {
                alloc::free_array(&mut self.pool, self.buf.get(), self.cap_mask + 1);
            }
        }
    }
}
