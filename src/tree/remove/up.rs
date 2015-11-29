// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {Node};

/// Cleans a node up after a delete operation.
///
/// [argument, cur]
/// The current node.
///
/// [return_value]
/// Returns the new top node after possible rotations.
///
/// = Remarks
///
/// This function removes four-nodes and right-leaning three-nodes.
pub fn clean(cur: &Node) -> &Node {
    let (left, right) = unsafe { (cur.unchecked_left(), cur.unchecked_right()) };

    if !right.is_red.get() {
        return cur;
    }

    if left.is_red.get() {
        // Case:
        //
        //                         +---+
        //                         |cur|   <-- black
        //                         +---+
        //                        /     \
        //                       /       \
        //                    +---+     +---+
        //          red -->   |lft|     |rgt|   <-- red
        //                    +---+     +---+
        //
        // Solution:
        //
        //                         +---+
        //                         |cur|   <-- red
        //                         +---+
        //                        /     \
        //                       /       \
        //                    +---+     +---+
        //        black -->   |lft|     |rgt|   <-- black
        //                    +---+     +---+
        //
        cur.is_red.set(true);
        left.is_red.set(false);
        right.is_red.set(false);
        cur
    } else {
        // I think this can't happen.
        assert!(!cur.is_red.get());

        // Case:
        //
        //                         +---+
        //                         |cur|   <-- black
        //                         +---+
        //                        /     \
        //                       /       \
        //                    +---+     +---+
        //        black -->   |lft|     |rgt|   <-- red
        //                    +---+     +---+
        //
        // Solution:
        //
        //                         +---+
        //                         |rgt|   <-- black
        //                         +---+
        //                        /
        //                       /
        //                    +---+
        //          red -->   |cur|
        //                    +---+
        //                   /
        //                  /
        //               +---+
        //   black -->   |lft|
        //               +---+
        //
        cur.right.set(right.left.get());
        right.left.set(cur);
        cur.is_red.set(true);
        right.is_red.set(false);
        right
    }
}
