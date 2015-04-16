use std::{ptr, mem, cmp};
use std::num::{Int};
use std::sync::atomic::{AtomicUsize, AtomicBool};
use std::sync::atomic::Ordering::{SeqCst};
use std::sync::{Mutex, Condvar};
use std::rt::heap::{allocate, deallocate};
use std::cell::{Cell};

use select::{_Selectable, WaitQueue, Payload};
use alloc::{oom};
use {Error, Sendable};

struct Node<T> {
    val: T,
    pos: AtomicUsize,
}

#[repr(C)]
pub struct Packet<'a, T: Sendable+'a> {
    // The buffer we store the massages in.
    buf: *mut Node<T>,
    // One less than the capacity of the channel. Note that the capacity is a power of
    // two.
    cap_mask: usize,

    next_write: AtomicUsize,
    next_read: AtomicUsize,

    // Is the receiver sleeping?
    have_sleeping_receiver: AtomicBool,
    // Condvar the receiver is sleeping on.
    recv_condvar:           Condvar,

    // Number of senders that are currently sleeping.
    sleeping_senders: AtomicUsize,
    // Condvar the senders are sleeping on.
    send_condvar:     Condvar,

    receiver_disconnected: AtomicBool,
    num_senders: AtomicUsize,

    // Mutex that protects the two atomic variables above.
    sleep_mutex: Mutex<()>,
}

impl<'a, T: Sendable+'a> Packet<'a, T> {
    pub fn new(mut buf_size: usize) -> Packet<'a, T> {
        buf_size = cmp::max(buf_size, 2);
        let cap = buf_size.checked_next_power_of_two().unwrap_or(!0);
        let size = cap.checked_mul(mem::size_of::<Node<T>>()).unwrap_or(!0);
        let buf = unsafe { allocate(size, mem::min_align_of::<T>()) };
        if buf.is_null() {
            oom();
        }
        let packet = Packet {
            buf: buf as *mut Node<T>,
            cap_mask: cap - 1,

            next_write: AtomicUsize::new(0),
            next_read: AtomicUsize::new(0),

            have_sleeping_receiver: AtomicBool::new(false),
            recv_condvar:           Condvar::new(),

            sleeping_senders: AtomicUsize::new(0),
            send_condvar:     Condvar::new(),

            receiver_disconnected: AtomicBool::new(false),
            num_senders: AtomicUsize::new(1),

            sleep_mutex: Mutex::new(()),
        };
        for i in 0..cap {
            packet.get_node(i).pos.store(i, SeqCst);
        }
        packet
    }

    /// Call this function when the sender is cloned.
    pub fn add_sender(&self) {
        self.num_senders.fetch_add(1, SeqCst);
    }

    /// Call this function when a sender is dropped.
    pub fn remove_sender(&self) {
        if self.num_senders.fetch_sub(1, SeqCst) == 1 {
            let _guard = self.sleep_mutex.lock().unwrap();
            if self.have_sleeping_receiver.load(SeqCst) {
                self.recv_condvar.notify_one();
            }
        }
    }

    /// Call this function when the consumer is dropped.
    pub fn remove_receiver(&self) {
        self.receiver_disconnected.store(true, SeqCst);
        let _guard = self.sleep_mutex.lock().unwrap();
        if self.sleeping_senders.load(SeqCst) > 0 {
            self.send_condvar.notify_all();
        }
    }

    fn get_node(&self, pos: usize) -> &mut Node<T> {
        unsafe { &mut *self.buf.offset((pos & self.cap_mask) as isize) }
    }

    /// Get a position to write to if the queue isn't full
    fn get_write_pos(&self) -> Option<usize> {
        let mut next_write = self.next_write.load(SeqCst);
        loop {
            let node = self.get_node(next_write);
            let diff = node.pos.load(SeqCst) as isize - next_write as isize;
            if diff < 0 {
                return None;
            } else if diff > 0 {
                next_write = self.next_write.load(SeqCst);
            } else {
                let next_write_old = next_write;
                next_write = self.next_write.compare_and_swap(next_write, next_write + 1,
                                                              SeqCst);
                if next_write_old == next_write {
                    return Some(next_write);
                }
            }
        }
    }

    pub fn send_async(&self, val: T, have_lock: bool) -> Result<(), (T, Error)> {
        if self.receiver_disconnected.load(SeqCst) {
            return Err((val, Error::Disconnected))
        }

        let write_pos = if let Some(w) = self.get_write_pos() {
            w
        } else {
            return if self.receiver_disconnected.load(SeqCst) {
                Err((val, Error::Disconnected))
            } else {
                Err((val, Error::Full))
            };
        };
        {
            let node = self.get_node(write_pos);
            unsafe { ptr::write(&mut node.val, val); }
            node.pos.store(write_pos + 1, SeqCst);
        }

        if self.have_sleeping_receiver.load(SeqCst) {
            if have_lock {
                self.recv_condvar.notify_one();
            } else {
                let _guard = self.sleep_mutex.lock().unwrap();
                self.recv_condvar.notify_one();
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
        let mut guard = self.sleep_mutex.lock().unwrap();
        self.sleeping_senders.fetch_add(1, SeqCst);
        loop {
            val = match self.send_async(val, true) {
                Err((v, Error::Full)) => v,
                e @ Err(_) => { rv = e; break; },
                Ok(_) => break,
            };
            guard = self.send_condvar.wait(guard).unwrap();
        }
        self.sleeping_senders.fetch_sub(1, SeqCst);

        rv
    }

    /// Get a position to read from if the queue isn't empty
    fn get_read_pos(&self) -> Option<usize> {
        let next_read = self.next_read.load(SeqCst);
        let node = self.get_node(next_read);
        let diff = node.pos.load(SeqCst) as isize - 1 - next_read as isize;
        if diff < 0 {
            None
        } else {
            assert!(diff == 0);
            self.next_read.store(next_read + 1, SeqCst);
            Some(next_read)
        }
    }

    pub fn recv_async(&self, have_lock: bool) -> Result<T, Error> {
        let read_pos = if let Some(r) = self.get_read_pos() {
            r
        } else {
            return if self.num_senders.load(SeqCst) == 0 {
                Err(Error::Disconnected)
            } else {
                Err(Error::Empty)
            };
        };
        let val;
        {
            let node = self.get_node(read_pos);
            val = unsafe { ptr::read(&node.val) };
            node.pos.store(read_pos + self.cap_mask + 1, SeqCst);
        }

        if self.sleeping_senders.load(SeqCst) > 0 {
            if have_lock {
                self.send_condvar.notify_one();
            } else {
                let _guard = self.sleep_mutex.lock().unwrap();
                self.send_condvar.notify_one();
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
        let mut guard = self.sleep_mutex.lock().unwrap();
        self.have_sleeping_receiver.store(true, SeqCst);
        loop {
            match self.recv_async(true) {
                Err(Error::Empty) => { },
                e @ Err(_) => { rv = e; break; },
                v @ Ok(_) => { rv = v; break; },
            }
            guard = self.recv_condvar.wait(guard).unwrap();
        }
        self.have_sleeping_receiver.store(false, SeqCst);

        rv
    }
}

impl<'a, T: Send+'a> Drop for Packet<'a, T> {
    fn drop(&mut self) {
        while self.recv_async(false).is_ok() { }
        
        unsafe {
            deallocate(self.buf as *mut u8,
                       (self.cap_mask as usize + 1) * mem::size_of::<Node<T>>(),
                       mem::min_align_of::<Node<T>>());
        }
    }
}
