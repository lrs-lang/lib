// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem, ptr, slice};
use core::ptr::{NoAliasMemPtr};
use core::ops::{Eq};
use core::iter::{IntoIterator};
use hash::{self, Hash};
use alloc::{self};
use bucket::{self, SetBucket, MutSetBucket};
use fmt::{Debug, Write};

#[derive(Eq)]
enum SearchResult {
    Exists,
    Empty,
    Deleted,
}

const DEFAULT_CAPACITY: usize = 8;

/// A generic hash map that uses open addressing and quadratic probing.
pub struct GenericMap<Key, Value, Bucket, Hasher = hash::xx_hash::XxHash32, Seed = (),
                      Allocator: ?Sized = alloc::Heap>
    where Allocator: alloc::MemPool,
          Bucket: bucket::Bucket<Key, Value>,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+To,
          Key: Eq + Hash,
{
    table: NoAliasMemPtr<Bucket>,
    /// Invariant: Power of two.
    buckets: usize,
    elements: usize,
    deleted: usize,
    seed: Seed,
    _marker: PhantomData<(Key, Value, Hasher)>,
    pool: Allocator,
}

impl<K, V, B, H, S, A1: ?Sized, A2>
    TryTo<GenericMap<K, V, B, H, S, A2>> for GenericMap<K, V, B, H, S, A1>
    where A1: alloc::MemPool,
          A2: alloc::MemPool+OutOf,
          B: bucket::Bucket<K, V>,
          H: hash::Hasher,
          S: Into<H::Seed>+To,
          K: Eq + Hash + TryTo,
          V: TryTo,
{
    fn try_to(&self) -> Result<GenericMap<K, V, B, H, S, A2>> {
        let mut new = try!(GenericMap::details(self.size(), self.seed.to(),
                                               A2::out_of(())));
        for (key, val) in self {
            new.set(try!(key.try_to()), try!(val.try_to()));
        }
        Ok(new)
    }
}

impl<Key, Value, Bucket, Hasher, Allocator>
    GenericMap<Key, Value, Bucket, Hasher, (), Allocator>
    where Allocator: alloc::MemPool + OutOf,
          Bucket: bucket::Bucket<Key, Value>,
          Hasher: hash::Hasher,
          (): Into<Hasher::Seed>,
          Key: Eq + Hash,
{
    /// Creates a new map with the default parameters.
    pub fn new() -> Result<Self> {
        Self::details(DEFAULT_CAPACITY, (), Allocator::out_of(()))
    }

    pub fn with_capacity(cap: usize) -> Result<Self> {
        Self::details(cap, (), Allocator::out_of(()))
    }
}

impl<Key, Value, Bucket, Hasher, Seed, Allocator>
    GenericMap<Key, Value, Bucket, Hasher, Seed, Allocator>
    where Allocator: alloc::MemPool,
          Bucket: bucket::Bucket<Key, Value>,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+To,
          Key: Eq + Hash,
{
    /// Creates a new map.
    ///
    /// [argument, capacity]
    /// The number of elements that can be stored in the map before it has to be resized.
    ///
    /// [argument, seed]
    /// The seed to use for the hash operations.
    ///
    /// [argument, pool]
    /// The memory pool which will be used for allocations.
    pub fn details(capacity: usize, seed: Seed,
                   mut pool: Allocator) -> Result<Self> {
        let buckets = capacity
                           .checked_next_power_of_two().unwrap_or(!0)
                           .checked_mul(2).unwrap_or(!0);

        let table = unsafe {
            let table: *mut Bucket = try!(alloc::alloc_array(&mut pool, buckets)).0;
            for i in 0..buckets {
                (&mut *table.add(i)).set_empty();
            }
            NoAliasMemPtr::new(table)
        };

        let map = GenericMap {
            table: table,
            buckets: buckets,
            elements: 0,
            deleted: 0,
            pool: pool,
            seed: seed,
            _marker: PhantomData,
        };
        Ok(map)
    }
}

