// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::atomic::{AtomicU8, AtomicI32, ATOMIC_I32_INIT, AtomicCInt};
use std::cty::{c_int};
use std::{mem};

#[test]
fn cint_size() {
    test!(mem::size_of::<AtomicCInt>() == mem::size_of::<c_int>());
}

#[test]
fn wrap() {
    let mut x = 2;
    unsafe {
        let ax = AtomicI32::wrap(&mut x);
        test!(ax.load() == 2);
        ax.store(1);
    }
    test!(x == 1);
}

#[test]
fn unwrap() {
    let x = AtomicI32::new(2);
    unsafe {
        test!(*x.unwrap() == 2);
        *x.unwrap() = 3;
    }
    test!(x.load() == 3);
}

#[test]
fn load() {
    let x = AtomicI32::new(6);
    test!(x.load_unordered() == 6);
    test!(x.load_weak() == 6);
    test!(x.load_acquire() == 6);
    test!(x.load() == 6);
}

#[test]
fn store() {
    let x = ATOMIC_I32_INIT;
    x.store_unordered(1);
    test!(x.load() == 1);
    x.store_weak(2);
    test!(x.load() == 2);
    x.store_release(3);
    test!(x.load() == 3);
    x.store(4);
    test!(x.load() == 4);
}

#[test]
fn exchange() {
    let x = ATOMIC_I32_INIT;
    test!(x.exchange_weak(1) == 0);
    test!(x.exchange_release(2) == 1);
    test!(x.exchange_acquire(3) == 2);
    test!(x.exchange_acquire_release(4) == 3);
    test!(x.exchange(5) == 4);
}

#[test]
fn compare_exchange() {
    let x = ATOMIC_I32_INIT;

    test!(x.compare_exchange_weak(-1, 1) == 0);
    test!(x.compare_exchange_release(-1, 2) == 0);
    test!(x.compare_exchange_acquire(-1, 3) == 0);
    test!(x.compare_exchange_acquire_release(-1, 4) == 0);
    test!(x.compare_exchange(-1, 5) == 0);

    test!(x.compare_exchange_weak(0, 1) == 0);
    test!(x.compare_exchange_release(1, 2) == 1);
    test!(x.compare_exchange_acquire(2, 3) == 2);
    test!(x.compare_exchange_acquire_release(3, 4) == 3);
    test!(x.compare_exchange(4, 5) == 4);
}

#[test]
fn add() {
    let x = ATOMIC_I32_INIT;
    test!(x.add_weak(1) == 0);
    test!(x.add_release(1) == 1);
    test!(x.add_acquire(1) == 2);
    test!(x.add_acquire_release(1) == 3);
    test!(x.add(1) == 4);
}

#[test]
fn sub() {
    let x = ATOMIC_I32_INIT;
    test!(x.sub_weak(1) == 0);
    test!(x.sub_release(1) == -1);
    test!(x.sub_acquire(1) == -2);
    test!(x.sub_acquire_release(1) == -3);
    test!(x.sub(1) == -4);
}

#[test]
fn and() {
    let x = AtomicI32::new(!0);
    test!(x.and_weak(0b11111) == !0);
    test!(x.and_release(0b01111) == 0b11111);
    test!(x.and_acquire(0b00111) == 0b01111);
    test!(x.and_acquire_release(0b00011) == 0b00111);
    test!(x.and(0b00001) == 0b00011);
}

#[test]
fn or() {
    let x = AtomicI32::new(0);
    test!(x.or_weak(0b00001) == 0);
    test!(x.or_release(0b00010) == 0b00001);
    test!(x.or_acquire(0b00100) == 0b00011);
    test!(x.or_acquire_release(0b01000) == 0b00111);
    test!(x.or(0b10000) == 0b01111);
}

#[test]
fn nand() {
    let x = AtomicU8::new(0b1111_1111);
    test!(x.nand_weak(0b0111_1111) == 0b1111_1111);
    test!(x.nand_release(0b1000_0000) == 0b1000_0000);
    test!(x.nand_acquire(0b0011_1111) == 0b0111_1111);
    test!(x.nand_acquire_release(0b0100_0000) == 0b1100_0000);
    test!(x.nand(0b1001_1111) == 0b1011_1111);
}

#[test]
fn xor() {
    let x = AtomicI32::new(0);
    test!(x.xor_weak(0b00001) == 0);
    test!(x.xor_release(0b00011) == 0b00001);
    test!(x.xor_acquire(0b00111) == 0b00010);
    test!(x.xor_acquire_release(0b01111) == 0b00101);
    test!(x.xor(0b11111) == 0b01010);
}

#[test]
fn min() {
    let x = ATOMIC_I32_INIT;
    test!(x.min_weak(-1) == 0);
    test!(x.min_release(-2) == -1);
    test!(x.min_acquire(-1) == -2);
    test!(x.min_acquire_release(-3) == -2);
    test!(x.min(-4) == -3);
}

#[test]
fn max() {
    let x = ATOMIC_I32_INIT;
    test!(x.max_weak(1) == 0);
    test!(x.max_release(2) == 1);
    test!(x.max_acquire(1) == 2);
    test!(x.max_acquire_release(3) == 2);
    test!(x.max(4) == 3);
}
