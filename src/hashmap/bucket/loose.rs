// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem, ptr};

use bucket::{Bucket};

const DELETED: u8 = 0;
const EMPTY: u8 = 1;
const SET: u8 = 2;

pub struct LooseBucket<K, V> {
    value: V,
    key: K,
    state: u8,
}

impl<K, V> Bucket<K, V> for LooseBucket<K, V> {
    fn is_empty(&self) -> bool {
        self.state == EMPTY
    }

    fn is_deleted(&self) -> bool {
        self.state == DELETED
    }

    fn is_set(&self) -> bool {
        self.state == SET
    }

    unsafe fn copy(&mut self, other: &LooseBucket<K, V>) {
        ptr::memcpy(&mut self.key, &other.key, 1);
        ptr::memcpy(&mut self.value, &other.value, 1);
        self.state = SET;
    }

    unsafe fn set_empty(&mut self) {
        self.state = EMPTY;
    }

    unsafe fn set_deleted(&mut self) {
        self.state = DELETED;
    }

    unsafe fn set(&mut self, key: K, value: V) {
        ptr::write(&mut self.key, key);
        ptr::write(&mut self.value, value);
        self.state = SET;
    }

    unsafe fn swap(&mut self, mut key: K, mut value: V) -> (K, V) {
        mem::swap(&mut self.key, &mut key);
        mem::swap(&mut self.value, &mut value);
        (key, value)
    }

    unsafe fn replace(&mut self, key: K, value: V) {
        ptr::drop(&mut self.key);
        ptr::drop(&mut self.value);
        ptr::write(&mut self.key, key);
        ptr::write(&mut self.value, value);
    }

    unsafe fn remove(&mut self) -> (K, V) {
        let key = ptr::read(&self.key);
        let value = ptr::read(&self.value);
        self.set_deleted();
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
