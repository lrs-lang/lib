// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_hashmap"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_alloc as alloc;
extern crate lrs_hash as hash;
extern crate lrs_fmt as fmt;
extern crate lrs_fd as fd;

#[prelude_import] use base::prelude::*;
use core::{mem, ptr};
use core::ops::{Eq};
use base::undef::{UndefState};
use base::clone::{Clone};
use base::default::{Default};
use alloc::{Heap};
use hash::{Hash};
use hash::xx_hash::{XxHash32};
use bucket::{Bucket};
use bucket::compact::{CompactBucket};
use bucket::loose::{LooseBucket};
use table::{Table};

pub mod lrs { pub use fmt::lrs::*; pub use fd; }

mod bucket;
mod table;

const DEFAULT_CAPACITY: usize = 8;

pub struct CompactHashMap<Key, Value, Hasher = XxHash32, Seed = (), Allocator = Heap>
    where Allocator: alloc::Allocator,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+Clone,
          Key: Eq + Hash + UndefState,
{
    inner: Table<Key, Value, CompactBucket<Key, Value>, Hasher, Seed, Allocator>,
}

impl<Key, Value, Hasher, Seed, Allocator>
        CompactHashMap<Key, Value, Hasher, Seed, Allocator>
    where Allocator: alloc::Allocator,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+Clone,
          Key: Eq + Hash + UndefState,
{
    pub fn new() -> Result<Self>
        where Allocator::Pool: Default,
              Seed: Default,
    {
        Ok(CompactHashMap {
            inner: try!(Table::new(Allocator::Pool::default(), Seed::default(),
                                   DEFAULT_CAPACITY)),
        })
    }

    pub fn set(&mut self, key: Key, value: Value) {
        self.inner.set(key, value);
    }

    /// Finds an entry in the table and returns a reference to it.
    ///
    /// [argument, key]
    /// The key of the entry to find.
    pub fn find<Q>(&self, key: &Q) -> Option<&Value>
        where Q: Hash,
              Key: Eq<Q>,
    {
        self.inner.find(key)
    }

    /// Finds an entry in the table and returns a reference to it.
    ///
    /// [argument, key]
    /// The key of the entry to find.
    pub fn find_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
        where Q: Hash,
              Key: Eq<Q>,
    {
        self.inner.find_mut(key)
    }

    pub fn entry<'a>(&'a mut self, key: &Key) -> Result<Entry<'a, Key, Value, CompactBucket<Key, Value>>> {
        self.inner.entry(key)
    }

    pub fn reserve(&mut self, n: usize) -> Result {
        self.inner.reserve(n).ignore_ok()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(Key, Value)>
        where Q: Hash,
              Key: Eq<Q>,
    {
        self.inner.remove(key)
    }

    pub fn debug(&self) {
        self.inner.debug();
    }
}

pub struct HashMap<Key, Value, Hasher = XxHash32, Seed = (), Allocator = Heap>
    where Allocator: alloc::Allocator,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+Clone,
          Key: Eq + Hash,
{
    inner: Table<Key, Value, LooseBucket<Key, Value>, Hasher, Seed, Allocator>,
}

impl<Key, Value, Hasher, Seed, Allocator>
        HashMap<Key, Value, Hasher, Seed, Allocator>
    where Allocator: alloc::Allocator,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+Clone,
          Key: Eq + Hash,
{
    pub fn new() -> Result<Self>
        where Allocator::Pool: Default,
              Seed: Default,
    {
        Ok(HashMap {
            inner: try!(Table::new(Allocator::Pool::default(), Seed::default(),
                                   DEFAULT_CAPACITY)),
        })
    }

    pub fn set(&mut self, key: Key, value: Value) {
        self.inner.set(key, value);
    }

    /// Finds an entry in the table and returns a reference to it.
    ///
    /// [argument, key]
    /// The key of the entry to find.
    pub fn find<Q>(&self, key: &Q) -> Option<&Value>
        where Q: Hash,
              Key: Eq<Q>,
    {
        self.inner.find(key)
    }

    /// Finds an entry in the table and returns a reference to it.
    ///
    /// [argument, key]
    /// The key of the entry to find.
    pub fn find_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
        where Q: Hash,
              Key: Eq<Q>,
    {
        self.inner.find_mut(key)
    }

    pub fn entry<'a>(&'a mut self, key: &Key) -> Result<Entry<'a, Key, Value, LooseBucket<Key, Value>>> {
        self.inner.entry(key)
    }

    pub fn reserve(&mut self, n: usize) -> Result {
        self.inner.reserve(n).ignore_ok()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(Key, Value)>
        where Q: Hash,
              Key: Eq<Q>,
    {
        self.inner.remove(key)
    }

    pub fn debug(&self) {
        self.inner.debug();
    }
}

