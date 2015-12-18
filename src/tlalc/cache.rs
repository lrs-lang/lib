// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {thread, arch_fns};

use {sys};
use chunk::{Chunk, Slot};
use p::{P};
use {
    CACHE_SIZE, LARGE_CLASS_SHIFT, CHUNK_SIZE, MAX_SMALL, CHUNK_MASK, BLOCK_SIZE,
    MIN_ALLOC,
};

pub struct Cache {
    cache_size: [usize; 20],
    cache: [Option<P<Slot>>; 20],
    chunk: P<Chunk>,
    free_chunk: Option<P<Chunk>>,
    init: bool,
}

impl Cache {
    pub const fn new() -> Cache {
        Cache {
            cache_size: [0; 20],
            cache: [None; 20],
            chunk: unsafe { P::zero() },
            free_chunk: None,
            init: false,
        }
    }

    /// Allocates memory.
    ///
    /// [argument, size]
    /// The size of the memory to allocate. Bounded above by `!0 >> 1`.
    ///
    /// [return_value]
    /// Returns the allocated memory or an error.
    pub unsafe fn alloc(&mut self, size: usize) -> Result<*mut u8> {
        if unlikely!(size > MAX_SMALL) {
            return self.alloc_large(size);
        }

        let class = (size - 1) / MIN_ALLOC;
        let size = (size + MIN_ALLOC - 1) & !(MIN_ALLOC - 1);
        self.alloc_bin(size, class)
    }

    pub unsafe fn free(&mut self, ptr: *mut u8, size: usize) {
        if unlikely!(size > MAX_SMALL) {
            return self.free_large(ptr, size);
        }

        let class = (size - 1) / MIN_ALLOC;
        let size = (size + MIN_ALLOC - 1) & !(MIN_ALLOC - 1);
        self.free_bin(ptr, size, class)
    }

    pub unsafe fn realloc(&mut self, ptr: *mut u8, old_size: usize,
                          new_size: usize) -> Result<*mut u8> {
        unsafe fn size_to_class(size: usize) -> (usize, usize) {
            if likely!(size <= MAX_SMALL) {
                let class = (size - 1) / MIN_ALLOC;
                let size = (size + MIN_ALLOC - 1) & !(MIN_ALLOC - 1);
                (size, class)
            } else {
                let class = usize::bits() - (size - 1).leading_zeros() + LARGE_CLASS_SHIFT;
                let size = 1 << (class - LARGE_CLASS_SHIFT);
                (size, class)
            }
        }

        if likely!(old_size <= BLOCK_SIZE && new_size <= BLOCK_SIZE) {
            let (old_size, old_class) = size_to_class(old_size);
            let (new_size, new_class) = size_to_class(new_size);

            if likely!(old_class == new_class) {
                return Ok(ptr)
            }

            let slot = try!(self.alloc_bin(new_size, new_class));
            if unlikely!(new_size < old_size) {
                arch_fns::memcpy_aligned_16_16(slot, ptr, new_size);
            } else {
                arch_fns::memcpy_aligned_16_16(slot, ptr, old_size);
            }
            self.free_bin(ptr, old_size, old_class);
            return Ok(slot);
        }

        let old_size_block = align!(old_size, [%] BLOCK_SIZE); 
        let new_size_block = align!(new_size, [%] BLOCK_SIZE); 
        if likely!(old_size_block == new_size_block) {
            return Ok(ptr);
        }

        if old_size_block == BLOCK_SIZE {
            let (old_size, old_class) = size_to_class(old_size);
            let new = try!(sys::map(new_size_block));
            arch_fns::memcpy_aligned_16_16(new, ptr, old_size);
            self.free_bin(ptr, old_size, old_class);
            return Ok(new);
        }

        if new_size_block == BLOCK_SIZE {
            let (new_size, new_class) = size_to_class(new_size);
            let slot = try!(self.alloc_bin(new_size, new_class));
            arch_fns::memcpy_aligned_16_16(slot, ptr, new_size);
            sys::unmap(ptr, old_size_block);
            return Ok(slot);
        }

        let new = try!(sys::map(new_size_block));
        if new_size_block < old_size_block {
            arch_fns::memcpy_aligned_16_64(new, ptr, new_size_block);
        } else {
            arch_fns::memcpy_aligned_16_64(new, ptr, old_size_block);
        }
        sys::unmap(ptr, old_size_block);
        Ok(new)
    }
}

impl Cache {
    /// Allocates a large amount of memory.
    ///
    /// [argument, size]
    /// The size of the memory to allocate. Strictly bounded below by `MAX_SMALL`.
    ///
    /// [return_value]
    /// Returns the allocated memory or an error.
    unsafe fn alloc_large(&mut self, size: usize) -> Result<*mut u8> {
        if likely!(size > BLOCK_SIZE) {
            return sys::map((size + BLOCK_SIZE - 1) & (BLOCK_SIZE - 1));
        }

        let class = usize::bits() - (size - 1).leading_zeros() + LARGE_CLASS_SHIFT;
        let size = 1 << (class - LARGE_CLASS_SHIFT);
        self.alloc_bin(size, class)
    }

