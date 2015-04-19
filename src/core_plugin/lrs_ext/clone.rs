use ast::{Generics, MetaItem, Item, Expr, ExprRet, TokenTree};

use codemap::{Span};

use util::small_vector::{SmallVector};

use owned_slice::{OwnedSlice};

use ext::base::{ExtCtxt, MacEager, MacResult, DummyResult};
use ext::deriving::generic::{
    TraitDef, MethodDef, Substructure, combine_substructure, FieldInfo, Struct,
    EnumMatching, EnumNonMatchingCollapsed, StaticEnum, StaticStruct,
};
use ext::deriving::generic::ty::{
    LifetimeBounds, borrowed_explicit_self, Ty, Path,
};
use ext::build::{AstBuilder};

use ptr::{P};

use parse::token::{InternedString, Eof};

fn expr_ok(cx: &ExtCtxt, sp: Span, expr: P<Expr>) -> P<Expr> {
    let ok = vec!(
        cx.ident_of("linux"),
        cx.ident_of("result"),
        cx.ident_of("Result"),
        cx.ident_of("Ok"));
    cx.expr_call_global(sp, ok, vec!(expr))
}

fn expr_try(cx: &ExtCtxt, sp: Span, head: P<Expr>) -> P<Expr> {
    let ok = vec!(
        cx.ident_of("linux"),
        cx.ident_of("result"),
        cx.ident_of("Result"),
        cx.ident_of("Ok")
    );
    let ok_path = cx.path_global(sp, ok);
    let err = vec!(
        cx.ident_of("linux"),
        cx.ident_of("result"),
        cx.ident_of("Result"),
        cx.ident_of("Err")
    );
    let err_path = cx.path_global(sp, err);

    let binding_variable = cx.ident_of("__try_var");
    let binding_pat = cx.pat_ident(sp, binding_variable);
    let binding_expr = cx.expr_ident(sp, binding_variable);

    // Ok(__try_var) pattern
    let ok_pat = cx.pat_enum(sp, ok_path, vec!(binding_pat.clone()));

    // Err(__try_var)  (pattern and expression resp.)
    let err_pat = cx.pat_enum(sp, err_path.clone(), vec!(binding_pat));
    let err_inner_expr = cx.expr_call(sp, cx.expr_path(err_path),
                                        vec!(binding_expr.clone()));
    // return Err(__try_var)
    let err_expr = cx.expr(sp, ExprRet(Some(err_inner_expr)));

    // Ok(__try_var) => __try_var
    let ok_arm = cx.arm(sp, vec!(ok_pat), binding_expr);
    // Err(__try_var) => return Err(__try_var)
    let err_arm = cx.arm(sp, vec!(err_pat), err_expr);

    // match head { Ok() => ..., Err() => ... }
    cx.expr_match(sp, head, vec!(ok_arm, err_arm))
}

