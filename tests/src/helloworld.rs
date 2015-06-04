#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[allow(unused_imports)] #[prelude_import] use lrs::prelude::*;

fn main() {
    println!("Hello World");
}
