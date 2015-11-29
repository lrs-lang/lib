// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::cmp::{Ordering};

use {Node, Entree, get_owner, get_node};
use path::{Path};

/// Inserts an element into a tree.
///
/// [argument, root]
/// The root of the tree.
///
/// [argument, new_val]
/// The element to insert into the tree.
///
/// [return_value]
/// Returns the new root of the tree.
pub fn insert<'a, E>(root: &'a Node, new_val: &'a E::Owner) -> &'a Node
    where E: Entree,
{
    let mut path = unsafe { Path::new() };
    path.reset();

    let mut cur_opt = Some(root);

    while let Some(cur) = cur_opt {
        let cur_el = get_owner::<E>(cur);
        let ord = E::cmp(cur_el, new_val);

        unsafe { path.push(cur, ord); }

        cur_opt = match ord {
            Ordering::Equal => abort!(),
            Ordering::Less => cur.right(),
            Ordering::Greater => cur.left(),
        }
    }

    fix_insert(root, get_node::<E>(new_val), &mut path)
}

/// Fixes a tree by removing right-leaning three-nodes and four-nodes.
///
/// [argument, root]
/// The root of the tree.
///
/// [argument, next]
/// The next value to append to the path.
///
/// [argument, path]
/// The path that was taken to the next value.
///
/// [return_value]
/// Returns the new root of the tree.
fn fix_insert<'a>(root: &'a Node, mut next: &'a Node,
                  path: &mut Path<'a, Ordering>) -> &'a Node {
    while let Some((cur, ord)) = path.pop() {
        if ord == Ordering::Less {
            cur.right.set(next);

            let right = next;
            if !right.is_red.get() {
                return root;
            }

            let left = unsafe { cur.unchecked_left() };

            if left.is_red.get() {
                // Case:
                //
                //                  +---+
                //                  |cur|   <-- black
                //                  +---+
                //                 /     \
                //                /       \
                //             +---+     +---+
                //   red -->   |lft|     |nxt|   <-- red
                //             +---+     +---+
                //
                // Solution:
                //
                //                  +---+
                //                  |cur|   <-- red
                //                  +---+
                //                 /     \
                //                /       \
                //             +---+     +---+
                // black -->   |lft|     |nxt|   <-- black
                //             +---+     +---+
                //
                cur.is_red.set(true);
                left.is_red.set(false);
                right.is_red.set(false);

                next = cur;
                continue;
            }

            // Case:
            //
            //                  +---+
            //                  |cur|   <-- black or red
            //                  +---+
            //                 /     \
            //                /       \
            //             +---+     +---+
            // black -->   |lft|     |nxt|   <-- red
            //             +---+     +---+
            //
            // Solution: Rotate and let the parent handle the rest.
            //
            //                       +---+
            //                       |nxt|   <-- black or red
            //                       +---+
            //                      /
            //                     /
            //                  +---+
            //        red -->   |cur|
            //                  +---+
            //                 /
            //                /
            //             +---+
            // black -->   |lft|
            //             +---+
            //
            cur.right.set(right.left.get());
            right.left.set(cur);

            right.is_red.set(cur.is_red.get());
            cur.is_red.set(true);

            next = right;
            continue;
        }

        cur.left.set(next);

        let left = next;
        if !left.is_red.get() {
            return root;
        }

        if cur.is_red.get() {
            // Case:
            //
            //                  +---+
            //                  |cur|   <-- red
            //                  +---+
            //                 /
            //                /
            //             +---+
            //             |nxt|   <-- red
            //             +---+
            //
            // Solution: Let the parent handle it.
            next = cur;
            continue;
        }

        if let Some(left_left) = left.left() {
            if left_left.is_red.get() {
                // Case:
                //
                //                  +---+
                //                  |cur|   <-- black
                //                  +---+
                //                 /
                //                /
                //             +---+
                //             |nxt|   <-- red
                //             +---+
                //            /
                //           /
                //        +---+
                //        |l_l|   <-- red
                //        +---+
                //
                // Solution:
                //
                //                  +---+
                //                  |nxt|   <-- red
                //                  +---+
                //                 /     \
                //                /       \
                //             +---+     +---+
                // black -->   |l_l|     |cur|   <-- black
                //             +---+     +---+
                //
                cur.left.set(left.right.get());
                left.right.set(cur);
                left_left.is_red.set(false);
                next = left;
                continue;
            }
        }

        return root;
    }

    next
}
