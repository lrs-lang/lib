// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{c_int};
use {VarArg};
use core::{mem, ptr};
use core::intrinsics::{type_id};

#[repr(C)]
pub struct VarArgs {
    __stack: *mut u8,
    __gr_top: *mut u8,
    __vr_top: *mut u8,
    __gr_offs: c_int,
    __vr_offs: c_int,
}

impl VarArgs {
    pub unsafe fn get<T: VarArg>(&mut self) -> T {
        if fp::<T>() {
            let mut offs = self.__vr_offs as isize;
            if offs <= -16 {
                self.__vr_offs += 16;
                if cfg!(target_endian = "big") {
                    offs += 16 - mem::size_of::<T>() as isize;
                }
                return ptr::read(self.__vr_top.offset(offs) as *mut T);
            }
        } else {
            let mut offs = self.__gr_offs as isize;
            if offs <= -8 {
                self.__gr_offs += 8;
                if cfg!(target_endian = "big") {
                    offs += 8 - mem::size_of::<T>() as isize
                }
                return ptr::read(self.__gr_top.offset(offs) as *mut T);
            }
        }

        let mut ptr = self.__stack;
        self.__stack = ptr.add(8);
        if cfg!(target_endian = "big") {
            ptr = ptr.add(8 - mem::size_of::<T>());
        }
        ptr::read(ptr as *mut T)
    }
}

unsafe fn fp<T>() -> bool {
    let id = type_id::<T>();
    id == type_id::<f32>() || id == type_id::<f64>()
}
