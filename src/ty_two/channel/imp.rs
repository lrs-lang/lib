// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::{ptr, mem, cmp};
use arch::atomic::{AtomicUsize, AtomicU8};
use lock::{self, Lock, Condvar};
use {error, alloc};

use super::{Error};

pub fn set_lock<T: Send>(p: &mut Packet<T>) {
    unsafe {
        p.recv_condvar.set_lock(p.sleep_lock.as_static());
        p.send_condvar.set_lock(p.sleep_lock.as_static());
    }
}

struct Node<T> {
    val: T,
    pos: AtomicUsize,
}

#[repr(C)]
pub struct Packet<T: Send> {
    // The buffer we store the massages in.
    buf: *mut Node<T>,
    // One less than the capacity of the channel. Note that the capacity is a power of
    // two.
    cap_mask: usize,

    next_write: AtomicUsize,
    next_read:  AtomicUsize,

    // Is the receiver sleeping?
    have_sleeping_receiver: AtomicU8,
    // Condvar the receiver is sleeping on.
    recv_condvar:           Condvar,

    // Number of senders that are currently sleeping.
    sleeping_senders: AtomicUsize,
    // Condvar the senders are sleeping on.
    send_condvar:     Condvar,

    receiver_disconnected: AtomicU8,
    num_senders:           AtomicUsize,

    // Lock that protects the two atomic variables above.
    sleep_lock: Lock,
}

impl<T: Send> Packet<T> {
    pub fn new(mut buf_size: usize) -> Result<Packet<T>> {
        buf_size = cmp::max(buf_size, 2);
        let cap = buf_size.checked_next_power_of_two().unwrap_or(!0);
        let size = cap.checked_mul(mem::size_of::<Node<T>>()).unwrap_or(!0);
        let buf = unsafe { alloc::allocate(size, mem::align_of::<T>()) };
        if buf.is_null() {
            return Err(error::NoMemory);
        }
        let packet = Packet {
            buf: buf as *mut Node<T>,
            cap_mask: cap - 1,

            next_write: AtomicUsize::new(0),
            next_read:  AtomicUsize::new(0),

            have_sleeping_receiver: AtomicU8::new(0),
            recv_condvar:           Condvar::new(&lock::DUMMY),

            sleeping_senders: AtomicUsize::new(0),
            send_condvar:     Condvar::new(&lock::DUMMY),

            receiver_disconnected: AtomicU8::new(0),
            num_senders:           AtomicUsize::new(1),

            sleep_lock: lock::INIT,
        };
        for i in 0..cap {
            packet.get_node(i).pos.store_seqcst(i);
        }
        Ok(packet)
    }

