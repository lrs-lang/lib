// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use marker::{Sized};

extern "rust-intrinsic" {
    /// Calculates the discriminant of an enum variant.
    ///
    /// [argument, v]
    /// The variant whose discriminant will be calculated.
    ///
    /// [return_value]
    /// Returns the discriminant value cast to an `u64` or `0` if the argument does not
    /// have a discriminant value.
    ///
    /// = Remarks
    ///
    /// If the discriminant cannot be represented in an `i64`, the result is unspecified.
    /// Note that it is `i64`, not `u64`.
    ///
    /// = Examples
    ///
    /// ----
    /// enum T {
    ///     X,
    ///     Y,
    /// }
    ///
    /// assert!(unsafe { discriminant_value(&T::X) } == 0);
    /// ----
    pub fn discriminant_value<T>(v: &T) -> u64;

    /// Aborts the process.
    ///
    /// = Remarks
    ///
    /// :abt: link:lrs::intrinsics::lrs_abort[lrs_abort]
    ///
    /// You should never use this function directly since it can cause strange code
    /// generation even in unoptimized builds. Instead use {abt} which simply calls this
    /// function.
    ///
    /// = See also
    ///
    /// * {abt}
    pub fn abort() -> !;

    /// Informs the optimizer that the execution cannot reach this point.
    ///
    /// = Remarks
    ///
    /// If it can reach this point, the behavior is undefined.
    pub fn unreachable() -> !;

    /// Raises a `SIGTRAP`.
    ///
    /// = Remarks
    ///
    /// This is useful for debugging as it causes debuggers to interrupt the program at
    /// this point. Note that the program will terminate if the signal is not caught.
    pub fn breakpoint();

    /// Calculates the size of a type.
    ///
    /// [return_value]
    /// Returns the size of `T` objects.
    ///
    /// = Remarks
    ///
    /// :so: link:lrs::mem::size_of[size_of]
    ///
    /// You should never use this function directly. Use {so} instead.
    ///
    /// = See also
    ///
    /// * {so}
    pub fn size_of<T>() -> usize;

    /// Stores a new value in an object without running the object's destructor.
    ///
    /// [argument, dst]
    /// The location where `src` will be stored.
    ///
    /// [argument, src]
    /// The value to store in `dst`.
    ///
    /// = Remarks
    ///
    /// :uninit: link:lrs::intrinsics::uninit[uninit]
    ///
    /// This can be used to initialize memory that was previously created with {uninit}.
    ///
    /// = Examples
    ///
    /// ----
    /// unsafe fn f(x: T) {
    ///     let mut y = uninit():
    ///     move_val_init(&mut y, x);
    /// }
    /// ----
    ///
    /// = See also
    ///
    /// * {uninit}
    pub fn move_val_init<T>(dst: *mut T, src: T);

    /// Runs the destructor of an object.
    ///
    /// [argument, obj]
    /// A pointer to the object whose destructor will be run.
    pub fn drop_in_place<T: ?Sized>(obj: *mut T);

    /// Calculates the alignment of a type.
    ///
    /// [return_value]
    /// Returns the alignment of `T` objects.
    ///
    /// = Remarks
    ///
    /// :ao: link:lrs::mem::align_of[align_of]
    ///
    /// You should never use this function directly. Use {ao} instead.
    ///
    /// = See also
    ///
    /// * {ao}
    pub fn min_align_of<T>() -> usize;

    /// Creates a value that appears to have had its destructor run.
    ///
    /// [return_value]
    /// Returns an object that appears to have had its destructor run.
    ///
    /// = Remarks
    ///
    /// The returned object has all of its bytes set to a special value which will prevent
    /// its destructor from running when it goes out of scope.
    ///
    /// = See also
    ///
    /// * link:lrs::intrinsics::init
    pub fn init_dropped<T>() -> T;

    /// Creates a value with all bytes set to zero.
    ///
    /// [return_value]
    /// Returns an object with all bytes set to zero.
    ///
    /// = Remarks
    ///
    /// :i: link:lrs::mem::init
    /// :pod: link:lrs::marker::Pod[Pod]
    ///
    /// For {pod} objects you should use the safe {i} instead.
    ///
    /// = See also
    ///
    /// * {i}
    /// * {pod}
    pub fn init<T>() -> T;

    /// Creates an uninitialized object.
    ///
    /// [return_value]
    /// Returns an uninitialized object.
    ///
    /// = Remarks
    ///
    /// This is a very efficient way to create large objects that will later be
    /// initialized. Note that this is *not* the same as creating an object that contains
    /// the bytes that were previously stored in the memory location the object is stored
    /// in. If the returned object is used before it is properly initialized, the behavior
    /// is undefined.
    ///
    /// = See also
    ///
    /// * link:lrs::mem::uninit
    pub fn uninit<T>() -> T;

    /// Moves an object without running its destructor.
    ///
    /// [argument, val]
    /// The object to be forgotten.
    ///
    /// = Remarks
    ///
    /// :f: link:lrs::mem::forget
    ///
    /// The object will be moved into the forget function which will return without
    /// running the object's destructor. This is unsafe because it can cause objects whose
    /// destructor must run at the end of their lifetime to not be destroyed.
    ///
    /// You probably want to use the safe variant {f}.
    ///
    /// = See also
    ///
    /// * {f}
    pub fn forget<T>(val: T);

    /// Casts an object to another type.
    ///
    /// [argument, val]
    /// The object to be cast.
    ///
    /// [return_vale]
    /// Returns the same object but interpreted as an object of type `U`.
    ///
    /// = Remarks
    ///
    /// The returned object has the same memory representation as the argument. The
    /// types must have the same size. This is checked at compile time.
    ///
    /// = See also
    ///
    /// * link:lrs::mem::cast
    /// * link:lrs::mem::size_of
    pub fn transmute<T, U>(val: T) -> U;

    /// Checks whether a type has a destructor.
    ///
    /// [return_value]
    /// Return whether `T` has a destructor.
    pub fn needs_drop<T>() -> bool;

    /// Creates a pointer by calculating an offset from another one.
    ///
    /// [argument, dst]
    /// The original pointer.
    ///
    /// [argument, offset]
    /// The offset to be added to the original pointer.
    ///
    /// [return_value]
    /// Returns the offset pointer.
    ///
    /// = Remarks
    ///
    /// The offset argument is in units of `T`, not in byte units. If `dst` is not a valid
    /// pointer, or `dst + offset` does not point into the same object, or `dst + offset`
    /// overflows, the behavior is undefined.
    ///
    /// Using this function instead of casting to integers can enable more optimizations.
    pub fn offset<T>(dst: *const T, offset: isize) -> *const T;

    /// Copies memory between two pointers.
    ///
    /// [argument, src]
    /// The source of the memory.
    ///
    /// [argument, dst]
    /// Where the memory will be stored.
    ///
    /// [argument, count]
    /// The number of `T` objects to copy.
    ///
    /// = Remarks
    ///
    /// :copy: link:lrs::ptr::memmove[memmove]
    ///
    /// Never use this function. Use :copy: instead.
    ///
    /// = See also
    ///
    /// * {copy}
    pub fn copy<T>(src: *const T, dst: *mut T, count: usize);

    /// Copies memory between two non-overlapping pointers.
    ///
    /// [argument, src]
    /// The source of the memory.
    ///
    /// [argument, dst]
    /// Where the memory will be stored.
    ///
    /// [argument, count]
    /// The number of `T` objects to copy.
    ///
    /// = Remarks
    ///
    /// :copy: link:lrs::ptr::memcpy[memcpy]
    ///
    /// Never use this function. Use :copy: instead.
    ///
    /// = See also
    ///
    /// * {copy}
    pub fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);

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

    pub fn atomic_fence();
    pub fn atomic_fence_acq();
    pub fn atomic_fence_rel();
    pub fn atomic_fence_acqrel();

    pub fn atomic_singlethreadfence();
    pub fn atomic_singlethreadfence_acq();
    pub fn atomic_singlethreadfence_rel();
    pub fn atomic_singlethreadfence_acqrel();
}

pub unsafe fn bswap8(x: u8) -> u8 { x }

/// Aborts the process.
///
/// = Remarks
///
/// :abort: link:lrs::abort![abort!]
///
/// This function is called by the {abort} macro and exists for easier debugging as
/// calling the `abort` intrinsic directly can cause the code to be modified even without
/// optimization enabled.
#[no_mangle]
#[inline]
pub fn lrs_abort() -> ! {
    unsafe { abort() }
}
