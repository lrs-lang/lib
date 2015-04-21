// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_queue"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_cell as cell;
extern crate linux_arch_fns as arch_fns;
extern crate linux_atomic as atomic;
extern crate linux_lock as lock;
extern crate linux_alloc as alloc;

#[prelude_import] use base::prelude::*;
use base::{error};
use alloc::{allocate_array, free_array, empty_ptr};
use arch_fns::{spin};
use atomic::{AtomicUsize};
use cell::cell::{Cell};
use core::{ptr, mem};
use lock::{Lock, LockGuard, RawCondvar, LOCK_INIT, RAW_CONDVAR_INIT};

pub mod linux { pub use base::linux::*; }

/// A queue.
pub struct Queue<T> {
    // The buffer we store the massages in.
    buf: *mut Cell<T>,
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
}

impl<T> Queue<T> {
    /// Creates a new queue with capacity at least `cap`.
    pub fn new(cap: usize) -> Result<Queue<T>> {
        let cap = match cap.checked_next_power_of_two() {
            Some(c) => c,
            _ => return Err(error::NoMemory),
        };
        let buf = match mem::size_of::<T>() {
            0 => empty_ptr(),
            _ => unsafe { allocate_array(cap) },
        };
        if buf.is_null() {
            return Err(error::NoMemory);
        }
        Ok(Queue {
            buf: buf,
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
        })
    }

    /// Get a position to write to if the queue isn't full
    fn get_write_pos(&self) -> Option<usize> {
        loop {
            let read_start = self.read_start.load_seqcst();
            let next_write = self.next_write.load_seqcst();
            if next_write.wrapping_sub(read_start) > self.cap_mask {
                return None;
            }
            let next_next_write = next_write.wrapping_add(1);
            if self.next_write.compare_exchange_seqcst(next_write,
                                                       next_next_write) == next_write {
                if cfg!(target_pointer_width = "32") {
                    let read_start = self.read_start.load_seqcst();
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
        let mut write_end = self.write_end.load_seqcst();
        while write_end != pos {
            spin();
            write_end = self.write_end.load_seqcst();
        }

        self.write_end.store_seqcst(pos.wrapping_add(1));
    }

    fn set_mem(&self, pos: usize, val: T) {
        unsafe {
            ptr::write(self.buf.add(pos & self.cap_mask), Cell::new(val));
        }
    }

    fn push_int(&self, val: T, guard: Option<&LockGuard>) -> Option<T> {
        let write_pos = match self.get_write_pos() {
            Some(w) => w,
            _ => return Some(val),
        };
        self.set_mem(write_pos, val);
        self.set_write_end(write_pos);

        if self.sleeping_receivers.load_seqcst() > 0 {
            let _guard = match guard.is_some() {
                true => None,
                _ => Some(self.sleep_lock.lock()),
            };
            self.recv_condvar.signal(1);
        }
        None
    }

    /// Tries to add an element to the queue. Returns the element if the queue is full.
    pub fn push(&self, val: T) -> Option<T> {
        self.push_int(val, None)
    }

    /// Blocks until it can add the element to the queue.
    pub fn push_wait(&self, mut val: T) {
        val = match self.push_int(val, None) {
            Some(v) => v,
            _ => return,
        };

        let mut guard = self.sleep_lock.lock();
        self.sleeping_senders.add_seqcst(1);
        loop {
            val = match self.push_int(val, Some(&guard)) {
                Some(v) => v,
                _ => break,
            };
            guard = self.send_condvar.wait(guard);
        }
        self.sleeping_senders.sub_seqcst(1);
    }

    /// Get a position to read from if the queue isn't empty
    fn get_read_pos(&self) -> Option<usize> {
        loop {
            let write_end = self.write_end.load_seqcst();
            let next_read = self.next_read.load_seqcst();
            let next_next_read = next_read.wrapping_add(1);
            if write_end.wrapping_sub(next_next_read) > self.cap_mask {
                return None;
            }
            if self.next_read.compare_exchange_seqcst(next_read,
                                                      next_next_read) == next_read {
                if cfg!(target_pointer_width = "32") {
                    let write_end = self.write_end.load_seqcst();
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
        let mut read_start = self.read_start.load_seqcst();
        while read_start != pos {
            spin();
            read_start = self.read_start.load_seqcst();
        }

        self.read_start.store_seqcst(pos.wrapping_add(1));
    }

    fn get_mem(&self, pos: usize) -> T {
        unsafe {
            ptr::read(self.buf.add(pos & self.cap_mask)).data
        }
    }

    fn pop_int(&self, guard: Option<&LockGuard>) -> Option<T> {
        let read_pos = match self.get_read_pos() {
            Some(r) => r,
            _ => return None,
        };
        let val = self.get_mem(read_pos);
        self.set_read_start(read_pos);

        if self.sleeping_senders.load_seqcst() > 0 {
            let _guard = match guard.is_some() {
                true => None,
                false => Some(self.sleep_lock.lock()),
            };
            self.send_condvar.signal(1);
        }

        Some(val)
    }

    /// Removes an element to the queue. Returns None if the queue is empty.
    pub fn pop(&self) -> Option<T> {
        self.pop_int(None)
    }

    /// Blocks until there is an element in the queue.
    pub fn pop_wait(&self) -> T {
        let mut rv = self.pop_int(None);

        if rv.is_none() {
            let mut guard = self.sleep_lock.lock();
            self.sleeping_receivers.add_seqcst(1);
            loop {
                rv = self.pop_int(Some(&guard));
                if rv.is_some() {
                    break;
                }
                guard = self.recv_condvar.wait(guard);
            }
            self.sleeping_receivers.sub_seqcst(1);
        }

        rv.unwrap()
    }
}

unsafe impl<T: Send> Send for Queue<T> { }
unsafe impl<T: Send> Sync for Queue<T> { }

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                let write_end = self.write_end.load_seqcst();
                let read_start = self.read_start.load_seqcst();
                for i in 0..write_end-read_start {
                    self.get_mem(read_start + i);
                }
            }

            if mem::size_of::<T>() > 0 {
                free_array(self.buf, self.cap_mask + 1);
            }
        }
    }
}
