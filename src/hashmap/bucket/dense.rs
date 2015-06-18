// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem, ptr};
use base::unused::{UnusedState};

use bucket::{Bucket};

const DELETED: usize = 0;
const EMPTY: usize = 1;

pub struct DenseBucket<K, V>
    where K: UnusedState,
{
    key: K,
    value: V,
}

impl<K, V> Bucket<K, V> for DenseBucket<K, V>
    where K: UnusedState,
{
    fn is_empty(&self) -> bool {
        mem::as_bytes(&self.key) == mem::as_bytes(&K::unused_state(EMPTY))
    }

    fn is_deleted(&self) -> bool {
        mem::as_bytes(&self.key) == mem::as_bytes(&K::unused_state(DELETED))
    }

    fn is_set(&self) -> bool {
        !self.is_empty() && !self.is_deleted()
    }

    unsafe fn copy(&mut self, other: &DenseBucket<K, V>) {
        ptr::memcpy(&mut self.key, &other.key, 1);
        ptr::memcpy(&mut self.value, &other.value, 1);
    }

    unsafe fn set_empty(&mut self) {
        ptr::write(&mut self.key as *mut _ as *mut K::Plain, K::unused_state(EMPTY));
    }

    unsafe fn set_deleted(&mut self) {
        ptr::write(&mut self.key as *mut _ as *mut K::Plain, K::unused_state(DELETED));
    }

    unsafe fn set(&mut self, key: K, value: V) {
        ptr::write(&mut self.key, key);
        ptr::write(&mut self.value, value);
    }

    unsafe fn replace(&mut self, mut key: K, mut value: V) -> (K, V) {
        mem::swap(&mut self.key, &mut key);
        mem::swap(&mut self.value, &mut value);
        (key, value)
    }

    unsafe fn remove(&mut self) -> (K, V) {
        let key = ptr::read(&self.key);
        let value = ptr::read(&self.value);
        ptr::write(&mut self.key as *mut _ as *mut K::Plain, K::unused_state(DELETED));
        (key, value)
    }

    unsafe fn key(&self) -> &K {
        &self.key
    }

    unsafe fn mut_key(&mut self) -> &mut K {
        &mut self.key
    }

    unsafe fn value(&self) -> &V {
        &self.value
    }

    unsafe fn mut_value(&mut self) -> &mut V {
        &mut self.value
    }
}
