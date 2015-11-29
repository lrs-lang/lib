// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_tree"]
#![crate_type = "lib"]
#![feature(no_std, lang_items, optin_builtin_traits, const_fn, associated_type_defaults)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cell as cell;
extern crate lrs_fmt as fmt;

use base::prelude::*;
use core::{mem};
use core::marker::{Leak};
use core::cmp::{Ordering};
use cell::{Cell};

mod std { pub use ::fmt::std::*; }

mod path;
mod insert;
mod remove;
mod test;
mod debug;

/// Recipes for retrieving nodes from objects.
///
/// = Remarks
///
/// This trait is unsafe because its methods must follow certain rules.
///
/// The methods exposed by this trait should not be called from outside the tree
/// implementation.
pub unsafe trait GetNode {
    /// The object containing the node.
    type Owner = Self;

    /// Retrieves a node from an object.
    ///
    /// [argument, el]
    /// A pointer to an object containing a node.
    ///
    /// [return_value]
    /// The node in the object.
    ///
    /// = Remarks
    ///
    /// The pointer always points to an allocated object, however, the contents of said
    /// object can be undefined. Hence, the object must not be dereferenced or the
    /// behavior is undefined.
    ///
    /// The implementation must be expressable as a constant offset from the passed
    /// pointer. That is, it must be possible to write the implementation as follows:
    ///
    /// ----
    /// (el as usize + OFFSET) as *const Node
    /// ----
    ///
    /// Where `OFFSET` is a constant. Otherwise the behavior is undefined. This
    /// restriction allows implementations such as
    ///
    /// ----
    /// &(*el).node
    /// ----
    unsafe fn get_node(el: *const Self::Owner) -> *const Node;

    /// Compares two objects.
    ///
    /// [argument, left]
    /// The object on the left-hand-side of the inequality.
    ///
    /// [argument, right]
    /// The object on the right-hand-side of the inequality.
    ///
    /// [return_value]
    /// Returns the comparison of the objects.
    fn cmp(left: &Self::Owner, right: &Self::Owner) -> Ordering;
}

/// Recipes for inserting objects into trees.
///
/// = Remarks
///
/// An `Entree` type describes how an object containing a node is inserted into a tree.
///
/// This trait is unsafe because its methods must follow certain rules.
///
/// The methods exposed by this trait should not be called from outside the tree
/// implementation.
pub unsafe trait Entree: GetNode + Leak {
    /// A reference type.
    type Ref = Self;

    /// Retrieves a reference to the owner.
    ///
    /// [argument, el]
    /// A reference to the reference type.
    ///
    /// [return_value]
    /// Returns a reference to the node owner.
    unsafe fn get_owner(el: &Self::Ref) -> &Self::Owner;

    /// Increases the reference count by one.
    ///
    /// [argument, el]
    /// The node owner whose reference count to increase.
    ///
    /// = Remarks
    ///
    /// The object must not be moved as long as there exists a reference to it.
    unsafe fn add_ref(el: &Self::Owner);

    /// Decreases the reference count by one.
    ///
    /// [argument, el]
    /// The node owner whose reference count to decrease.
    unsafe fn del_ref(el: &Self::Owner);

    /// Turns a reference into an instance of the reference type.
    ///
    /// [argument, el]
    /// The node owner which the return value will reference.
    ///
    /// [return_value]
    /// Returns an instance of the reference type.
    ///
    /// = Remarks
    ///
    /// This function does not modify the reference count.
    unsafe fn get_ref(el: &Self::Owner) -> Self::Ref;
}

/// A node in a tree.
pub struct Node {
    left: Cell<*const Node>,
    right: Cell<*const Node>,
    is_red: Cell<bool>,
    in_tree: Cell<bool>,
}

struct __SyncNode {
    node: Node,
}

unsafe impl Sync for __SyncNode { }

static __LEAF: __SyncNode = __SyncNode {
    node: Node {
        left: Cell::new(0 as *const _),
        right: Cell::new(0 as *const _),
        is_red: Cell::new(false),
        in_tree: Cell::new(true),
    }
};

impl Node {
    /// Creates a new node.
    pub fn new() -> Node {
        Node {
            left: Cell::new(&__LEAF.node),
            right: Cell::new(&__LEAF.node),
            is_red: Cell::new(true),
            in_tree: Cell::new(false),
        }
    }

    /// Returns the left child of the node, if any.
    fn left(&self) -> Option<&Node> {
        if self.left.get() == &__LEAF.node {
            None
        } else {
            unsafe { Some(&*self.left.get()) }
        }
    }

    /// Returns the right child of the node, if any.
    fn right(&self) -> Option<&Node> {
        if self.right.get() == &__LEAF.node {
            None
        } else {
            unsafe { Some(&*self.right.get()) }
        }
    }

    /// Returns whether this node is a leaf.
    fn is_leaf(&self) -> bool {
        mem::addr(self) == mem::addr(&__LEAF.node)
    }

