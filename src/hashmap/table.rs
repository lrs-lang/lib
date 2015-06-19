// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem, ptr};
use core::ops::{Eq};
use base::clone::{Clone};
use hash::{Hash, Hasher};
use alloc::{Allocator};
use bucket::{Bucket, SetBucket, MutSetBucket};
use {Entry, VacantEntry, OccupiedEntry};

#[derive(Eq)]
enum SearchResult {
    Exists,
    Empty,
    Deleted,
}

pub struct Table<K, V, B, H, S, A>
    where A: Allocator,
          B: Bucket<K, V>,
          H: Hasher,
          S: Into<H::Seed>+Clone,
          K: Eq + Hash,
{
    table: *mut B,
    /// Invariant: Power of two.
    buckets: usize,
    elements: usize,
    deleted: usize,
    pool: A::Pool,
    seed: S,
    _marker: PhantomData<(K, V, H)>,
}

impl<K, V, B, H, S, A> Table<K, V, B, H, S, A>
    where A: Allocator,
          B: Bucket<K, V>,
          H: Hasher,
          S: Into<H::Seed>+Clone,
          K: Eq + Hash,
{
    pub fn new(mut pool: A::Pool, seed: S,
               capacity: usize) -> Result<Table<K, V, B, H, S, A>> {
        let buckets = capacity.checked_next_power_of_two().unwrap_or(!0);

        let table = unsafe {
            let table: *mut B = try!(A::allocate_array(&mut pool, buckets));
            for i in 0..buckets {
                (&mut *table.add(i)).set_empty();
            }
            table
        };

        let map = Table {
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
              K: Eq<Q>,
    {
        let hash = H::hash(key, self.seed.clone()).into() as usize;
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
              K: Eq<Q>,
    {
        let mut bucket = self.bucket_idx(hash);
        let mut del_pos = None;

        let mut i = 1;
        loop {
            let cur = *(key as *const _ as *const u32);
            // println!("{:x}, {:x}, {:x}", cur, bucket, *(self.get_bucket(bucket).key() as *const _ as *const u32));
            if self.is_empty(bucket) {
                // println!("iterations: {} buckets: {}, entries: {}",
                //          i, self.buckets, self.elements);
                return match del_pos {
                    Some(pos) => (SearchResult::Deleted, pos),
                    _         => (SearchResult::Empty, bucket),
                };
            } else if self.is_deleted(bucket) {
                abort!();
                if del_pos.is_none() {
                    del_pos = Some(bucket);
                }
            } else if may_exist && self.contains_key(bucket, key) {
                // println!("iterations: {} buckets: {}, entries: {}",
                //          i, self.buckets, self.elements);
                return (SearchResult::Exists, bucket);
            }
            // println!("conflict");
            bucket = self.bucket_idx(bucket + i);
            i += 1;
        }
    }

    pub fn debug(&self) {
        let trailing = self.buckets.trailing_zeros();
        let height = 1 << (trailing / 2);
        let width = 1 << (trailing - trailing / 2);
        println!("P1");
        println!("{} {}", width, height);
        for i in 0..self.buckets {
            unsafe {
                if self.get_bucket(i).is_empty() {
                    write!(::fd::STDOUT, "0 ");
                } else {
                    write!(::fd::STDOUT, "1 ");
                }
                if (i + 1) % width == 0 {
                    println!("");
                }
            }
        }
    }

    /// Finds an entry in the table and returns a reference to it.
    ///
    /// [argument, key]
    /// The key of the entry to find.
    pub fn find<Q>(&self, key: &Q) -> Option<&V>
        where Q: Hash,
              K: Eq<Q>,
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
    pub fn find_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where Q: Hash,
              K: Eq<Q>,
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

    pub fn set(&mut self, key: K, value: V) {
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

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
        where Q: Hash,
              K: Eq<Q>,
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

    pub fn entry<'a>(&'a mut self, key: &K) -> Result<Entry<'a, K, V, B>> {
        let hash = H::hash(key, self.seed.clone()).into() as usize;

        let (kind, bucket) = unsafe { self.search2(key, hash, true) };

        if kind == SearchResult::Exists {
            unsafe {
                let bucket = self.get_mut_bucket(bucket);
                return Ok(Entry::Occupied(OccupiedEntry::from_bucket(bucket)));
            }
        }

        let create_vacant = |table: &'a mut Table<K, V, B, H, S, A>, key, bucket, kind| {
            unsafe {
                let bucket = table.get_mut_bucket(bucket);
                ptr::memcpy(bucket.mut_key(), key, 1);
                let was_empty = kind == SearchResult::Empty;
                Ok(Entry::Vacant(VacantEntry::from_bucket(bucket, was_empty)))
            }
        };

        // The key is not in the table and can be inserted via the vacant entry. Therefore
        // we have to reserve space for at least one more element. If we don't have to
        // resize then we can use the bucket that has been returned by the previous
        // search. Otherwise we have to search for the new bucket at which to insert.
        if !try!(self.reserve(1)) {
            return create_vacant(self, key, bucket, kind);
        }

        let (kind, bucket) = unsafe { self.search2(key, hash, false) };
        create_vacant(self, key, bucket, kind)
    }

    /// Reserves space for new entries.
    ///
    /// [argument, n]
    /// The number of additional entries that can be placed in the table.
    ///
    /// [return_value]
    /// Returns whether the table was resized.
    pub fn reserve(&mut self, n: usize) -> Result<bool> {
        const GROW_THRESHOLD: usize = 2;

        if self.buckets / GROW_THRESHOLD - self.elements > n {
            // Don't have to resize.
            return Ok(false);
        }

        // let new_buckets = (self.elements - self.deleted)
        //                             .checked_add(n).unwrap_or(!0)
        //                             .checked_next_power_of_two().unwrap_or(!0);
        let new_buckets = self.buckets
                                    .checked_add(n).unwrap_or(!0)
                                    .checked_next_power_of_two().unwrap_or(!0);

        unsafe {
            let new_table = try!(A::allocate_array(&mut self.pool, new_buckets));
            self.copy_table(new_table, new_buckets);

            let old_table = mem::replace(&mut self.table, new_table);
            let old_buckets = mem::replace(&mut self.buckets, new_buckets);
            self.elements -= self.deleted;
            self.deleted = 0;
            A::free_array(&mut self.pool, old_table, old_buckets);
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
    unsafe fn copy_table(&self, new_table: *mut B, new_buckets: usize) {
        // Initialize the new table to all slots empty:
        for i in 0..new_buckets {
            (&mut *new_table.add(i)).set_empty();
        }

        let mut elements = self.elements - self.deleted;
        let mut bucketp = self.table;

        // There will be far fewer elements in the table than there are buckets. We only
        // copy until we've copied all elements.
        while elements > 0 {
            let bucket = &*bucketp;
            if bucket.is_set() {
                // Find an empty bucket in the new array.
                let mut new_pos = H::hash(bucket.key(), self.seed.clone()).into() as usize
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
              K: Eq<Q>,
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
    unsafe fn get_bucket(&self, n: usize) -> &B {
        debug_assert!(n < self.buckets);
        &*self.table.add(n)
    }

    /// Returns a mutable reference to a bucket.
    ///
    /// [argument, n]
    /// The index of the bucket to return.
    ///
    /// = Remarks
    ///
    /// This is unsafe because the validity of `n` is not checked.
    unsafe fn get_mut_bucket(&mut self, n: usize) -> &mut B {
        debug_assert!(n < self.buckets);
        &mut *self.table.add(n)
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
    unsafe fn get_set_bucket<'a>(&'a self, n: usize) -> SetBucket<'a, K, V, B> {
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
                                     n: usize) -> MutSetBucket<'a, K, V, B> {
        debug_assert!(n < self.buckets);
        debug_assert!(!self.is_empty(n));
        debug_assert!(!self.is_deleted(n));
        MutSetBucket::from_bucket(self.get_mut_bucket(n))
    }
}