    /// Allocates memory from a bin.
    ///
    /// [argument, size]
    /// The upper bound of the size class specified by `bin`.
    ///
    /// [argument, bin]
    /// The bin from which to allocate.
    ///
    /// [return_value]
    /// Returns the allocated memory or an error.
    #[inline]
    unsafe fn alloc_bin(&mut self, size: usize, class: usize) -> Result<*mut u8> {
        let slot = self.cache[class];

        if unlikely!(slot.is_none()) {
            return self.alloc_bin_slow(size, class);
        }

        let slot = slot.unwrap();
        self.cache[class] = slot.next;
        self.cache_size[class] -= size;
        Ok(slot.ptr() as *mut u8)
    }

    /// Allocates memory from a bin.
    ///
    /// [argument, size]
    /// The upper bound of the size class specified by `bin`.
    ///
    /// [argument, bin]
    /// The bin from which to allocate.
    ///
    /// [return_value]
    /// Returns the allocated memory or an error.
    #[cold]
    unsafe fn alloc_bin_slow(&mut self, size: usize, class: usize) -> Result<*mut u8> {
        if unlikely!(!self.init) {
            try!(self.initialize());
        }

        let mut chunk = self.chunk;
        while self.cache_size[class] < CACHE_SIZE {
            if let Some((first, mut last, size)) = chunk.alloc(size, class) {
                self.cache_size[class] += size;
                last.next = self.cache[class];
                self.cache[class] = first.to_opt();
            } else {
                if let Some(c) = chunk.next {
                    chunk = c;
                } else {
                    chunk = match self.free_chunk.take() {
                        Some(c) => c,
                        _ => try!(Chunk::new()),
                    };
                    chunk.prev = None;
                    chunk.next = self.chunk.to_opt();
                    self.chunk.prev = chunk.to_opt();
                    self.chunk = chunk;
                    while self.cache_size[class] < CACHE_SIZE {
                        let (first, mut last, size) = chunk.alloc(size, class).unwrap();
                        self.cache_size[class] += size;
                        last.next = self.cache[class];
                        self.cache[class] = first.to_opt();
                    }
                }
            }
        }

        let slot = self.cache[class].unwrap();
        self.cache[class] = slot.next;
        self.cache_size[class] -= size;
        Ok(slot.ptr() as *mut _)
    }

    /// Initializes the cache.
    ///
    /// = Remarks
    ///
    /// `self` is located in thread-local-storage.
    unsafe fn initialize(&mut self) -> Result {
        self.chunk = try!(Chunk::new());

        let ptr = self as *mut Cache;
        let res = thread::at_exit(move || {
            let cache = &mut *ptr;
            let mut chunk = cache.chunk.to_opt();
            while let Some(c) = chunk {
                chunk = c.next;
                sys::unmap(c.ptr(), CHUNK_SIZE);
            }
            if let Some(c) = cache.free_chunk {
                sys::unmap(c.ptr(), CHUNK_SIZE);
            }
        });

        if res.is_err() {
            sys::unmap(self.chunk.ptr(), CHUNK_SIZE);
            return res;
        }

        self.init = true;
        Ok(())
    }

    unsafe fn free_large(&mut self, ptr: *mut u8, size: usize) {
        if unlikely!(size > BLOCK_SIZE) {
            sys::unmap(ptr, (size + BLOCK_SIZE - 1) & (BLOCK_SIZE - 1));
            return;
        }

        let class = usize::bits() - size.leading_zeros() + LARGE_CLASS_SHIFT;
        let size = 1 << (class - LARGE_CLASS_SHIFT);
        self.free_bin(ptr, size, class);
    }

    #[inline]
    unsafe fn free_bin(&mut self, ptr: *mut u8, size: usize, class: usize) {
        let mut slot = P::new(ptr as *mut Slot);
        slot.next = self.cache[class];
        self.cache[class] = slot.to_opt();
        self.cache_size[class] += size;

        if unlikely!(self.cache_size[class] > 2 * CACHE_SIZE) {
            self.trim_cache(size, class);
        }
    }

    #[cold]
    #[inline(never)]
    unsafe fn trim_cache(&mut self, size: usize, class: usize) {
        while self.cache_size[class] > CACHE_SIZE + size {
            let slot = self.cache[class].unwrap();
            self.cache[class] = slot.next;
            self.cache_size[class] -= size;

            let mut chunk = P::new((slot.ptr() as usize & !CHUNK_MASK) as *mut Chunk);
            if unlikely!(chunk.free(slot, class)) {
                if let Some(mut prev) = chunk.prev {
                    prev.next = chunk.next;
                }
                if let Some(mut next) = chunk.next {
                    next.prev = chunk.prev;
                }
                if self.free_chunk.is_none() {
                    self.free_chunk = chunk.to_opt();
                } else {
                    sys::unmap(chunk.ptr(), CHUNK_SIZE);
                }
            }
        }
    }
}