    /// Returns the left child of the node without checking if it's a leaf.
    unsafe fn unchecked_left(&self) -> &Node {
        &*self.left.get()
    }

    /// Returns the right child of the node without checking if it's a leaf.
    unsafe fn unchecked_right(&self) -> &Node {
        &*self.right.get()
    }
}

/// A left-leaning red-black tree.
pub struct Tree<E>
    where E: Entree,
{
    root: Cell<*const Node>,
    _marker: PhantomData<E>,
}

impl<E> Tree<E>
    where E: Entree
{
    /// Creates a new tree.
    ///
    /// [return_value]
    /// Returns an empty tree.
    pub const fn new() -> Tree<E> {
        Tree {
            root: Cell::new(0 as *const _),
            _marker: PhantomData,
        }
    }

    /// Inserts an element into the tree.
    ///
    /// [argument, val]
    /// The element to insert.
    ///
    /// [return_value]
    /// Returns whether the element was inserted.
    pub fn insert(&self, val: &E::Ref) -> bool {
        let owner = unsafe { E::get_owner(val) };

        let node = get_node::<E>(owner);
        if node.in_tree.get() {
            return false;
        }

        unsafe { E::add_ref(owner); }
        node.left.set(&__LEAF.node);
        node.right.set(&__LEAF.node);
        node.is_red.set(true);
        node.in_tree.set(true);

        let root = if let Some(root) = self.root() {
            insert::insert::<E>(root, owner)
        } else {
            node
        };

        root.is_red.set(false);
        self.root.set(root);

        if cfg!(debug_tree) {
            self.test();
        }
        true
    }

    /// Removes the smallest element in the tree.
    ///
    /// [return_value]
    /// Returns the smallest element, if any.
    pub fn remove_min(&self) -> Option<E::Ref> {
        let root = match self.root() {
            Some(r) => r,
            _ => return None,
        };

        let (min, root) = if let Some(root_left) = root.left() {
            if !root_left.is_red.get() {
                root.is_red.set(true);
            }
            remove::remove_min(root)
        } else {
            (root, &__LEAF.node)
        };

        if root.is_leaf() {
            self.root.set(0 as *const _);
        } else {
            root.is_red.set(false);
            self.root.set(root);
        }

        min.in_tree.set(false);

        let min = get_owner::<E>(min);

        if cfg!(debug_tree) {
            self.test();
        }

        unsafe { Some(E::get_ref(min)) }
    }

    /// Removes an element from the tree.
    ///
    /// [argument, val]
    /// The element to remove.
    ///
    /// [return_value]
    /// Returns whether the element was removed.
    pub fn remove(&self, val: &E::Ref) -> bool {
        let root = match self.root() {
            Some(r) => r,
            _ => return false,
        };

        let owner = unsafe { E::get_owner(val) };

        root.is_red.set(true);
        if let Some(root_left) = root.left() {
            if root_left.is_red.get() {
                root.is_red.set(false);
            }
        }

        let (root, found) = remove::remove::<E>(root, owner);

        if root.is_leaf() {
            self.root.set(0 as *const _);
        } else {
            root.is_red.set(false);
            self.root.set(root);
        }

        if let Some(found) = found {
            found.in_tree.set(false);
            unsafe { E::del_ref(owner); }
        }

        if cfg!(debug_tree) {
            self.test();
        }

        found.is_some()
    }

    /// Tests that the tree is in a valid state.
    ///
    /// [return_value]
    /// Returns the depth of the tree.
    ///
    /// = Remarks
    ///
    /// If the tree is in an invalid state, the process is aborted.
    pub fn test(&self) -> usize {
        if let Some(root) = self.root() {
            assert!(!root.is_red.get());
            test::test(root)
        } else {
            0
        }
    }

    /// Returns the root of the tree, if any.
    fn root(&self) -> Option<&Node> {
        if self.root.get().is_null() {
            None
        } else {
            unsafe { Some(&*self.root.get()) }
        }
    }
}

/// Prints the tree in the graphviz language.
///
/// = Remarks
///
/// This only works if the `Debug` implementation of the contained object does not print
/// any unescapted newlines or quotation marks.
impl<E> fmt::Debug for Tree<E>
    where E: Entree,
          E::Owner: fmt::Debug
{
    fn fmt<W: fmt::Write>(&self, w: &mut W) -> Result {
        if let Some(root) = self.root() {
            debug::debug::<E, W>(w, root)
        } else {
            Ok(())
        }
    }
}

/// Returns a reference to the owner of a node.
fn get_owner<E>(node: &Node) -> &E::Owner
    where E: Entree
{
    unsafe {
        let el = mem::uninit();
        let offset = mem::addr(get_node::<E>(&el)) - mem::addr(&el);
        &*((node as *const _ as *const u8).sub(offset) as *const _)
    }
}

/// Returns the node in an owner.
fn get_node<E>(el: &E::Owner) -> &Node
    where E: Entree
{
    unsafe { &*E::get_node(el) }
}
