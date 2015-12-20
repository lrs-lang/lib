use syntax::ast;
use syntax::ast::{MetaItem, Expr,};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax::ptr::P;

use syntax_ext::deriving::generic::*;
use syntax_ext::deriving::generic::ty::*;

pub fn derive_debug(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Annotatable,
                    push: &mut FnMut(Annotatable)) {
    // &mut ::std::io::Write
    let fmtr = Ptr(Box::new(Literal(path_std!(cx, lrd::io::Write))),
                   Borrowed(None, ast::MutMutable));

    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path_std!(cx, std::fmt::Debug),
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        is_unsafe: false,
        methods: vec![
            MethodDef {
                name: "fmt",
                generics: LifetimeBounds::empty(),
                explicit_self: borrowed_explicit_self(),
                args: vec!(fmtr),
                ret_ty: Literal(path_std!(cx, std::result::Result)),
                attributes: Vec::new(),
                is_unsafe: false,
                combine_substructure: combine_substructure(Box::new(|a, b, c| {
                    debug_substructure(a, b, c)
                }))
            }
        ],
        associated_types: Vec::new(),
    };
    trait_def.expand(cx, mitem, item, push)
}

/// We use the debug builders to do the heavy lifting here
fn debug_substructure(cx: &mut ExtCtxt, span: Span,
                      substr: &Substructure) -> P<Expr> {
    // build fmt.debug_struct(<name>).field(<fieldname>, &<fieldval>)....build()
    // or fmt.debug_tuple(<name>).field(&<fieldval>)....build()
    // based on the "shape".
    let ident = match *substr.fields {
        Struct(_) => substr.type_ident,
        EnumMatching(_, v, _) => v.node.name,
        EnumNonMatchingCollapsed(..) | StaticStruct(..) | StaticEnum(..) => {
            cx.span_bug(span, "nonsensical .fields in `#[derive(Debug)]`")
        }
    };

    // We want to make sure we have the expn_id set so that we can use unstable methods
    let span = Span { expn_id: cx.backtrace(), .. span };
    let name = cx.expr_lit(span, ast::Lit_::LitStr(ident.name.as_str(),
                                                   ast::StrStyle::CookedStr));
    let mut expr = substr.nonself_args[0].clone();

    match *substr.fields {
        Struct(ref fields) | EnumMatching(_, _, ref fields) => {

            if fields.is_empty() || fields[0].name.is_none() {
                // tuple struct/"normal" variant
                expr = cx.expr_method_call(span,
                                           expr,
                                           token::str_to_ident("debug_tuple"),
                                           vec![name]);

                for field in fields {
                    // Use double indirection to make sure this works for unsized types
                    let field = cx.expr_addr_of(field.span, field.self_.clone());
                    let field = cx.expr_addr_of(field.span, field);

                    expr = cx.expr_method_call(span,
                                               expr,
                                               token::str_to_ident("field"),
                                               vec![field]);
                }
            } else {
                // normal struct/struct variant
                expr = cx.expr_method_call(span,
                                           expr,
                                           token::str_to_ident("debug_struct"),
                                           vec![name]);

                for field in fields {
                    let name = cx.expr_lit(field.span, ast::Lit_::LitStr(
                            field.name.as_ref().unwrap().name.as_str(),
                            ast::StrStyle::CookedStr));

                    // Use double indirection to make sure this works for unsized types
                    let field = cx.expr_addr_of(field.span, field.self_.clone());
                    let field = cx.expr_addr_of(field.span, field);
                    expr = cx.expr_method_call(span,
                                               expr,
                                               token::str_to_ident("field"),
                                               vec![name, field]);
                }
            }
        }
        _ => unreachable!()
    }

    cx.expr_method_call(span,
                        expr,
                        token::str_to_ident("finish"),
                        vec![])
}
