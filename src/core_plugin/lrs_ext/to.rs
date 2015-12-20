use syntax::ast::{MetaItem, Expr, ExprRet, TokenTree, LifetimeDef, Ident, Item_};
use syntax::codemap::{Span};
use syntax::util::small_vector::{SmallVector};
use syntax::ext::base::{ExtCtxt, MacEager, MacResult, DummyResult, Annotatable};
use syntax::ext::build::{AstBuilder};
use syntax::ptr::{P};
use syntax::parse::token::{InternedString, Eof, Token, BinOpToken};

use syntax_ext::deriving::generic::{
    TraitDef, MethodDef, Substructure, combine_substructure, FieldInfo, Struct,
    EnumMatching, EnumNonMatchingCollapsed, StaticEnum, StaticStruct,
};
use syntax_ext::deriving::generic::ty::{
    LifetimeBounds, borrowed_explicit_self, Ty, Path,
};

fn expr_ok(cx: &ExtCtxt, sp: Span, expr: P<Expr>) -> P<Expr> {
    let ok = vec!(
        cx.ident_of("std"),
        cx.ident_of("result"),
        cx.ident_of("Result"),
        cx.ident_of("Ok"));
    cx.expr_call_global(sp, ok, vec!(expr))
}

fn expr_try(cx: &ExtCtxt, sp: Span, head: P<Expr>) -> P<Expr> {
    let ok = vec!(
        cx.ident_of("std"),
        cx.ident_of("result"),
        cx.ident_of("Result"),
        cx.ident_of("Ok")
    );
    let ok_path = cx.path_global(sp, ok);
    let err = vec!(
        cx.ident_of("std"),
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

fn cs_try_to(name: &str, cx: &mut ExtCtxt, trait_span: Span,
             substr: &Substructure) -> P<Expr> {
    let ctor_path;
    let all_fields;

    let fn_path = vec!(
        cx.ident_of("std"),
        cx.ident_of("conv"),
        cx.ident_of("TryTo"),
        cx.ident_of("try_to"),
    );

    let subcall = |field: &FieldInfo| {
        // ::std::conv::TryTo::try_to(&field)
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

pub fn derive_try_to(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem,
                          item: &Annotatable, push: &mut FnMut(Annotatable)) {
    let inline = cx.meta_word(span, InternedString::new("inline"));
    let attrs = vec!(cx.attribute(span, inline));
    let ret_ty = Ty::Literal(Path {
        path: vec!("std", "result", "Result"),
        lifetime: None,
        params: vec!(box Ty::Self_),
        global: true,
    });
    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path!(std::conv::TryTo),
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        is_unsafe: false,
        methods: vec!(
            MethodDef {
                name: "try_to",
                generics: LifetimeBounds::empty(),
                explicit_self: borrowed_explicit_self(),
                args: Vec::new(),
                ret_ty: ret_ty,
                attributes: attrs,
                is_unsafe: false,
                combine_substructure: combine_substructure(Box::new(|c, s, sub| {
                    cs_try_to("TryTo", c, s, sub)
                })),
            }
        ),
        associated_types: Vec::new(),
    };

    trait_def.expand(cx, mitem, item, push)
}

fn cs_to(name: &str, cx: &mut ExtCtxt, trait_span: Span,
         substr: &Substructure) -> P<Expr> {
    let ctor_path;
    let all_fields;

    let fn_path = vec!(
        cx.ident_of("std"),
        cx.ident_of("conv"),
        cx.ident_of("To"),
        cx.ident_of("to"),
    );

    let subcall = |field: &FieldInfo| {
        // ::std::conv::To::to(&field)
        let args = vec![cx.expr_addr_of(field.span, field.self_.clone())];
        cx.expr_call_global(field.span, fn_path.clone(), args)
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
        cx.expr_call(trait_span, path, subcalls)
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

        if fields.is_empty() {
            // no fields, so construct like `None`
            cx.expr_path(ctor_path)
        } else {
            cx.expr_struct(trait_span, ctor_path, fields)
        }
    }
}

pub fn derive_to(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem,
                 item: &Annotatable, push: &mut FnMut(Annotatable)) {
    let inline = cx.meta_word(span, InternedString::new("inline"));
    let attrs = vec!(cx.attribute(span, inline));
    let ret_ty = Ty::Self_;
    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path!(std::conv::To),
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        is_unsafe: false,
        methods: vec!(
            MethodDef {
                name: "to",
                generics: LifetimeBounds::empty(),
                explicit_self: borrowed_explicit_self(),
                args: Vec::new(),
                ret_ty: ret_ty,
                attributes: attrs,
                is_unsafe: false,
                combine_substructure: combine_substructure(Box::new(|c, s, sub| {
                    cs_to("To", c, s, sub)
                })),
            }
        ),
        associated_types: Vec::new(),
    };

    trait_def.expand(cx, mitem, item, push);
}

pub fn derive_to_for_copy(cx: &mut ExtCtxt, span: Span, _mitem: &MetaItem,
                             item: &Annotatable, push: &mut FnMut(Annotatable)) {
    let item = match *item {
        Annotatable::Item(ref i) => i,
        _ => cx.bug("expected ItemStruct or ItemEnum in #[derive(Copy)]"),
    };

    let generics = match item.node {
        Item_::ItemStruct(_, ref generics) => generics,
        Item_::ItemEnum(_, ref generics) => generics,
        _ => cx.bug("expected ItemStruct or ItemEnum in #[derive(Copy)]"),
    };

    // generics doesn't implement ToTokens anymore so we'll just use this ugly thing:
    struct Lts<'a>(&'a [LifetimeDef], Span);

    impl<'a> Lts<'a> {
        fn to_tokens(&self, _cx: &ExtCtxt) -> Vec<TokenTree> {
            let mut vec = Vec::new();
            vec.push(TokenTree::Token(self.1, Token::Lt));
            for lt in self.0 {
                vec.push(TokenTree::Token(self.1, Token::Lifetime(Ident::with_empty_ctxt(lt.lifetime.name))));
                if lt.bounds.len() > 0 {
                    vec.push(TokenTree::Token(self.1, Token::Colon));
                    vec.push(TokenTree::Token(self.1, Token::Lifetime(Ident::with_empty_ctxt(lt.bounds[0].name))));
                    for lt in &lt.bounds[1..] {
                        vec.push(TokenTree::Token(self.1, Token::BinOp(BinOpToken::Plus)));
                        vec.push(TokenTree::Token(self.1, Token::Lifetime(Ident::with_empty_ctxt(lt.name))));
                    }
                }
                vec.push(TokenTree::Token(self.1, Token::Comma));
            }
            vec.push(TokenTree::Token(self.1, Token::Gt));
            vec
        }
    }

    if generics.ty_params.len() == 0 {
        let ty = item.ident;
        let lts = Lts(&generics.lifetimes, span);

        let impl_item = quote_item!(cx,
            impl $lts ::std::conv::To for $ty $lts {
                fn to(&self) -> $ty $lts {
                    *self
                }
            }
        ).unwrap();

        push(Annotatable::Item(impl_item));

        let impl_item = quote_item!(cx,
            impl $lts ::std::conv::TryTo for $ty $lts {
                fn try_to(&self) -> ::std::result::Result<$ty $lts> {
                    ::std::result::Result::Ok(*self)
                }
            }
        ).unwrap();

        push(Annotatable::Item(impl_item));
    }
}

pub fn derive_copy_to_for<'cx>(cx: &'cx mut ExtCtxt, sp: Span,
                               tts: &[TokenTree]) -> Box<MacResult+'cx> {
    let mut p = cx.new_parser_from_tts(tts);
    if p.token == Eof {
        cx.span_err(sp, "requires a target");
        return DummyResult::expr(sp);
    }
    let dest = match p.parse_expr() {
        Ok(d) => d,
        _ => {
            cx.span_err(sp, "could not parse");
            return DummyResult::expr(sp);
        },
    };
    let item = quote_item!(cx,
        #[automatically_derived]
        #[inline(always)]
        impl ::std::conv::To for $dest {
            fn to(&self) -> $dest {
                *self
            }
        }
    ).unwrap();
    MacEager::items(SmallVector::one(item))
}
