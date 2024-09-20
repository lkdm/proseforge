use bon::{bon, builder};
use slotmap::{new_key_type, Key, SlotMap};
use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};
// Make a tree with indexmap as backing storage
use indexmap::{map::*, Equivalent};
use std::hash::Hash;

enum Node<K: Key> {
    Branch { id: K, children: Vec<Node<K>> },
    Leaf { id: K },
}

impl<K: Key> Node<K> {
    pub fn new_branch(id: K, children: Vec<Node<K>>) -> Self {
        Node::Branch { id, children }
    }
    pub fn new_leaf(id: K) -> Self {
        Node::Leaf { id }
    }

    /// Insert child
    pub fn insert_child(&mut self, id: &K) {}
    /// Insert child before a particular child.
    pub fn insert_before(&mut self, id: &K, before: &K) {}
    /// Swap an entry to the end, then pop it off– returning a reference to it.
    pub fn swap_remove(&mut self, id: &K) -> (&K, &Self) {}
}

/// DataTree
///
/// Data referenced by ID, paired with a tree of IDs.
struct Tree<K: Key, V> {
    children: Vec<Node<K>>,
    data: SlotMap<K, V>,
}

impl<K: Key, V> Tree<K, V> {
    pub fn new() -> Self {
        Tree {
            children: Vec::new(),
            data: SlotMap::with_key(),
        }
    }
    pub fn insert_child(&mut self, id: &K, parent: &K) {}
    pub fn insert_before(&mut self, id: &K, before: &K) {}
    pub fn remove(&mut self, id: &K) {}
    pub fn move_to_parent(&mut self, id: &K, parent: &K) {}
    pub fn move_before_sibling(&mut self, id: &K, parent: &K) {}
    // TODO: recursive crawl.

    /// Borrow a reference to the data
    pub fn get_data(&self, id: &K) -> Option<&V> {
        self.data.get(*id)
    }
    /// Borrow a mutable reference to the data
    pub fn get_data_mut(&mut self, id: &K) -> Option<&mut V> {
        self.data.get_mut(*id)
    }
}
