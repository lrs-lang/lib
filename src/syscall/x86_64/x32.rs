// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{
    c_longlong, __X32_SYSCALL_BIT
};

/// Syscall type
pub type SCT = c_longlong;

#[inline(always)]
pub unsafe fn syscall0(n: SCT) -> SCT {
    ::arch::common0(n + __X32_SYSCALL_BIT)
}

#[inline(always)]
pub unsafe fn syscall1(n: SCT, a1: SCT) -> SCT {
    ::arch::common1(n + __X32_SYSCALL_BIT, a1)
}

#[inline(always)]
pub unsafe fn syscall2(n: SCT, a1: SCT, a2: SCT) -> SCT {
    ::arch::common2(n + __X32_SYSCALL_BIT, a1, a2)
}

#[inline(always)]
pub unsafe fn syscall3(n: SCT, a1: SCT, a2: SCT, a3: SCT) -> SCT {
    ::arch::common3(n + __X32_SYSCALL_BIT, a1, a2, a3)
}

#[inline(always)]
pub unsafe fn syscall4(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT) -> SCT {
    ::arch::common4(n + __X32_SYSCALL_BIT, a1, a2, a3, a4)
}

#[inline(always)]
pub unsafe fn syscall5(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT) -> SCT {
    ::arch::common5(n + __X32_SYSCALL_BIT, a1, a2, a3, a4, a5)
}

#[inline(always)]
pub unsafe fn syscall6(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT,
                       a6: SCT) -> SCT {
    ::arch::common6(n + __X32_SYSCALL_BIT, a1, a2, a3, a4, a5, a6)
}
