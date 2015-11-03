use ast::{MetaItem};

use codemap::{Span};

use ext::base::{ExtCtxt, Annotatable};
use ext::deriving::generic::{TraitDef};
use ext::deriving::generic::ty::{Path, LifetimeBounds};

pub fn derive_marker(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Annotatable,
                     push: &mut FnMut(Annotatable), name: &str) {
    let path = Path::new(vec!("std", "marker", name));

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

pub fn derive_copy(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Annotatable,
                   push: &mut FnMut(Annotatable)) {
    derive_marker(cx, span, mitem, item, push, "Copy");
}

pub fn derive_copy_and_clone(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem,
                             item: &Annotatable, push: &mut FnMut(Annotatable)) {
    derive_copy(cx, span, mitem, item, push);
    super::clone::derive_clone_for_copy(cx, span, mitem, item, push);
}

pub fn derive_pod_copy_and_clone(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem,
                                 item: &Annotatable, push: &mut FnMut(Annotatable)) {
    derive_marker(cx, span, mitem, item, push, "Pod");
    derive_copy_and_clone(cx, span, mitem, item, push);
}
