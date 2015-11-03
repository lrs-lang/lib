#![feature(custom_derive, plugin, plugin_registrar, rustc_private, unboxed_closures,
           slice_patterns, box_syntax, box_patterns, libc, quote)]
#![crate_name = "lrs_core_plugin"]
#![crate_type = "dylib"]

#[macro_use]
extern crate rustc;

extern crate syntax as _syntax;
extern crate arena;
extern crate fmt_macros;
extern crate serialize;
extern crate term;
extern crate libc;
#[macro_use] extern crate log;
#[macro_use] #[no_link] extern crate rustc_bitflags;
extern crate serialize as rustc_serialize;

macro_rules! panictry {
    ($e:expr) => ({
        use std::result::Result::{Ok, Err};
        use diagnostic::FatalError;
        match $e {
            Ok(e) => e,
            Err(FatalError) => panic!(FatalError)
        }
    })
}

pub mod abi          { pub use _syntax::abi::*;          }
pub mod ast          { pub use _syntax::ast::*;          }
pub mod ast_util     { pub use _syntax::ast_util::*;     }
pub mod attr         { pub use _syntax::attr::*;         }
pub mod codemap      { pub use _syntax::codemap::*;      }
pub mod config       { pub use _syntax::config::*;       }
pub mod diagnostic   { pub use _syntax::diagnostic::*;   }
pub mod diagnostics  { pub use _syntax::diagnostics::*;  }
pub mod feature_gate { pub use _syntax::feature_gate::*; }
pub mod fold         { pub use _syntax::fold::*;         }
pub mod owned_slice  { pub use _syntax::owned_slice::*;  }
pub mod ptr          { pub use _syntax::ptr::*;          }
pub mod show_span    { pub use _syntax::show_span::*;    }
pub mod std_inject   { pub use _syntax::std_inject::*;   }
pub mod test         { pub use _syntax::test::*;         }
pub mod util         { pub use _syntax::util::*;         }
pub mod visit        { pub use _syntax::visit::*;        }

pub mod syntax {
    pub use ::{ext, parse, ast};
}

pub mod parse {
    pub use _syntax::parse::{
        PResult, parser, lexer, token, attr, common, classify, obsolete, ParseSess,
        parse_crate_from_file,
        parse_crate_attrs_from_file, parse_crate_from_source_str,
        parse_crate_attrs_from_source_str, parse_expr_from_source_str,
        parse_item_from_source_str, parse_meta_from_source_str,
        parse_stmt_from_source_str, parse_tts_from_source_str, new_parser_from_source_str,
        new_parser_from_file, new_sub_parser_from_file, filemap_to_parser,
        new_parser_from_tts, filemap_to_tts,
        tts_to_parser, maybe_aborted, char_lit, str_lit, raw_str_lit,
        float_lit, byte_lit, integer_lit,
    };
}

pub mod print {
    pub use _syntax::print::{
        pp, pprust
    };
}

pub mod ext {
    pub use _syntax::ext::{
        asm, base, build, cfg, concat, concat_idents, env, expand,
        log_syntax, mtwt, quote, source_util, trace_macros, tt, deriving,
    };
}

//////////////////////////////////////////////

use ext::base::{MultiDecorator};
use parse::{token};
use rustc::plugin::{Registry};

mod lrs_ext;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        token::intern("derive_onlyCopy"),
        MultiDecorator(Box::new(lrs_ext::derive_copy)));

    reg.register_syntax_extension(
        token::intern("derive_Clone"),
        MultiDecorator(Box::new(lrs_ext::derive_clone)));

    reg.register_syntax_extension(
        token::intern("derive_MaybeClone"),
        MultiDecorator(Box::new(lrs_ext::derive_maybe_clone)));

    reg.register_syntax_extension(
        token::intern("derive_Copy"),
        MultiDecorator(Box::new(lrs_ext::derive_copy_and_clone)));

    reg.register_syntax_extension(
        token::intern("derive_Pod"),
        MultiDecorator(Box::new(lrs_ext::derive_pod_copy_and_clone)));

    reg.register_syntax_extension(
        token::intern("derive_Eq"),
        MultiDecorator(Box::new(lrs_ext::derive_eq)));

    reg.register_syntax_extension(
        token::intern("derive_Debug"),
        MultiDecorator(Box::new(lrs_ext::derive_debug)));

    reg.register_macro(
        "write",
        lrs_ext::expand_format_args);

    reg.register_macro(
        "derive_Clone_for",
        lrs_ext::derive_copy_clone_for);
}