impl<Key, Value, Bucket, Hasher, Seed, Allocator: ?Sized>
    GenericMap<Key, Value, Bucket, Hasher, Seed, Allocator>
    where Allocator: alloc::MemPool,
          Bucket: bucket::Bucket<Key, Value>,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+To,
          Key: Eq + Hash,
{
    /// Searches for a key in the table.
    ///
    /// [argument, key]
    /// The key to search for.
    ///
    /// [return_value]
    /// Returns the position where the key was found or the position where it should be
    /// inserted.
    ///
    /// = Remarks
    ///
    /// This is unsafe because it assumes that either the key is in the table or the is an
    /// empty bucket.
    ///
    /// We use triangular probing, a variant of quadratic probing, to find the key.
    ///
    /// Due to a property of triangular numbers and powers of two, we never test the same
    /// bucket twice. More precisely, we check the buckets in the following order:
    ///
    /// ----
    ///     (hash + 1 + 2 + ... + n) % num_buckets
    /// ----
    ///
    /// where `0 <= n < num_buckets`. The difference of two such indices is
    ///
    /// ----
    ///    (n + 1) + ... + m = (m + n + 1)*(m + n)/2
    /// ----
    ///
    /// where `0 <= n < m < num_buckets. One of the two factors is odd. The other one is
    /// strictly bounded by `2*num_buckes`. Due to the denominator, the whole thing has a
    /// non-zero remainder when divided by `num_buckets`.
    unsafe fn search<Q>(&self, key: &Q) -> (SearchResult, usize)
        where Q: Hash,
              Key: Eq<Q>,
    {
        let hash = Hasher::hash(key, self.seed.to()).into() as usize;
        self.search2(key, hash, true)
    }

    /// Searches for a key in the table.
    ///
    /// [argument, key]
    /// The key to search for.
    ///
    /// [argument, hash]
    /// The hash of the key. This must be the hash of the key or the behavior is
    /// undefined.
    ///
    /// [return_value]
    /// Returns the position where the key was found or the position where it should be
    /// inserted.
    ///
    /// = Remarks
    ///
    /// See the documentation of search above.
    unsafe fn search2<Q>(&self, key: &Q, hash: usize,
                         may_exist: bool) -> (SearchResult, usize)
        where Q: Hash,
              Key: Eq<Q>,
    {
        let mut bucket = self.bucket_idx(hash);
        let mut del_pos = None;

        let mut i = 1;
        loop {
            if self.is_empty(bucket) {
                return match del_pos {
                    Some(pos) => (SearchResult::Deleted, pos),
                    _         => (SearchResult::Empty, bucket),
                };
            } else if self.is_deleted(bucket) {
                if del_pos.is_none() {
                    del_pos = Some(bucket);
                }
            } else if may_exist && self.contains_key(bucket, key) {
                return (SearchResult::Exists, bucket);
            }
            bucket = self.bucket_idx(bucket + i);
            i += 1;
        }
    }

    pub fn size(&self) -> usize {
        self.elements - self.deleted
    }

    /// Finds an entry in the table and returns a reference to it.
    ///
    /// [argument, key]
    /// The key of the entry to find.
    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
        where Q: Hash,
              Key: Eq<Q>,
    {
        unsafe {
            match self.search(key) {
                (SearchResult::Exists, bucket) => {
                    Some(self.get_set_bucket(bucket).value())
                },
                _ => None,
            }
        }
    }

    /// Finds an entry in the table and returns a mutable reference to it.
    ///
    /// [argument, key]
    /// The key of the entry to find.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
        where Q: Hash,
              Key: Eq<Q>,
    {
        unsafe {
            match self.search(key) {
                (SearchResult::Exists, bucket) => {
                    Some(self.get_mut_set_bucket(bucket).mut_value())
                },
                _ => None,
            }
        }
    }

    /// Sets a value in the map.
    ///
    /// [argument, key]
    /// The key to set.
    ///
    /// [argument, value]
    /// The value to store.
    ///
    /// = Remarks
    ///
    /// If the key is already in the table, the passed key will be dropped. If the key is
    /// not in the table and reserving space for another entry fails, the process will be
    /// aborted.
    ///
    /// :entry: link:lrs::hashmap::GenericHashMap::entry[entry]
    ///
    /// See {entry} for a more robust API.
    ///
    /// = See also
    ///
    /// * {entry}
    pub fn set(&mut self, key: Key, value: Value) {
        self.reserve(1).unwrap();
        unsafe {
            let (kind, bucket) = self.search(&key);
            match kind {
                SearchResult::Empty => self.elements += 1,
                SearchResult::Deleted => self.deleted -= 1,
                _ => { },
            }
            let bucket = self.get_mut_bucket(bucket);
            match kind {
                SearchResult::Exists => *bucket.mut_value() = value,
                _ => bucket.set(key, value),
            }
        }
    }

    /// Removes an element from the map.
    ///
    /// [argument, key]
    /// The key to remove.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<(Key, Value)>
        where Q: Hash,
              Key: Eq<Q>,
    {
        unsafe {
            match self.search(key) {
                (SearchResult::Exists, bucket) => {
                    self.deleted += 1;
                    Some(self.get_mut_bucket(bucket).remove())
                },
                _ => None,
            }
        }
    }

    /// Returns a mutable reference to a bucket.
    ///
    /// [argument, key]
    /// The key that identifies the bucket.
    ///
    /// = Remarks
    ///
    /// The returned bucket can be set or empty. If it's empty then the API allows a key
    /// and element to be inserted into it. In this cas the key should hash to the same
    /// value the `key` argument hashes to, or it might not be possible to retrieve the
    /// inserted value until the table is resized.
    ///
    /// :get: link:lrs::hashmap::GenericHashMap::get[get]
    /// :get_mut: link:lrs::hashmap::GenericHashMap::get_mut[get_mut]
    ///
    /// If you don't plan to insert an element, you should use the {get} or {get_mut}
    /// functions instead.
    pub fn entry<'a, Q>(&'a mut self, key: &Q) -> Result<Entry<'a, Key, Value, Bucket>>
        where Q: Hash,
              Key: Eq<Q>,
    {
        let hash = Hasher::hash(key, self.seed.to()).into() as usize;

        let (kind, bucket) = unsafe { self.search2(key, hash, true) };

        if kind == SearchResult::Exists {
            unsafe {
                let (bucket, deleted) = self.get_mut_bucket_and_deleted(bucket);
                return Ok(Entry::Occupied(OccupiedEntry::from_bucket(bucket, deleted)));
            }
        }

        let create_vacant = |table: &'a mut Self, bucket, kind| {
            unsafe {
                if kind == SearchResult::Empty {
                    table.elements += 1;
                    table.deleted += 1;
                }
                let (bucket, deleted) = table.get_mut_bucket_and_deleted(bucket);
                Ok(Entry::Vacant(VacantEntry::from_bucket(bucket, deleted)))
            }
        };

        // The key is not in the table and can be inserted via the vacant entry. Therefore
        // we have to reserve space for at least one more element. If we don't have to
        // resize then we can use the bucket that has been returned by the previous
        // search. Otherwise we have to search for the new bucket at which to insert.
        if !try!(self.reserve(1)) {
            return create_vacant(self, bucket, kind);
        }

        let (kind, bucket) = unsafe { self.search2(key, hash, false) };
        create_vacant(self, bucket, kind)
    }

    /// Reserves space for new entries.
    ///
    /// [argument, n]
    /// The number of additional entries that can be placed in the table.
    ///
    /// [return_value]
    /// Returns whether the table was resized.
    pub fn reserve(&mut self, n: usize) -> Result<bool> {
        if self.buckets / 2 - self.elements > n {
            // Don't have to resize.
            Ok(false)
        } else {
            self.resize(n)
        }
    }

    pub fn shrink_to_fit(&mut self) -> Result<bool> {
        if self.buckets / 2 > self.elements - self.deleted + 1 {
            self.resize(0)
        } else {
            Ok(false)
        }
    }

    fn resize(&mut self, n: usize) -> Result<bool> {
        let new_buckets = (self.elements - self.deleted + 1)
                                    .checked_add(n).unwrap_or(!0)
                                    .checked_next_power_of_two().unwrap_or(!0)
                                    .checked_mul(2).unwrap_or(!0);

        unsafe {
            let new_table = try!(alloc::alloc_array(&mut self.pool, new_buckets)).0;
            self.copy_table(new_table, new_buckets);

            let old_table = mem::replace(&mut self.table, NoAliasMemPtr::new(new_table));
            let old_buckets = mem::replace(&mut self.buckets, new_buckets);
            self.elements -= self.deleted;
            self.deleted = 0;
            alloc::free_array(&mut self.pool, old_table.get(), old_buckets);
        }

        Ok(true)
    }

    /// Copies all elements from this table into a new array of buckets.
    ///
    /// [argument, new_table]
    /// A pointer to an uninitialized array of buckets.
    ///
    /// [argument, new_buckets]
    /// {
    /// The length of the array pointed to by `new_table`.
    ///
    /// This must be a power of two and at least as large as the number of set buckets in
    /// the current table. Otherwise the behavior is undefined.
    ///
    /// }
    ///
    /// = Remarks
    ///
    /// This does not modify the current table. Afterwards the new table contains
    /// byte-wise copies of the set elements in this table. All unused slots in the new
    /// table are empty.
    unsafe fn copy_table(&self, new_table: *mut Bucket, new_buckets: usize) {
        // Initialize the new table to all slots empty:
        for i in 0..new_buckets {
            (&mut *new_table.add(i)).set_empty();
        }

        let mut elements = self.elements - self.deleted;
        let mut bucketp = self.table.get();

        // There will be far fewer elements in the table than there are buckets. We only
        // copy until we've copied all elements.
        while elements > 0 {
            let bucket = &*bucketp;
            if bucket.is_set() {
                // Find an empty bucket in the new array.
                let mut new_pos = Hasher::hash(bucket.key(),
                                               self.seed.to()).into() as usize
                                                            & (new_buckets - 1);
                let mut i = 1;
                loop {
                    let mut new_bucket = &mut *new_table.add(new_pos);
                    if new_bucket.is_empty() {
                        new_bucket.copy(bucket);
                        break;
                    }
                    new_pos = (new_pos + i) & (new_buckets - 1);
                    i += 1;
                }
                // println!("{}", i);
                elements -= 1;
            }
            bucketp = bucketp.add(1);
        }
    }

    /// Turns an arbitrary index into a valid bucket index.
    ///
    /// [argument, pos]
    /// The index to reduce.
    ///
    /// = Remarks
    ///
    /// Since we always have a power-of-two number of buckets, this is simply masking.
    fn bucket_idx(&self, pos: usize) -> usize {
        // Checks that we actually have a valid number of buckets.
        debug_assert!(self.buckets & (self.buckets - 1) == 0);
        pos & (self.buckets - 1)
    }

    /// Checks whether a bucket is empty.
    ///
    /// [argument, n]
    /// {
    /// The bucket to check.
    ///
    /// Must be `< self.buckets`.
    ///
    /// }
    ///
    /// = Remarks
    ///
    /// This is unsafe because the validity of `n` is not checked.
    unsafe fn is_empty(&self, n: usize) -> bool {
        self.get_bucket(n).is_empty()
    }

    /// Checks whether a bucket is deleted.
    ///
    /// [argument, n]
    /// {
    /// The bucket to check.
    ///
    /// Must be `< self.buckets`.
    ///
    /// }
    ///
    /// = Remarks
    ///
    /// This is unsafe because the validity of `n` is not checked.
    unsafe fn is_deleted(&self, n: usize) -> bool {
        self.deleted > 0 && self.get_bucket(n).is_deleted()
    }

    /// Checks whether a bucket contains a key.
    ///
    /// [argument, n]
    /// The index of the bucket.
    ///
    /// [argument, key]
    /// The key.
    ///
    /// = Remarks
    ///
    /// `n` must be a valid index of a bucket which contains a valid key or the behavior
    /// is undefined.
    unsafe fn contains_key<Q>(&self, n: usize, key: &Q) -> bool
        where Q: Hash,
              Key: Eq<Q>,
    {
        let bucket = self.get_bucket(n);
        debug_assert!(bucket.is_set());
        bucket.key() == key
    }

    /// Returns a reference to a bucket.
    ///
    /// [argument, n]
    /// The index of the bucket to return.
    ///
    /// = Remarks
    ///
    /// This is unsafe because the validity of `n` is not checked.
    unsafe fn get_bucket(&self, n: usize) -> &Bucket {
        debug_assert!(n < self.buckets);
        &*self.table.get().add(n)
    }

    /// Returns a mutable reference to a bucket.
    ///
    /// [argument, n]
    /// The index of the bucket to return.
    ///
    /// = Remarks
    ///
    /// This is unsafe because the validity of `n` is not checked.
    unsafe fn get_mut_bucket(&mut self, n: usize) -> &mut Bucket {
        debug_assert!(n < self.buckets);
        &mut *self.table.get().add(n)
    }

    /// Returns mutable references to a bucket and the deleted counter.
    ///
    /// [argument, n]
    /// The index of the bucket to return.
    ///
    /// = Remarks
    ///
    /// This is unsafe because the validity of `n` is not checked.
    unsafe fn get_mut_bucket_and_deleted(&mut self,
                                         n: usize) -> (&mut Bucket, &mut usize) {
        debug_assert!(n < self.buckets);
        (&mut *self.table.get().add(n), &mut self.deleted)
    }

    /// Returns a reference to an non-empty / non-deleted bucket.
    ///
    /// [argument, n]
    /// The index of the bucket to return.
    ///
    /// = Remarks
    ///
    /// This is unsafe because the validity of `n` and the state of the bucket are not
    /// checked.
    unsafe fn get_set_bucket<'a>(&'a self,
                                 n: usize) -> SetBucket<'a, Key, Value, Bucket> {
        debug_assert!(n < self.buckets);
        debug_assert!(!self.is_empty(n));
        debug_assert!(!self.is_deleted(n));
        SetBucket::from_bucket(self.get_bucket(n))
    }

    /// Returns a mutable reference to an non-empty / non-deleted bucket.
    ///
    /// [argument, n]
    /// The index of the bucket to return.
    ///
    /// = Remarks
    ///
    /// This is unsafe because the validity of `n` and the state of the bucket are not
    /// checked.
    unsafe fn get_mut_set_bucket<'a>(&'a mut self,
                                     n: usize) -> MutSetBucket<'a, Key, Value, Bucket> {
        debug_assert!(n < self.buckets);
        debug_assert!(!self.is_empty(n));
        debug_assert!(!self.is_deleted(n));
        MutSetBucket::from_bucket(self.get_mut_bucket(n))
    }
}

