// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {get_owner, Node, Entree};
use base::prelude::*;
use fmt::{Write, Debug};

pub fn debug<E: Entree, W: Write>(mut w: &mut W, node: &Node) -> Result
    where E::Owner: Debug,
{
    try!(write!(w, "digraph tree {{\n"));
    let mut n = 0;
    try!(debug_::<E, W>(w, node, &mut n));
    write!(w, "}}\n")
}

fn debug_<E: Entree, W: Write>(mut w: &mut W, node: &Node, id: &mut usize) -> Result
    where E::Owner: Debug,
{
    let owner = get_owner::<E>(node);
    try!(write!(w, "n{} [label=\"{:?}\"", *id, owner));

    if node.is_red.get() {
        try!(write!(w, ", style=filled, fillcolor=red"));
    }
    try!(write!(w, "];\n"));

    let my_id = *id;

    if let Some(left) = node.left() {
        *id += 1;
        try!(write!(w, "n{} -> n{};\n", my_id, *id));
        try!(debug_::<E, W>(w, left, id));
    }

    if let Some(right) = node.right() {
        *id += 1;
        try!(write!(w, "n{} -> n{};\n", my_id, *id));
        try!(debug_::<E, W>(w, right, id));
    }

    Ok(())
}
