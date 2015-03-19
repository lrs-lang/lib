#![crate_name = "linux"]
#![crate_type = "lib"]
#![feature(std_misc, core, io, plugin, asm)]
#![plugin(rest_easy)]

pub mod file;
pub mod user;
pub mod group;
pub mod errno;
pub mod result;
pub mod fs;

mod imp;