impl<Key, Value, Bucket, Hasher, Seed, Allocator: ?Sized>
    Drop for GenericMap<Key, Value, Bucket, Hasher, Seed, Allocator>
    where Allocator: alloc::MemPool,
          Bucket: bucket::Bucket<Key, Value>,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+To,
          Key: Eq + Hash,
{
    fn drop(&mut self) {
        unsafe {
            let mut elements = self.elements - self.deleted;
            let mut bucket = self.table.get();

            while elements > 0 {
                if (&*bucket).is_set() {
                    ptr::drop(bucket);
                    elements -= 1;
                }
                bucket = bucket.add(1);
            }

            alloc::free_array(&mut self.pool, self.table.get(), self.buckets);
        }
    }
}

impl<'a, Key: 'a, Value: 'a, Bucket, Hasher, Seed, Allocator: ?Sized>
    IntoIterator for &'a GenericMap<Key, Value, Bucket, Hasher, Seed, Allocator>
    where Allocator: alloc::MemPool,
          Bucket: bucket::Bucket<Key, Value>,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+To,
          Key: Eq + Hash,
{
    type Item = (&'a Key, &'a Value);
    type IntoIter = MapIter<'a, Key, Value, Bucket>;
    fn into_iter(self) -> MapIter<'a, Key, Value, Bucket> {
        MapIter {
            table: unsafe { slice::from_ptr(self.table.get(), self.buckets) },
            _marker: PhantomData,
        }
    }
}

