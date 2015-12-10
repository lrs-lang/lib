// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_hashmap"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_alloc as alloc;
extern crate lrs_hash as hash;
extern crate lrs_fmt as fmt;

use base::prelude::*;
use core::ops::{Eq};
use base::undef::{UndefState};
use alloc::{Heap};
use hash::{Hash};
use hash::xx_hash::{XxHash32};
use bucket::compact::{CompactBucket};
use bucket::loose::{LooseBucket};
use table::{GenericMap};

pub use table::{Entry, VacantEntry, OccupiedEntry};

mod std { pub use fmt::std::*; }

mod bucket;
mod table;

pub type CompactMap<Key, Value, Hasher = XxHash32, Seed = (), Allocator = Heap>
    where Allocator: alloc::MemPool,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+To,
          Key: Eq + Hash + UndefState
    = GenericMap<Key, Value, CompactBucket<Key, Value>, Hasher, Seed, Allocator>;

pub type HashMap<Key, Value, Hasher = XxHash32, Seed = (), Allocator = Heap>
    where Allocator: alloc::MemPool,
          Hasher: hash::Hasher,
          Seed: Into<Hasher::Seed>+To,
          Key: Eq + Hash + UndefState
    = GenericMap<Key, Value, LooseBucket<Key, Value>, Hasher, Seed, Allocator>;
