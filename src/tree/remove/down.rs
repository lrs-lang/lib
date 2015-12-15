// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {Node};

/// Input: Either `cur` or `cur.left` is red.
///
/// Output: Either `left` or `left.left` is red.
///
/// Pushes the path to `left` (starting at `cur`, excluding `left`).
///
/// Returns `left` if `left` is red, `left.left` otherwise. If `push2` is true and `left`
/// is not red, then `left` is also pushed.
pub fn left<'a, F>(cur: &'a Node, left: &'a Node, mut push: F, push2: bool) -> &'a Node
    where F: FnMut(&'a Node),
{
    if left.is_red.get() {
        // Case 1:
        //
        //                         +---+
        //                         |cur|   <-- black
        //                         +---+
        //                        /
        //                       /
        //                    +---+
        //          red -->   |lft|
        //                    +---+
        //
        push(cur);
        return left;
    }

    // Since `left` is not red, `cur` is red.

    if let Some(left_left) = left.left() {
        if left_left.is_red.get() {
            // Case 2:
            //
            //                         +---+
            //                         |cur|   <-- red
            //                         +---+
            //                        /
            //                       /
            //                    +---+
            //        black -->   |lft|
            //                    +---+
            //                   /
            //                  /
            //               +---+
            //     red -->   |l_l|
            //               +---+
            //
            push(cur);
            if push2 {
                push(left);
            }
            return left_left;
        }
    }

    // Since `left` is black, `right` exists and is black.
    let right = unsafe { cur.unchecked_right() };

    if let Some(right_left) = right.left() {
        if right_left.is_red.get() {
            // Case 3:
            //
            //                         +---+
            //                         |cur|   <-- red
            //                         +---+
            //                        /     \
            //                       /       \
            //                    +---+     +---+
            //        black -->   |lft|     |rgt|   <-- black
            //                    +---+     +---+
            //                   /         /
            //                  /         /
            //               +---+     +---+
            //   black -->   |l_l|     |r_l|   <-- red
            //               +---+     +---+
            //                        /
            //                       /
            //                    +---+
            //                    |rll|   <-- black
            //                    +---+
            //
            // Solution:
            //
            //                              +---+
            //                              |r_l|   <-- red
            //                              +---+
            //                             /     \
            //                            /       \
            //                         +---+     +---+
            //             black -->   |cur|     |rgt|   <-- black
            //                         +---+     +---+
            //                        /     \
            //                       /       \
            //                    +---+     +---+
            //          red -->   |lft|     |rll|   <-- black
            //                    +---+     +---+
            //                   /
            //                  /
            //               +---+
            //   black -->   |l_l|
            //               +---+
            //
            right.left.set(right_left.right.get());
            right_left.right.set(right);
            cur.right.set(right_left.left.get());
            right_left.left.set(cur);

            cur.is_red.set(false);
            left.is_red.set(true);

            push(right_left);
            push(cur);

            return left;
        }
    }

    // Case 4:
    //
    //                         +---+
    //                         |cur|   <-- red
    //                         +---+
    //                        /     \
    //                       /       \
    //                    +---+     +---+
    //        black -->   |lft|     |rgt|   <-- black
    //                    +---+     +---+
    //                   /         /
    //                  /         /
    //               +---+     +---+
    //   black -->   |l_l|     |r_l|   <-- black
    //               +---+     +---+
    //
    // Solution:
    //
    //                         +---+
    //                         |cur|   <-- black
    //                         +---+
    //                        /     \
    //                       /       \
    //                    +---+     +---+
    //          red -->   |lft|     |rgt|   <-- red
    //                    +---+     +---+
    //                   /         /
    //                  /         /
    //               +---+     +---+
    //   black -->   |l_l|     |r_l|   <-- black
    //               +---+     +---+
    //
    cur.is_red.set(false);
    left.is_red.set(true);
    right.is_red.set(true);
    push(cur);
    left
}

/// Input: Either `cur` or `cur.left` is red.
///
/// Output: Either `right` or `right.left` is red.
///
/// Pushes the path to `right` (starting at `cur`, excluding `right`).
pub fn right<'a, F>(cur: &'a Node, right: &'a Node, mut push: F)
    where F: FnMut(&'a Node),
{
    if let Some(right_left) = right.left() {
        if right_left.is_red.get() {
            // Case:
            //
            //               +---+
            //               |cur|
            //               +---+
            //                    \
            //                     \
            //                    +---+
            //                    |rgt|   <-- black
            //                    +---+
            //                   /
            //                  /
            //               +---+
            //               |r_l|   <-- red
            //               +---+
            push(cur);
            return;
        }
    }

    // Exists because `right` exists.
    let mut left = unsafe { cur.unchecked_left() };

    if left.is_red.get() {
        // Case:
        //
        //                  +---+
        //                  |cur|   <-- black
        //                  +---+
        //                 /     \
        //                /       \
        //             +---+     +---+
        //   red -->   |lft|     |rgt|   <-- black
        //             +---+     +---+
        //                  \
        //                   \
        //                  +---+
        //                  |l_r|   <-- black
        //                  +---+
        //
        // Solution: Reduce to one of the cases below.
        //
        //                  +---+
        //                  |lft|   <-- black
        //                  +---+
        //                       \
        //                        \
        //                       +---+
        //                       |cur|   <-- red
        //                       +---+
        //                      /     \
        //                     /       \
        //                  +---+     +---+
        //      black -->   |l_r|     |rgt|   <-- black
        //                  +---+     +---+
        //

        // Exists because `right` exists and `left` is red.
        let left_right = unsafe { left.unchecked_right() };
        cur.left.set(left_right);
        left.right.set(cur);

        left.is_red.set(false);
        cur.is_red.set(true);

        push(left);
        left = left_right;
        // fallthrough
    }

    if let Some(left_left) = left.left() {
        if left_left.is_red.get() {
            // Case:
            //                              +---+
            //                              |cur|   <-- red
            //                              +---+
            //                             /     \
            //                            /       \
            //                         +---+     +---+
            //             black -->   |lft|     |rgt|   <-- black
            //                         +---+     +---+
            //                        /         /
            //                       /         /
            //                    +---+      +---+
            //          red -->   |l_l|      |r_l|   <-- black
            //                    +---+      +---+
            //
            // Solution:
            //
            //                              +---+
            //                              |lft|   <-- red
            //                              +---+
            //                             /     \
            //                            /       \
            //                         +---+     +---+
            //             black -->   |l_l|     |cur|   <-- black
            //                         +---+     +---+
            //                                        \
            //                                         \
            //                                        +---+
            //                                        |rgt|   <-- red
            //                                        +---+
            //                                       /
            //                                      /
            //                                   +---+
            //                                   |r_l|   <-- black
            //                                   +---+
            //
            cur.left.set(left.right.get());
            left.right.set(cur);

            left_left.is_red.set(false);
            left.is_red.set(true);
            cur.is_red.set(false);
            right.is_red.set(true);

            push(left);
            push(cur);
            return;
        }
    }

    // Case:
    //
    //                              +---+
    //                              |cur|   <-- red
    //                              +---+
    //                             /     \
    //                            /       \
    //                         +---+     +---+
    //             black -->   |lft|     |rgt|   <-- black
    //                         +---+     +---+
    //                        /         /
    //                       /         /
    //                    +---+      +---+
    //        black -->   |l_l|      |r_l|   <-- black
    //                    +---+      +---+
    //
    // Solution:
    //
    //                              +---+
    //                              |cur|   <-- black
    //                              +---+
    //                             /     \
    //                            /       \
    //                         +---+     +---+
    //               red -->   |lft|     |rgt|   <-- red
    //                         +---+     +---+
    //                        /         /
    //                       /         /
    //                    +---+      +---+
    //        black -->   |l_l|      |r_l|   <-- black
    //                    +---+      +---+
    //
    cur.is_red.set(false);
    left.is_red.set(true);
    right.is_red.set(true);
    push(cur);
}
