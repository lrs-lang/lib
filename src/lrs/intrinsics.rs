// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Compiler intrinsics
//!
//! = Description
//!
//! This module provides direct access to compiler built-in functions. They are all unsafe
//! and have safe wrappers in other modules.

pub use lrs_core::intrinsics::{
    discriminant_value, abort, breakpoint, size_of, move_val_init, min_align_of,
    pref_align_of, init_dropped, init, uninit, forget, transmute, needs_drop, offset,
    sqrtf32, sqrtf64, powif32, powif64, sinf32, sinf64, cosf32, cosf64, powf32, powf64,
    expf32, expf64, exp2f32, exp2f64, logf32, logf64, log10f32, log10f64, log2f32,
    log2f64, fmaf32, fmaf64, fabsf32, fabsf64, copysignf32, copysignf64, floorf32,
    floorf64, ceilf32, ceilf64, truncf32, truncf64, rintf32, rintf64, nearbyintf32,
    nearbyintf64, roundf32, roundf64, ctpop8, ctpop16, ctpop32, ctpop64, ctlz8, ctlz16,
    ctlz32, ctlz64, cttz8, cttz16, cttz32, cttz64, bswap16, bswap32, bswap64,
    i8_add_with_overflow, i16_add_with_overflow, i32_add_with_overflow,
    i64_add_with_overflow, u8_add_with_overflow, u16_add_with_overflow,
    u32_add_with_overflow, u64_add_with_overflow, i8_sub_with_overflow,
    i16_sub_with_overflow, i32_sub_with_overflow, i64_sub_with_overflow,
    u8_sub_with_overflow, u16_sub_with_overflow, u32_sub_with_overflow,
    u64_sub_with_overflow, i8_mul_with_overflow, i16_mul_with_overflow,
    i32_mul_with_overflow, i64_mul_with_overflow, u8_mul_with_overflow,
    u16_mul_with_overflow, u32_mul_with_overflow, u64_mul_with_overflow, overflowing_add,
    overflowing_sub, overflowing_mul, copy, copy_nonoverlapping, atomic_cxchg,
    atomic_cxchg_acq, atomic_cxchg_rel, atomic_cxchg_acqrel, atomic_cxchg_relaxed,
    atomic_load, atomic_load_acq, atomic_load_relaxed, atomic_load_unordered,
    atomic_store, atomic_store_rel, atomic_store_relaxed, atomic_store_unordered,
    atomic_xchg, atomic_xchg_acq, atomic_xchg_rel, atomic_xchg_acqrel,
    atomic_xchg_relaxed, atomic_xadd, atomic_xadd_acq, atomic_xadd_rel,
    atomic_xadd_acqrel, atomic_xadd_relaxed, atomic_xsub, atomic_xsub_acq,
    atomic_xsub_rel, atomic_xsub_acqrel, atomic_xsub_relaxed, atomic_and, atomic_and_acq,
    atomic_and_rel, atomic_and_acqrel, atomic_and_relaxed, atomic_nand, atomic_nand_acq,
    atomic_nand_rel, atomic_nand_acqrel, atomic_nand_relaxed, atomic_or, atomic_or_acq,
    atomic_or_rel, atomic_or_acqrel, atomic_or_relaxed, atomic_xor, atomic_xor_acq,
    atomic_xor_rel, atomic_xor_acqrel, atomic_xor_relaxed, atomic_max, atomic_max_acq,
    atomic_max_rel, atomic_max_acqrel, atomic_max_relaxed, atomic_min, atomic_min_acq,
    atomic_min_rel, atomic_min_acqrel, atomic_min_relaxed, atomic_umin, atomic_umin_acq,
    atomic_umin_rel, atomic_umin_acqrel, atomic_umin_relaxed, atomic_umax,
    atomic_umax_acq, atomic_umax_rel, atomic_umax_acqrel, atomic_umax_relaxed,
    atomic_fence, atomic_fence_acq, atomic_fence_rel, atomic_fence_acqrel, bswap8,
    lrs_abort,
};