pub enum Entry<'a, K, V, B>
    where B: Bucket<K, V> + 'a,
          K: Eq + Hash,
{
    Occupied(OccupiedEntry<'a, K, V, B>),
    Vacant(VacantEntry<'a, K, V, B>),
}

impl<'a, K, V, B> Entry<'a, K, V, B>
    where B: Bucket<K, V> + 'a,
          K: Eq + Hash,
{
    pub fn or_insert(self, key: K, value: V) -> OccupiedEntry<'a, K, V, B> {
        self.or_insert_with(|| (key, value))
    }

    pub fn or_insert_with<F>(self, f: F) -> OccupiedEntry<'a, K, V, B>
        where F: FnOnce() -> (K, V),
    {
        match self {
            Entry::Occupied(e) => e,
            Entry::Vacant(v) => {
                let (key, value) = f();
                v.set(key, value)
            },
        }
    }

}

pub struct VacantEntry<'a, K, V, B>
    where B: Bucket<K, V> + 'a,
          K: Eq + Hash,
{
    bucket: &'a mut B,
    was_empty: bool,
    _marker: PhantomData<(K, V)>,
}

impl<'a, K, V, B> VacantEntry<'a, K, V, B>
    where B: Bucket<K, V>,
          K: Eq + Hash,
{
    unsafe fn from_bucket(b: &'a mut B, was_empty: bool) -> VacantEntry<'a, K, V, B> {
        VacantEntry {
            bucket: b,
            was_empty: was_empty,
            _marker: PhantomData,
        }
    }

    pub fn set(self, key: K, value: V) -> OccupiedEntry<'a, K, V, B> {
        unsafe {
            assert!(&key == self.bucket.key());
            mem::unsafe_forget(key);
            ptr::write(self.bucket.mut_value(), value);
            let bucket = ptr::read(&self.bucket);
            mem::unsafe_forget(self);
            OccupiedEntry::from_bucket(bucket)
        }
    }
}

impl<'a, K, V, B> Drop for VacantEntry<'a, K, V, B>
    where B: Bucket<K, V>,
          K: Eq + Hash,
{
    fn drop(&mut self) {
        unsafe {
            if self.was_empty {
                self.bucket.set_empty();
            } else {
                self.bucket.set_deleted();
            }
        }
    }
}

pub struct OccupiedEntry<'a, K, V, B>
    where B: Bucket<K, V> + 'a,
          K: Eq + Hash,
{
    bucket: &'a mut B,
    _marker: PhantomData<(K, V)>,
}

impl<'a, K, V, B> OccupiedEntry<'a, K, V, B>
    where B: Bucket<K, V>,
          K: Eq + Hash,
{
    unsafe fn from_bucket(b: &'a mut B) -> OccupiedEntry<'a, K, V, B> {
        OccupiedEntry {
            bucket: b,
            _marker: PhantomData,
        }
    }

    pub fn into_mut(self) -> &'a mut V {
        unsafe { self.bucket.mut_value() }
    }

    pub fn remove(self) -> (VacantEntry<'a, K, V, B>, K, V) {
        unsafe {
            let key = ptr::read(self.bucket.key());
            let value = ptr::read(self.bucket.value());
            let bucket = ptr::read(&self.bucket);
            // Not actually necessary since there is no Drop implementation.
            mem::unsafe_forget(self);
            let entry = VacantEntry::from_bucket(bucket, false);
            (entry, key, value)
        }
    }
}

impl<'a, K, V, B> Deref for OccupiedEntry<'a, K, V, B>
    where B: Bucket<K, V>,
          K: Eq + Hash,
{
    type Target = V;
    fn deref(&self) -> &V {
        unsafe { self.bucket.value() }
    }
}

impl<'a, K, V, B> DerefMut for OccupiedEntry<'a, K, V, B>
    where B: Bucket<K, V>,
          K: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut V {
        unsafe { self.bucket.mut_value() }
    }
}
