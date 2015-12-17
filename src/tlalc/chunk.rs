// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};

use {sys};
use p::{P, POpt};
use {
    BLOCK_SIZE, CHUNK_SIZE, BLOCKS_PER_CHUNK, CHUNK_MASK, BLOCK_SHIFT, BLOCK_MASK,
};
use util::{slots_per_class};

pub struct Slot {
    pub next: POpt<Slot>,
}

#[repr(C)]
pub struct RawBlock {
    _unused: [usize; 5],
}

#[repr(C)]
pub struct FreeBlock {
    next: POpt<FreeBlock>,
    mem: *mut u8,
    _unused: [usize; 3],
}

#[repr(C)]
pub struct BusyBlock {
    next: POpt<BusyBlock>,
    prev: POpt<BusyBlock>,
    slot: P<Slot>,
    last: P<Slot>,
    live_slots: usize,
}

pub struct Chunk {
    pub next: POpt<Chunk>,
    pub prev: POpt<Chunk>,
    live_blocks: usize,
    free_block: POpt<FreeBlock>,
    cache: [POpt<BusyBlock>; 20],
    blocks: [RawBlock; BLOCKS_PER_CHUNK],
}

impl Chunk {
    /// Allocates a new chunk.
    pub unsafe fn new() -> Result<P<Chunk>> {
        let mut chunk  = P::new(try!(sys::map_chunk()));

        let (first_header, mut cur_block) = {
            let offset = align!(mem::size_of::<Chunk>(), [%] BLOCK_SIZE);
            (P::new(&chunk.blocks[offset/BLOCK_SIZE] as *const _ as *const FreeBlock),
             (chunk.ptr() as *mut u8).add(offset))
        };
        let last_block = (chunk.ptr() as *mut u8).add(CHUNK_SIZE - BLOCK_SIZE);

        let mut cur_header = first_header.ptr();
        while cur_block != last_block {
            let next_header = cur_header.add(1);
            (*cur_header).next = POpt::some(next_header);
            (*cur_header).mem = cur_block;
            cur_header = next_header;
            cur_block = cur_block.add(BLOCK_SIZE);
        }
        (*cur_header).next = POpt::none();
        (*cur_header).mem = cur_block;

        chunk.free_block = first_header.to_opt();

        Ok(chunk)
    }

    /// Allocates slots.
    ///
    /// [argument, size]
    /// The upper bound of `class`.
    ///
    /// [argument, class]
    /// The size class of the block.
    pub unsafe fn alloc(&mut self, size: usize,
                        class: usize) -> Option<(P<Slot>, P<Slot>, usize)> {
        let mut rsize = 0;
        let mut first = POpt::none();
        let mut last = POpt::none();
        let slots_per_class = slots_per_class(class);

        while rsize < BLOCK_SIZE {
            if let Some(mut block) = *self.cache[class] {
                self.cache[class] = block.next;
                rsize += size * (slots_per_class - block.live_slots);
                block.live_slots = 0;
                block.last.next = first;
                if last.is_none() {
                    last = block.last.to_opt();
                }
                first = block.slot.to_opt();
            } else {
                break;
            }
        }

        if let Some(mut block) = *self.cache[class] {
            block.prev = POpt::none();
        }

        if rsize < BLOCK_SIZE {
            if let Some((sfirst, mut slast)) = self.split_block(size, class) {
                rsize += size * slots_per_class;
                slast.next = first;
                if last.is_none() {
                    last = slast.to_opt();
                }
                first = sfirst.to_opt();
            }
        }

        if first.is_some() {
            Some((first.unwrap(), last.unwrap(), rsize))
        } else {
            None
        }
    }

    /// Tries to split a block into slots of the specified size.
    ///
    /// [argument, size]
    /// The upper bound of `class`.
    ///
    /// [argument, class]
    /// The size class of the block.
    ///
    /// [return_value]
    /// Returns the first and last slot in the block.
    unsafe fn split_block(&mut self, size: usize,
                          class: usize) -> Option<(P<Slot>, P<Slot>)> {
        let mem = match *self.free_block {
            Some(block) => {
                self.live_blocks += 1;
                self.free_block = block.next;
                let mem = block.mem;
                let mut block: P<BusyBlock> = mem::cast(block);
                block.live_slots = 0;
                mem
            },
            _ => return None,
        };

        let slots = slots_per_class(class);
        let first_slot = P::new(mem as *mut Slot);
        let mut last_slot = P::new(mem.add((slots - 1) * size) as *mut Slot);

        let mut cur_slot = first_slot.ptr();
        while cur_slot != last_slot.ptr() {
            let next_slot = (cur_slot as *mut u8).add(size) as *mut Slot;
            (*cur_slot).next = mem::cast(next_slot);
            cur_slot = next_slot;
        }
        last_slot.next = POpt::none();

        Some((first_slot, last_slot))
    }

    /// Frees a slot.
    ///
    /// [argument, slot]
    /// The slot to free.
    ///
    /// [argument, class]
    /// The size class of the slot.
    ///
    /// [return_value]
    /// Returns whether all blocks of the chunk are unused after the operation.
    pub unsafe fn free(&mut self, mut slot: P<Slot>, class: usize) -> bool {
        let addr = slot.ptr() as usize;
        let block_idx = (addr & CHUNK_MASK) >> BLOCK_SHIFT;
        let mut block: P<BusyBlock> = mem::cast(&self.blocks[block_idx]);
        slot.next = block.slot.to_opt();
        block.slot = slot;

        if likely!(block.live_slots > 1) {
            block.live_slots -= 1;
            return false;
        }

        if block.live_slots == 0 {
            block.live_slots = slots_per_class(class) - 1;

            if likely!(block.live_slots > 0) {
                slot.next = POpt::none();

                block.next = self.cache[class];
                block.prev = POpt::none();
                block.last = slot;
                if let Some(mut next) = *block.next {
                    next.prev = block.to_opt();
                }
                self.cache[class] = block.to_opt();

                return false
            }
        } else {
            if let Some(mut prev) = *block.prev {
                prev.next = block.next;
            } else {
                self.cache[class] = block.next;
            }
            if let Some(mut next) = *block.next {
                next.prev = block.prev
            }
        }

        let mut block: P<FreeBlock> = mem::cast(block);
        block.mem = (addr & !BLOCK_MASK) as *mut u8;
        block.next = self.free_block;
        self.free_block = block.to_opt();
        self.live_blocks -= 1;

        self.live_blocks == 0
    }
}
