// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{c_uint};
use {VarArg};
use core::ptr::{self, NoAliasMutObjPtr};
use core::intrinsics::{type_id};

#[repr(C)]
pub struct VarArgs {
    int: *mut VarInt,
}

#[repr(C)]
struct VarInt {
    gp_offset: c_uint,
    fp_offset: c_uint,
    overflow_arg_area: *mut u8,
    reg_save_area: *mut u8,
}

impl VarArgs {
    pub unsafe fn get<T: VarArg>(&mut self) -> T {
        let mut var = NoAliasMutObjPtr::new(self.int);

        if fp::<T>() {
            // ABI says here 304 instead of 176 which is 48 + 16 * 16 instead of
            // 48 + 8 * 16. That is, ABI says that up to 16 floating point registers are
            // used as arguments. However, GCC says that only 8 fp registers are used and
            // this agrees with what the ABI says about argument passing elsewhere.
            if var.fp_offset <= 176 - 16 {
                let val = ptr::read(var.reg_save_area.add(var.fp_offset as usize) as *mut T);
                var.fp_offset += 16;
                return val;
            }
        } else {
            if var.gp_offset <= 48 - 8 {
                let val = ptr::read(var.reg_save_area.add(var.gp_offset as usize) as *mut T);
                var.gp_offset += 8;
                return val;
            }
        }

        let val = ptr::read(var.overflow_arg_area as *mut T);
        var.overflow_arg_area = var.overflow_arg_area.add(8);
        val
    }
}

unsafe fn fp<T>() -> bool {
    let id = type_id::<T>();
    id == type_id::<f32>() || id == type_id::<f64>()
}
