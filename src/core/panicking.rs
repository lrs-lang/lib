// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[lang = "stack_exhausted"]
extern fn stack_exhausted() {
    abort!();
}

// #[inline(always)]
#[lang = "panic_bounds_check"]
fn panic_bounds_check(_: &(&'static str, u32), _: usize, _: usize) -> ! {
    unsafe { ::intrinsics::breakpoint(); }
    abort!();
}

// #[inline(always)]
#[lang = "panic"]
pub fn panic(_: &(&'static str, &'static str, u32)) -> ! {
    unsafe { ::intrinsics::breakpoint(); }
    abort!();
}

#[lang="eh_personality"]
#[no_mangle]
#[allow(private_no_mangle_fns)]
extern fn rust_eh_personality() {
    abort!();
}
