//! A bounded MPSC channel.

use arc::{Arc, ArcTrait};
use select::{Selectable, _Selectable};
use {Error, Sendable};

mod imp;
#[cfg(test)] mod test;

/// Creates a new bounded MPSC channel with capacity at least `cap`.
///
/// # Safety
///
/// This is unsafe because under just the right circumstances this implementation can lead
/// to undefined behavior. Note that these circumstances are extremely rare and almost
/// impossible on 64 bit systems.
pub unsafe fn new<'a, T: Sendable+'a>(cap: usize) -> (Producer<'a, T>, Consumer<'a, T>) {
    let packet = Arc::new(imp::Packet::new(cap));
    packet.set_id(packet.unique_id());
    (Producer { data: packet.clone() }, Consumer { data: packet })
}

/// A producer of a bounded MPSC channel.
pub struct Producer<'a, T: Sendable+'a> {
    data: Arc<imp::Packet<'a, T>>,
}

impl<'a, T: Sendable+'a> Producer<'a, T> {
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

unsafe impl<'a, T: Sendable+'a> Send for Producer<'a, T> { }

#[unsafe_destructor]
impl<'a, T: Sendable+'a> Drop for Producer<'a, T> {
    fn drop(&mut self) {
        self.data.remove_sender();
    }
}

impl<'a, T: Sendable+'a> Clone for Producer<'a, T> {
    fn clone(&self) -> Producer<'a, T> {
        self.data.add_sender();
        Producer { data: self.data.clone(), }
    }
}

/// A consumer of a bounded SPMC channel.
pub struct Consumer<'a, T: Sendable+'a> {
    data: Arc<imp::Packet<'a, T>>,
}

impl<'a, T: Sendable+'a> Consumer<'a, T> {
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

unsafe impl<'a, T: Sendable+'a> Send for Consumer<'a, T> { }

#[unsafe_destructor]
impl<'a, T: Sendable+'a> Drop for Consumer<'a, T> {
    fn drop(&mut self) {
        self.data.remove_receiver();
    }
}

impl<'a, T: Sendable+'a> Selectable<'a> for Consumer<'a, T> {
    fn id(&self) -> usize {
        self.data.unique_id()
    }

    fn as_selectable(&self) -> ArcTrait<_Selectable<'a>+'a> {
        unsafe { self.data.as_trait(&*self.data as &(_Selectable+'a)) }
    }
}

