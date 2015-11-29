// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod down;
mod up;

use core::cmp::{Ordering};
use core::{mem};
use path::{Path};
use {Entree, Node, get_owner, __LEAF};

/// Removes the smallest element in the tree.
///
/// [argument, node]
/// The node whose smallest descendant to remove.
///
/// [return_value]
/// Returns the smallest node and the new root.
///
/// = Remarks
///
/// Either the node or its left child must be red.
pub fn remove_min<'a>(node: &'a Node) -> (&'a Node, &'a Node) {
    let mut path = unsafe { Path::new() };
    path.reset();

    let mut cur = node;

    loop {
        let left = match cur.left() {
            Some(l) => l,
            _ => break,
        };

        cur = down::left(cur, left, |p| unsafe { path.push(p, ()); }, true);
    }

    let min = cur;

    let mut next = &__LEAF.node;

    while let Some((cur, _)) = path.pop() {
        cur.left.set(next);

        next = up::clean(cur);
    }

    (min, next)
}

/// Removes an element from a tree.
///
/// [argument, node]
/// The node among whose descendants the element can be found.
///
/// [return_value]
/// Returns the new root and the node of the removed element, if any.
///
/// = Remarks
///
/// Either the node or its left child must be red.
pub fn remove<'a, E>(node: &'a Node, val: &E::Owner) -> (&'a Node, Option<&'a Node>)
    where E: Entree,
{
    let mut path = unsafe { Path::new() };
    path.reset();

    // We use the following trick in the iteration in which we find the node: We let it
    // continue one more interation with `cmp = Ordering::Less`. This sets `cur` to the
    // note to our right (if any) and since the property above is preserved, we can pass
    // `cur` to `remove_min`.

    let mut cur = node;
    let mut found = None;

    while found.is_none() {
        let cur_val = get_owner::<E>(cur);
        let cmp = match E::cmp(cur_val, val) {
            Ordering::Equal => {
                found = Some(cur);
                Ordering::Less
            },
            o => o,
        };

        cur = match cmp {
            Ordering::Equal => { abort!() },
            Ordering::Greater => {
                let left = match cur.left() {
                    Some(l) => l,
                    _ => break,
                };

                down::left(cur, left, |p| unsafe { path.push(p, cmp); }, false);
                left
            },
            Ordering::Less => {
                let right = match cur.right() {
                    Some(r) => r,
                    _ => break,
                };

                down::right(cur, right, |p| unsafe { path.push(p, cmp); });
                right
            },
        };
    }

    let mut next = if let Some(found) = found {
        if mem::addr(found) != mem::addr(cur) {
            // Case: `found` has a right child.
            //
            //         +---+
            //         |fnd|
            //         +---+
            //              \
            //               \
            //              +---+
            //              |cur|
            //              +---+
            //
            // Where either `cur` or `cur.left` is red.
            //
            let (min, right) = remove_min(cur);
            min.left.set(found.left.get());
            min.is_red.set(found.is_red.get());

            // Swap `found` and `min` in the path.
            path.pop().unwrap();
            unsafe { path.push(min, Ordering::Less); }

            right
        } else if let Some(left) = found.left() {
            // Case: `found` has a left child.
            //
            //                +---+
            //                |fnd|   <-- black
            //                +---+
            //               /     \
            //              /       \
            //           +---+    no child
            // red -->   |lft|
            //           +---+
            //
            // Solution:
            //
            //               +---+
            //               |lft|   <-- black
            //               +---+
            //
            left.is_red.set(false);
            left
        } else {
            // Case: `found` has no children.
            //
            //                +---+
            //                |fnd|   <-- red
            //                +---+
            //               /     \
            //              /       \
            //             no children
            //
            &__LEAF.node
        }
    } else {
        // Case: Nothing found, nothing removed.
        cur
    };

    while let Some((cur, ord)) = path.pop() {
        if ord == Ordering::Greater {
            cur.left.set(next);
        } else {
            cur.right.set(next);
        }

        next = up::clean(cur);
    }

    (next, found)
}