fn cs_clone(name: &str, cx: &mut ExtCtxt, trait_span: Span,
            substr: &Substructure) -> P<Expr> {
    let ctor_path;
    let all_fields;

    let fn_path = vec!(
        cx.ident_of("linux"),
        cx.ident_of("clone"),
        cx.ident_of("Clone"),
        cx.ident_of("clone"),
    );

    let subcall = |field: &FieldInfo| {
        // ::linux::clone::Clone::clone(&field)
        let call = {
            let args = vec![cx.expr_addr_of(field.span, field.self_.clone())];
            cx.expr_call_global(field.span, fn_path.clone(), args)
        };

        // try!(...)
        expr_try(cx, trait_span, call)
    };

    match *substr.fields {
        Struct(ref af) => {
            ctor_path = cx.path(trait_span, vec![substr.type_ident]);
            all_fields = af;
        }
        EnumMatching(_, variant, ref af) => {
            ctor_path = cx.path(trait_span, vec![substr.type_ident, variant.node.name]);
            all_fields = af;
        },
        EnumNonMatchingCollapsed (..) => {
            cx.span_bug(trait_span,
                        &format!("non-matching enum variants in \
                                 `derive({})`", name))
        }
        StaticEnum(..) | StaticStruct(..) => {
            cx.span_bug(trait_span,
                        &format!("static method in `derive({})`", name))
        }
    }

    if all_fields.len() >= 1 && all_fields[0].name.is_none() {
        // enum-like
        let subcalls = all_fields.iter().map(subcall).collect();
        let path = cx.expr_path(ctor_path);
        let body = cx.expr_call(trait_span, path, subcalls);
        expr_ok(cx, trait_span, body)
    } else {
        // struct-like
        let fields = all_fields.iter().map(|field| {
            let ident = match field.name {
                Some(i) => i,
                None => {
                    cx.span_bug(trait_span,
                                &format!("unnamed field in normal struct in \
                                         `derive({})`", name))
                }
            };
            cx.field_imm(field.span, ident, subcall(field))
        }).collect::<Vec<_>>();

        let body = if fields.is_empty() {
            // no fields, so construct like `None`
            cx.expr_path(ctor_path)
        } else {
            cx.expr_struct(trait_span, ctor_path, fields)
        };
        expr_ok(cx, trait_span, body)
    }
}

pub fn derive_clone(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Item,
                        push: &mut FnMut(P<Item>)) {
    let inline = cx.meta_word(span, InternedString::new("inline"));
    let attrs = vec!(cx.attribute(span, inline));
    let ret_ty = Ty::Literal(Path {
        path: vec!("linux", "result", "Result"),
        lifetime: None,
        params: vec!(box Ty::Self_),
        global: true,
    });
    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path_std!(cx, linux::clone::Clone),
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        methods: vec!(
            MethodDef {
                name: "clone",
                generics: LifetimeBounds::empty(),
                explicit_self: borrowed_explicit_self(),
                args: Vec::new(),
                ret_ty: ret_ty,
                attributes: attrs,
                combine_substructure: combine_substructure(Box::new(|c, s, sub| {
                    cs_clone("Clone", c, s, sub)
                })),
            }
        ),
        associated_types: Vec::new(),
    };

    trait_def.expand(cx, mitem, item, push)
}

pub fn derive_clone_for_copy(cx: &mut ExtCtxt, span: Span, _mitem: &MetaItem,
                                 item: &Item, push: &mut FnMut(P<Item>)) {
    let generics = match item.node {
        ::ast::ItemStruct(_, ref generics) => generics,
        ::ast::ItemEnum(_, ref generics) => generics,
        _ => cx.bug("expected ItemStruct or ItemEnum in #[derive(Copy)]")
    };

    if generics.ty_params.len() == 0 {
        let ty = item.ident;

        let impl_item = quote_item!(cx,
            #[automatically_derived]
            #[inline(always)]
            impl $generics ::linux::clone::Clone for $ty $generics {
                fn clone(&self) -> ::linux::result::Result<$ty $generics> {
                    ::linux::result::Result::Ok(*self)
                }
            }
        ).unwrap();

        push(impl_item)
    }
}

pub fn derive_copy_clone_for<'cx>(cx: &'cx mut ExtCtxt, sp: Span,
                                  tts: &[TokenTree]) -> Box<MacResult+'cx> {

    let mut p = cx.new_parser_from_tts(tts);
    if p.token == Eof {
        cx.span_err(sp, "requires a target");
        return DummyResult::expr(sp);
    }
    let dest = p.parse_expr();
    let item = quote_item!(cx,
        #[automatically_derived]
        #[inline(always)]
        impl ::linux::clone::Clone for $dest {
            fn clone(&self) -> ::linux::result::Result<$dest> {
                ::linux::result::Result::Ok(*self)
            }
        }
    ).unwrap();
    MacEager::items(SmallVector::one(item))
}
