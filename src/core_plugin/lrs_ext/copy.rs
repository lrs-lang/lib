use std::cell::{RefCell};

use syntax::ast::{self, MetaItem, VariantData};
use syntax::codemap::{Span};
use syntax::ext::base::{ExtCtxt, Annotatable};

use syntax_ext::deriving::generic::{TraitDef, MethodDef};
use syntax_ext::deriving::generic::ty::{Path, LifetimeBounds, Ty};

pub fn derive_copy(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Annotatable,
                   push: &mut FnMut(Annotatable)) {
    let path = Path::new(vec!("std", "marker", "Copy"));

    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path,
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        is_unsafe: false,
        methods: Vec::new(),
        associated_types: Vec::new(),
    };

    trait_def.expand(cx, mitem, item, push)
}

pub fn derive_copy_and_to(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem,
                          item: &Annotatable, push: &mut FnMut(Annotatable)) {
    derive_copy(cx, span, mitem, item, push);
    super::to::derive_to_for_copy(cx, span, mitem, item, push);
}

pub fn derive_pod_copy_and_to(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem,
                              anno: &Annotatable, push: &mut FnMut(Annotatable)) {
    let item = match *anno {
        Annotatable::Item(ref item) => item,
        _ => {
            cx.span_err(mitem.span, "`Pod` types must be structs");
            return
        },
    };

    let assertions = match item.node {
        ast::ItemStruct(VariantData::Struct(ref fields, _), _) |
            ast::ItemStruct(VariantData::Tuple(ref fields, _), _) =>
        {
            fields.iter().map(|field| {
                let ty = &field.node.ty;
                quote_stmt!(cx, assert::<$ty>();)
            }).collect::<Vec<_>>()
        },
        _ => {
            cx.span_err(mitem.span, "`Pod` types must be structs");
            return
        },
    };

    let assert = MethodDef {
        name: "_assert_pod",
        generics: LifetimeBounds::empty(),
        explicit_self: None,
        args: Vec::new(),
        ret_ty: Ty::Tuple(Vec::new()),
        attributes: Vec::new(),
        is_unsafe: false,
        combine_substructure: RefCell::new(box move |cx: &mut ExtCtxt, _, _| {
            quote_expr!(cx, {
                fn assert<T: ::std::marker::Pod>() { }
                $assertions
            })
        }),
    };

    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: Path::new(vec!("std", "marker", "Pod")),
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        is_unsafe: true,
        methods: vec!(assert),
        associated_types: Vec::new(),
    };

    trait_def.expand(cx, mitem, anno, push);

    derive_copy_and_to(cx, span, mitem, anno, push);
}
