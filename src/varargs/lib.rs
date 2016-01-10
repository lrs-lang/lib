// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_varargs"]
#![crate_type = "lib"]
#![no_std]

extern crate lrs_cty as cty;

#[cfg(target_arch = "x86_64")] #[path = "x86_64.rs"] mod arch;
#[cfg(target_arch = "x86")] #[path = "x86.rs"] mod arch;
#[cfg(target_arch = "arm")] #[path = "arm.rs"] mod arch;
#[cfg(target_arch = "aarch64")] #[path = "aarch64.rs"] mod arch;

/// A list of variable arguments.
///
/// = Remarks
///
/// This type is compatible with the `va_list` C type in arguments.
#[repr(C)]
pub struct VarArgs {
    inner: arch::VarArgs,
}

impl VarArgs {
    /// Retrieves an object from the list of variable arguments.
    pub unsafe fn get<T: VarArg>(&mut self) -> T {
        self.inner.get()
    }
}

/// Objects that can be retrieved from a VarArgs object.
///
/// = Remarks
///
/// This trait must not be implemented by user code.
pub unsafe trait VarArg: Sized { }

unsafe impl    VarArg for u8       { }
unsafe impl    VarArg for u16      { }
unsafe impl    VarArg for u32      { }
unsafe impl    VarArg for u64      { }
unsafe impl    VarArg for usize    { }
unsafe impl    VarArg for i8       { }
unsafe impl    VarArg for i16      { }
unsafe impl    VarArg for i32      { }
unsafe impl    VarArg for i64      { }
unsafe impl    VarArg for isize    { }
unsafe impl    VarArg for f32      { }
unsafe impl    VarArg for f64      { }
unsafe impl<T> VarArg for *const T { }
unsafe impl<T> VarArg for *mut T   { }