    fn sleep_lock(&self) -> &'static Lock {
        unsafe { self.sleep_lock.as_static() }
    }

    /// Call this function when the sender is cloned.
    pub fn add_sender(&self) {
        self.num_senders.add_seqcst(1);
    }

    /// Call this function when a sender is dropped.
    pub fn remove_sender(&self) {
        if self.num_senders.sub_seqcst(1) == 1 {
            let _guard = self.sleep_lock().lock();
            if self.have_sleeping_receiver.load_seqcst() == 1 {
                self.recv_condvar.signal(1);
            }
        }
    }

    /// Call this function when the consumer is dropped.
    pub fn remove_receiver(&self) {
        self.receiver_disconnected.store_seqcst(1);
        let _guard = self.sleep_lock().lock();
        if self.sleeping_senders.load_seqcst() > 0 {
            self.send_condvar.signal(1);
        }
    }

    fn get_node(&self, pos: usize) -> &mut Node<T> {
        unsafe { &mut *self.buf.add(pos & self.cap_mask) }
    }

    /// Get a position to write to if the queue isn't full
    fn get_write_pos(&self) -> Option<usize> {
        let mut next_write = self.next_write.load_seqcst();
        loop {
            let node = self.get_node(next_write);
            let diff = node.pos.load_seqcst() as isize - next_write as isize;
            if diff < 0 {
                return None;
            } else if diff > 0 {
                next_write = self.next_write.load_seqcst();
            } else {
                let next_write_old = next_write;
                next_write = self.next_write.compare_exchange_seqcst(next_write,
                                                                     next_write + 1);
                if next_write_old == next_write {
                    return Some(next_write);
                }
            }
        }
    }

    pub fn send_async(&self, val: T, have_lock: bool) -> Result<(), (T, Error)> {
        if self.receiver_disconnected.load_seqcst() == 1 {
            return Err((val, Error::Disconnected))
        }

        let write_pos = if let Some(w) = self.get_write_pos() {
            w
        } else {
            return if self.receiver_disconnected.load_seqcst() == 1 {
                Err((val, Error::Disconnected))
            } else {
                Err((val, Error::Full))
            };
        };
        {
            let node = self.get_node(write_pos);
            unsafe { ptr::write(&mut node.val, val); }
            node.pos.store_seqcst(write_pos + 1);
        }

        if self.have_sleeping_receiver.load_seqcst() == 1 {
            if have_lock {
                self.recv_condvar.signal(1);
            } else {
                let _guard = self.sleep_lock().lock();
                self.recv_condvar.signal(1);
            }
        }

        Ok(())
    }

    pub fn send_sync(&self, mut val: T) -> Result<(), (T, Error)> {
        val = match self.send_async(val, false) {
            Err((v, Error::Full)) => v,
            e @ Err(_) => return e,
            Ok(_) => return Ok(()),
        };

        let mut rv = Ok(());
        let mut guard = self.sleep_lock().lock();
        self.sleeping_senders.add_seqcst(1);
        loop {
            val = match self.send_async(val, true) {
                Err((v, Error::Full)) => v,
                e @ Err(_) => { rv = e; break; },
                Ok(_) => break,
            };
            guard = self.send_condvar.wait(guard);
        }
        self.sleeping_senders.sub_seqcst(1);

        rv
    }

    /// Get a position to read from if the queue isn't empty
    fn get_read_pos(&self) -> Option<usize> {
        let next_read = self.next_read.load_seqcst();
        let node = self.get_node(next_read);
        let diff = node.pos.load_seqcst() as isize - 1 - next_read as isize;
        if diff < 0 {
            None
        } else {
            assert!(diff == 0);
            self.next_read.store_seqcst(next_read + 1);
            Some(next_read)
        }
    }

    pub fn recv_async(&self, have_lock: bool) -> Result<T, Error> {
        let read_pos = if let Some(r) = self.get_read_pos() {
            r
        } else {
            return if self.num_senders.load_seqcst() == 0 {
                Err(Error::Disconnected)
            } else {
                Err(Error::Empty)
            };
        };
        let val;
        {
            let node = self.get_node(read_pos);
            val = unsafe { ptr::read(&node.val) };
            node.pos.store_seqcst(read_pos + self.cap_mask + 1);
        }

        if self.sleeping_senders.load_seqcst() > 0 {
            if have_lock {
                self.send_condvar.signal(1);
            } else {
                let _guard = self.sleep_lock().lock();
                self.send_condvar.signal(1);
            }
        }

        Ok(val)
    }

    pub fn recv_sync(&self) -> Result<T, Error> {
        match self.recv_async(false) {
            Err(Error::Empty) => { },
            e @ Err(_) => return e,
            v @ Ok(_) => return v,
        }

        let mut rv;
        let mut guard = self.sleep_lock().lock();
        self.have_sleeping_receiver.store_seqcst(1);
        loop {
            match self.recv_async(true) {
                Err(Error::Empty) => { },
                e @ Err(_) => { rv = e; break; },
                v @ Ok(_) => { rv = v; break; },
            }
            guard = self.recv_condvar.wait(guard);
        }
        self.have_sleeping_receiver.store_seqcst(0);

        rv
    }
}

unsafe impl<T: Send> Sync for Packet<T> { }

impl<T: Send> Drop for Packet<T> {
    fn drop(&mut self) {
        while self.recv_async(false).is_ok() { }
        
        unsafe {
            alloc::free(self.buf as *mut u8,
                        (self.cap_mask as usize + 1) * mem::size_of::<Node<T>>(),
                        mem::align_of::<Node<T>>());
        }
    }
}
