// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
extern crate lrs_hash;
extern crate lrs_hashmap;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs::{mem};
use lrs_hashmap::{CompactHashMap, HashMap, Entry};
use lrs_hash::{Hasher};
use lrs_hash::xx_hash::{XxHash32, XxHash64};

const MAX_VAL: u32 = 10000000;
const MASK: u64 = 131072;

fn main() {
    let mut map: CompactHashMap<_, _> = CompactHashMap::new().unwrap();
    // map.reserve(10000).unwrap();
//     let mut collisions = 0;
//     let mut bits = [0; 32];
    for i in 0..5000 {
        let c: char = unsafe { mem::cast(i) };
        map.set(c, ());
    }
    map.debug();
//         let hash = XxHash32::hash(&i, 0u32) % (1 << 17);
//         macro_rules! test {
//             ($pos:expr) => { bits[$pos] += (hash & (1 << $pos) != 0) as u32; }
//         };
//         test!(0);
//         test!(1);
//         test!(2);
//         test!(3);
//         test!(4);
//         test!(5);
//         test!(6);
//         test!(7);
//         test!(8);
//         test!(9);
//         test!(10);
//         test!(11);
//         test!(12);
//         test!(13);
//         test!(14);
//         test!(15);
//         test!(16);
//         test!(17);
//         test!(18);
//         test!(19);
//         test!(20);
//         test!(21);
//         test!(22);
//         test!(23);
//         test!(24);
//         test!(25);
//         test!(26);
//         test!(27);
//         test!(28);
//         test!(29);
//         test!(30);
//         test!(31);
//     }
//     for i in 0..32usize {
//         println!("{:?}", bits[i]);
//     }
//         // println!("{}", hash);
//         // println!(
//         // if map.find(&hash).is_some() {
//         //     collisions += 1;
//         // } else {
//         //     map.set(hash, ());
//         // }
//         // match map.entry(&hash).unwrap() {
//         //     Entry::Vacant(v) => { v.set(hash, ()); },
//         //     Entry::Occupied(_) => collisions += 1,
//         // }
//         // println!("{}", 
//         // let c: char = unsafe { mem::cast(i) };
//         // map.set(c, ());
// 
    // println!("{}", collisions);
        // let trailing = MASK.trailing_zeros();
        // let height = 1 << (trailing / 2);
        // let width = 1 << (trailing - trailing / 2);
        // println!("P1");
        // println!("{} {}", width, height);
        // for i in 0..MASK {
        //     unsafe {
        //         if map.find(&i).is_none() {
        //             write!(lrs::fd::STDOUT, "0 ");
        //         } else {
        //             write!(lrs::fd::STDOUT, "1 ");
        //         }
        //         if (i + 1) % width == 0 {
        //             println!("");
        //         }
        //     }
        // }
    // map.debug();
    // for i in 0u8..255 {
    //     println!("{:?}, {:?}", i as char, map.find(&(i as char)));
    // }
}

