use ast::{MetaItem, Item};

use codemap::{Span};

use ext::base::{ExtCtxt};
use ext::deriving::generic::{TraitDef};
use ext::deriving::generic::ty::{Path, LifetimeBounds};

use ptr::{P};

pub fn derive_copy(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Item,
                       push: &mut FnMut(P<Item>)) {
    let path = Path::new(vec!("core", "marker", "Copy"));

    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path,
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        methods: Vec::new(),
        associated_types: Vec::new(),
    };

    trait_def.expand(cx, mitem, item, push)
}

pub fn derive_copy_and_clone(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem,
                                 item: &Item, push: &mut FnMut(P<Item>)) {
    derive_copy(cx, span, mitem, item, push);
    super::clone::derive_clone_for_copy(cx, span, mitem, item, push);
}
