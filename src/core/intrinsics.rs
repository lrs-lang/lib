// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern "rust-intrinsic" {
    pub fn discriminant_value<T>(v: &T) -> u64;
    pub fn abort() -> !;
    pub fn breakpoint();
    pub fn size_of<T>() -> usize;
    pub fn move_val_init<T>(dst: &mut T, src: T);
    pub fn min_align_of<T>() -> usize;
    pub fn pref_align_of<T>() -> usize;
    pub fn init_dropped<T>() -> T;
    pub fn init<T>() -> T;
    pub fn uninit<T>() -> T;
    pub fn forget<T>(_: T) -> ();
    pub fn transmute<T,U>(e: T) -> U;
    pub fn needs_drop<T>() -> bool;
    pub fn offset<T>(dst: *const T, offset: isize) -> *const T;
    pub fn sqrtf32(x: f32) -> f32;
    pub fn sqrtf64(x: f64) -> f64;
    pub fn powif32(a: f32, x: i32) -> f32;
    pub fn powif64(a: f64, x: i32) -> f64;
    pub fn sinf32(x: f32) -> f32;
    pub fn sinf64(x: f64) -> f64;
    pub fn cosf32(x: f32) -> f32;
    pub fn cosf64(x: f64) -> f64;
    pub fn powf32(a: f32, x: f32) -> f32;
    pub fn powf64(a: f64, x: f64) -> f64;
    pub fn expf32(x: f32) -> f32;
    pub fn expf64(x: f64) -> f64;
    pub fn exp2f32(x: f32) -> f32;
    pub fn exp2f64(x: f64) -> f64;
    pub fn logf32(x: f32) -> f32;
    pub fn logf64(x: f64) -> f64;
    pub fn log10f32(x: f32) -> f32;
    pub fn log10f64(x: f64) -> f64;
    pub fn log2f32(x: f32) -> f32;
    pub fn log2f64(x: f64) -> f64;
    pub fn fmaf32(a: f32, b: f32, c: f32) -> f32;
    pub fn fmaf64(a: f64, b: f64, c: f64) -> f64;
    pub fn fabsf32(x: f32) -> f32;
    pub fn fabsf64(x: f64) -> f64;
    pub fn copysignf32(x: f32, y: f32) -> f32;
    pub fn copysignf64(x: f64, y: f64) -> f64;
    pub fn floorf32(x: f32) -> f32;
    pub fn floorf64(x: f64) -> f64;
    pub fn ceilf32(x: f32) -> f32;
    pub fn ceilf64(x: f64) -> f64;
    pub fn truncf32(x: f32) -> f32;
    pub fn truncf64(x: f64) -> f64;
    pub fn rintf32(x: f32) -> f32;
    pub fn rintf64(x: f64) -> f64;
    pub fn nearbyintf32(x: f32) -> f32;
    pub fn nearbyintf64(x: f64) -> f64;
    pub fn roundf32(x: f32) -> f32;
    pub fn roundf64(x: f64) -> f64;
    pub fn ctpop8(x: u8) -> u8;
    pub fn ctpop16(x: u16) -> u16;
    pub fn ctpop32(x: u32) -> u32;
    pub fn ctpop64(x: u64) -> u64;
    pub fn ctlz8(x: u8) -> u8;
    pub fn ctlz16(x: u16) -> u16;
    pub fn ctlz32(x: u32) -> u32;
    pub fn ctlz64(x: u64) -> u64;
    pub fn cttz8(x: u8) -> u8;
    pub fn cttz16(x: u16) -> u16;
    pub fn cttz32(x: u32) -> u32;
    pub fn cttz64(x: u64) -> u64;
    pub fn bswap16(x: u16) -> u16;
    pub fn bswap32(x: u32) -> u32;
    pub fn bswap64(x: u64) -> u64;
    pub fn i8_add_with_overflow(x: i8, y: i8) -> (i8, bool);
    pub fn i16_add_with_overflow(x: i16, y: i16) -> (i16, bool);
    pub fn i32_add_with_overflow(x: i32, y: i32) -> (i32, bool);
    pub fn i64_add_with_overflow(x: i64, y: i64) -> (i64, bool);
    pub fn u8_add_with_overflow(x: u8, y: u8) -> (u8, bool);
    pub fn u16_add_with_overflow(x: u16, y: u16) -> (u16, bool);
    pub fn u32_add_with_overflow(x: u32, y: u32) -> (u32, bool);
    pub fn u64_add_with_overflow(x: u64, y: u64) -> (u64, bool);
    pub fn i8_sub_with_overflow(x: i8, y: i8) -> (i8, bool);
    pub fn i16_sub_with_overflow(x: i16, y: i16) -> (i16, bool);
    pub fn i32_sub_with_overflow(x: i32, y: i32) -> (i32, bool);
    pub fn i64_sub_with_overflow(x: i64, y: i64) -> (i64, bool);
    pub fn u8_sub_with_overflow(x: u8, y: u8) -> (u8, bool);
    pub fn u16_sub_with_overflow(x: u16, y: u16) -> (u16, bool);
    pub fn u32_sub_with_overflow(x: u32, y: u32) -> (u32, bool);
    pub fn u64_sub_with_overflow(x: u64, y: u64) -> (u64, bool);
    pub fn i8_mul_with_overflow(x: i8, y: i8) -> (i8, bool);
    pub fn i16_mul_with_overflow(x: i16, y: i16) -> (i16, bool);
    pub fn i32_mul_with_overflow(x: i32, y: i32) -> (i32, bool);
    pub fn i64_mul_with_overflow(x: i64, y: i64) -> (i64, bool);
    pub fn u8_mul_with_overflow(x: u8, y: u8) -> (u8, bool);
    pub fn u16_mul_with_overflow(x: u16, y: u16) -> (u16, bool);
    pub fn u32_mul_with_overflow(x: u32, y: u32) -> (u32, bool);
    pub fn u64_mul_with_overflow(x: u64, y: u64) -> (u64, bool);
    pub fn overflowing_add<T>(a: T, b: T) -> T;
    pub fn overflowing_sub<T>(a: T, b: T) -> T;
    pub fn overflowing_mul<T>(a: T, b: T) -> T;
    pub fn copy<T>(src: *const T, dst: *mut T, count: usize);
    pub fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);

    pub fn atomic_cxchg         <T>(dst: *mut T, old: T, src: T) -> T;
    pub fn atomic_cxchg_acq     <T>(dst: *mut T, old: T, src: T) -> T;
    pub fn atomic_cxchg_rel     <T>(dst: *mut T, old: T, src: T) -> T;
    pub fn atomic_cxchg_acqrel  <T>(dst: *mut T, old: T, src: T) -> T;
    pub fn atomic_cxchg_relaxed <T>(dst: *mut T, old: T, src: T) -> T;

    pub fn atomic_load           <T>(src: *const T) -> T;
    pub fn atomic_load_acq       <T>(src: *const T) -> T;
    pub fn atomic_load_relaxed   <T>(src: *const T) -> T;
    pub fn atomic_load_unordered <T>(src: *const T) -> T;

    pub fn atomic_store           <T>(dst: *mut T, val: T);
    pub fn atomic_store_rel       <T>(dst: *mut T, val: T);
    pub fn atomic_store_relaxed   <T>(dst: *mut T, val: T);
    pub fn atomic_store_unordered <T>(dst: *mut T, val: T);

    pub fn atomic_xchg         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xchg_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_xadd         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xadd_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_xsub         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xsub_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_and         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_and_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_nand         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_nand_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_or         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_or_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_xor         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_xor_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_max         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_max_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_min         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_min_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_umin         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umin_relaxed <T>(dst: *mut T, src: T) -> T;

    pub fn atomic_umax         <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_acq     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_rel     <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_acqrel  <T>(dst: *mut T, src: T) -> T;
    pub fn atomic_umax_relaxed <T>(dst: *mut T, src: T) -> T;
}

pub unsafe fn bswap8(x: u8) -> u8 { x }
