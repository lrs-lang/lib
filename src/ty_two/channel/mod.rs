// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A bounded MPSC channel.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::clone::{Clone};
use error::{self};
use arc::{Arc};

#[derive(Copy, Eq)]
pub enum Error {
    Disconnected,
    Full,
    Empty,
    Deadlock,
}

mod imp;

/// Creates a new bounded MPSC channel with capacity at least `cap`.
pub fn new<T: Send>(cap: usize) -> Result<(Producer<T>, Consumer<T>)> {
    let packet = try!(imp::Packet::new(cap));
    let mut packet = match Arc::new(packet) {
        Ok(p) => p,
        Err(_) => return Err(error::NoMemory),
    };
    imp::set_lock(packet.as_mut().unwrap());
    Ok((Producer { data: packet.clone() }, Consumer { data: packet }))
}

/// A producer of a bounded MPSC channel.
pub struct Producer<T: Send> {
    data: Arc<imp::Packet<T>>,
}

impl<T: Send> Producer<T> {
    /// Sends a message over the channel. Blocks if the channel is full.
    ///
    /// ### Error
    ///
    /// - `Disconnected` - The consumer has disconnected.
    pub fn send_sync(&self, val: T) -> Result<(), (T, Error)> {
        self.data.send_sync(val)
    }

    /// Sends a message over the channel. Does not block if the channel is full.
    ///
    /// ### Error
    ///
    /// - `Disconnected` - The consumer has disconnected.
    /// - `Full` - The buffer is full.
    pub fn send_async(&self, val: T) -> Result<(), (T, Error)> {
        self.data.send_async(val, false)
    }
}

impl<T: Send> Drop for Producer<T> {
    fn drop(&mut self) {
        self.data.remove_sender();
    }
}

impl<T: Send> Clone for Producer<T> {
    fn clone(&self) -> Producer<T> {
        self.data.add_sender();
        Producer { data: self.data.clone(), }
    }
}

/// A consumer of a bounded SPMC channel.
pub struct Consumer<T: Send> {
    data: Arc<imp::Packet<T>>,
}

impl<T: Send> Consumer<T> {
    /// Receives a message from the channel. Blocks if the channel is empty.
    ///
    /// ### Error
    ///
    /// - `Disconnected` - All producers have disconnected and the channel is empty.
    pub fn recv_sync(&self) -> Result<T, Error> {
        self.data.recv_sync()
    }

    /// Receives a message over the channel. Does not block if the channel is empty.
    ///
    /// ### Error
    ///
    /// - `Disconnected` - All producers have disconnected and the channel is empty.
    /// - `Empty` - The buffer is empty.
    pub fn recv_async(&self) -> Result<T, Error> {
        self.data.recv_async(false)
    }
}

impl<T: Send> Drop for Consumer<T> {
    fn drop(&mut self) {
        self.data.remove_receiver();
    }
}