pub struct MapIter<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value> + 'a,
{
    table: &'a [Bucket],
    _marker: PhantomData<(Key, Value)>,
}

impl<'a, Key: 'a, Value: 'a, Bucket> Iterator for MapIter<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value>,
{
    type Item = (&'a Key, &'a Value);
    fn next(&mut self) -> Option<(&'a Key, &'a Value)> {
        unsafe {
            while self.table.len() > 0 && !self.table[0].is_set() {
                self.table = &self.table[1..];
            }
            if self.table.len() == 0 {
                None
            } else {
                let e = &self.table[0];
                self.table = &self.table[1..];
                Some((e.key(), e.value()))
            }
        }
    }
}

impl<Key, Value, Bucket, Hasher, Seed, Allocator: ?Sized>
    Debug for GenericMap<Key, Value, Bucket, Hasher, Seed, Allocator>
    where Allocator: alloc::MemPool,
          Bucket: bucket::Bucket<Key, Value>,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+To,
          Key: Eq + Hash + Debug,
          Value: Debug,
{
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        try!(write!(w, "{{ "));
        for (key, value) in self {
            try!(write!(w, "{:?}: {:?}, ", key, value));
        }
        write!(w, "}}")
    }
}

/// A bucket in a hash map.
pub enum Entry<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value> + 'a,
          Key: Eq + Hash,
{
    Occupied(OccupiedEntry<'a, Key, Value, Bucket>),
    Vacant(VacantEntry<'a, Key, Value, Bucket>),
}

impl<'a, Key, Value, Bucket> Entry<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value> + 'a,
          Key: Eq + Hash,
{
    /// Returns the occupied entry or inserts a new key.
    ///
    /// [argument, key]
    /// {
    /// The key that will be inserted.
    ///
    /// This key should hash to the same value as the key that was used to retrieve this
    /// entry. Otherwise it might not be possible to retrieve the stored value until the
    /// table is resized.
    ///
    /// }
    ///
    /// [argument, value]
    /// The value that will be inserted.
    pub fn or_insert(self, key: Key,
                     value: Value) -> OccupiedEntry<'a, Key, Value, Bucket> {
        self.or_insert_with(|| (key, value))
    }

    /// Returns the occupied entry or inserts the result of a function.
    ///
    /// [argument, f]
    /// The function that will be called if the bucket is empty.
    pub fn or_insert_with<F>(self, f: F) -> OccupiedEntry<'a, Key, Value, Bucket>
        where F: FnOnce() -> (Key, Value),
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

/// An empty bucket in a hash map.
pub struct VacantEntry<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value> + 'a,
          Key: Eq + Hash,
{
    bucket: &'a mut Bucket,
    deleted: &'a mut usize,
    _marker: PhantomData<(Key, Value)>,
}

impl<'a, Key, Value, Bucket> VacantEntry<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value>,
          Key: Eq + Hash,
{
    unsafe fn from_bucket(b: &'a mut Bucket,
                          deleted: &'a mut usize) -> VacantEntry<'a, Key, Value, Bucket> {
        VacantEntry {
            bucket: b,
            deleted: deleted,
            _marker: PhantomData,
        }
    }

    /// Sets the content of the empty bucket.
    ///
    /// [argument, key]
    /// {
    /// The key that will be inserted.
    ///
    /// This key should hash to the same value as the key that was used to retrieve this
    /// entry. Otherwise it might not be possible to retrieve the stored value until the
    /// table is resized.
    ///
    /// }
    ///
    /// [argument, value]
    /// The value that will be inserted.
    pub fn set(self, key: Key, value: Value) -> OccupiedEntry<'a, Key, Value, Bucket> {
        unsafe {
            *self.deleted -= 1;
            self.bucket.set(key, value);
            OccupiedEntry::from_bucket(self.bucket, self.deleted)
        }
    }
}

/// A non-empty bucket.
pub struct OccupiedEntry<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value> + 'a,
          Key: Eq + Hash,
{
    bucket: &'a mut Bucket,
    deleted: &'a mut usize,
    _marker: PhantomData<(Key, Value)>,
}

impl<'a, Key, Value, Bucket> OccupiedEntry<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value>,
          Key: Eq + Hash,
{
    unsafe fn from_bucket(
        b: &'a mut Bucket,
        deleted: &'a mut usize) -> OccupiedEntry<'a, Key, Value, Bucket>
    {
        OccupiedEntry {
            bucket: b,
            deleted: deleted,
            _marker: PhantomData,
        }
    }

    /// Returns the contained mutable reference to the stored value.
    pub fn into_mut(self) -> &'a mut Value {
        unsafe { self.bucket.mut_value() }
    }

    /// Removes the contained value and key.
    pub fn remove(self) -> (VacantEntry<'a, Key, Value, Bucket>, Key, Value) {
        unsafe {
            *self.deleted += 1;
            let (key, value) = self.bucket.remove();
            let entry = VacantEntry::from_bucket(self.bucket, self.deleted);
            (entry, key, value)
        }
    }
}

impl<'a, Key, Value, Bucket> Deref for OccupiedEntry<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value>,
          Key: Eq + Hash,
{
    type Target = Value;
    fn deref(&self) -> &Value {
        unsafe { self.bucket.value() }
    }
}

impl<'a, Key, Value, Bucket> DerefMut for OccupiedEntry<'a, Key, Value, Bucket>
    where Bucket: bucket::Bucket<Key, Value>,
          Key: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Value {
        unsafe { self.bucket.mut_value() }
    }
}
