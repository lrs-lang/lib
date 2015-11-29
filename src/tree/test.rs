// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {Node};

/// Tests that the node has various properties:
///
/// * If the node is red, the left child is not red.
/// * The right child is not red.
/// * The number of black left children equals the number of black right children.
///
/// Returns the number of black children including itself.
pub fn test(node: &Node) -> usize {
    let is_red = node.is_red.get();

    let left_depth = if let Some(left) = node.left() {
        if is_red {
            assert!(!left.is_red.get());
        }
        test(left)
    } else {
        0
    };

    let right_depth = if let Some(right) = node.right() {
        assert!(!right.is_red.get());
        test(right)
    } else {
        0
    };

    assert!(left_depth == right_depth);
    left_depth + (!is_red as usize)
}
