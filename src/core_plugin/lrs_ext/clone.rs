use ast::{Generics, MetaItem, Item, Expr};

use codemap::{Span};

use owned_slice::{OwnedSlice};

use ext::base::{ExtCtxt};
use ext::deriving::generic::{
    TraitDef, MethodDef, Substructure, combine_substructure, FieldInfo, Struct,
    EnumMatching, EnumNonMatchingCollapsed, StaticEnum, StaticStruct,
};
use ext::deriving::generic::ty::{
    LifetimeBounds, borrowed_explicit_self, Self_,
};
use ext::build::{AstBuilder};

use ptr::{P};

use parse::token::{InternedString};

fn cs_clone(
    name: &str,
    cx: &mut ExtCtxt, trait_span: Span,
    substr: &Substructure) -> P<Expr> {
    let ctor_path;
    let all_fields;
    let fn_path = vec![
        cx.ident_of_std("core"),
        cx.ident_of("clone"),
        cx.ident_of("Clone"),
        cx.ident_of("clone"),
    ];
    let subcall = |field: &FieldInfo| {
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

pub fn derive_clone(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Item,
                        push: &mut FnMut(P<Item>)) {
    let inline = cx.meta_word(span, InternedString::new("inline"));
    let attrs = vec!(cx.attribute(span, inline));
    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path_std!(cx, core::clone::Clone),
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        methods: vec!(
            MethodDef {
                name: "clone",
                generics: LifetimeBounds::empty(),
                explicit_self: borrowed_explicit_self(),
                args: Vec::new(),
                ret_ty: Self_,
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

    let copy_path = path_std!(cx, core::marker::Copy)
                        .to_path(cx, span, item.ident, generics);

    // Create generics with additional Copy bound

    let Generics { lifetimes, ty_params, where_clause } = generics.clone();
    let mut ty_params = ty_params.into_vec();

    for ty_param in &mut ty_params {
        let mut bounds = vec!();
        bounds.push(cx.typarambound(copy_path.clone()));
        for declared_bound in &*ty_param.bounds {
            bounds.push((*declared_bound).clone());
        }
        ty_param.bounds = OwnedSlice::from_vec(bounds);
    }

    let generics_with_bounds = Generics {
        lifetimes: lifetimes,
        ty_params: OwnedSlice::from_vec(ty_params),
        where_clause: where_clause,
    };

    // Create generics without bounds

    let Generics { lifetimes, ty_params, where_clause } = generics.clone();
    let mut ty_params = ty_params.into_vec();

    for ty_param in &mut ty_params {
        ty_param.bounds = OwnedSlice::from_vec(vec!());
    }

    let generics_without_bounds = Generics { 
        lifetimes: lifetimes,
        ty_params: OwnedSlice::from_vec(ty_params),
        where_clause: where_clause,
    };

    let where_clause = &generics.where_clause;
    let ty = item.ident;

    let impl_item = quote_item!(cx,
        #[automatically_derived]
        impl $generics_with_bounds ::core::clone::Clone for $ty $generics_without_bounds $where_clause {
            fn clone(&self) -> $ty $generics_without_bounds { *self }
        }
    ).unwrap();

    push(impl_item)
}
