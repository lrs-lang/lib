#![feature(custom_derive, plugin, plugin_registrar, rustc_private, unboxed_closures,
           slice_patterns, box_syntax, box_patterns, quote)]
#![crate_name = "lrs_core_plugin"]
#![crate_type = "dylib"]

#[macro_use]
extern crate rustc_plugin;
extern crate syntax;
extern crate syntax_ext;
extern crate fmt_macros;

macro_rules! panictry {
    ($e:expr) => ({
        use std::result::Result::{Ok, Err};
        use ::syntax::errors::FatalError;
        match $e {
            Ok(e) => e,
            Err(FatalError) => panic!(FatalError)
        }
    })
}

use syntax::ext::base::{MultiDecorator};
use syntax::parse::{token};
use rustc_plugin::{Registry};

mod lrs_ext;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        token::intern("derive_onlyCopy"),
        MultiDecorator(Box::new(lrs_ext::derive_copy)));

    reg.register_syntax_extension(
        token::intern("derive_To"),
        MultiDecorator(Box::new(lrs_ext::derive_to)));

    reg.register_syntax_extension(
        token::intern("derive_TryTo"),
        MultiDecorator(Box::new(lrs_ext::derive_try_to)));

    reg.register_syntax_extension(
        token::intern("derive_Copy"),
        MultiDecorator(Box::new(lrs_ext::derive_copy_and_to)));

    reg.register_syntax_extension(
        token::intern("derive_Pod"),
        MultiDecorator(Box::new(lrs_ext::derive_pod_copy_and_to)));

    reg.register_syntax_extension(
        token::intern("derive_Eq"),
        MultiDecorator(Box::new(lrs_ext::derive_eq)));

    reg.register_syntax_extension(
        token::intern("derive_Debug"),
        MultiDecorator(Box::new(lrs_ext::derive_debug)));

    reg.register_macro("write", lrs_ext::expand_format_args);
    reg.register_macro("derive_To_for", lrs_ext::derive_copy_to_for);
}
