#![allow(dead_code)]

pub mod syscall {
    pub use super::arch::syscall::*;
}

pub mod cty {
    pub use super::arch::cty::*;
}

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
mod arch;
