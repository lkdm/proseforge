use bon::{bon, builder};
use slotmap::{new_key_type, Key, SlotMap};
use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};
// Make a tree with indexmap as backing storage
use indexmap::{map::*, Equivalent};
use std::hash::Hash;

// new_key_type! {
//     pub struct NoteId;
//     pub struct DraftId;
// }

// enum Node<K: Key> {
//     Branch { id: K, children: Vec<Node<K>> },
//     Leaf { id: K },
// }

// struct DirTree<K: Key, V> {
//     children: Vec<Node<K>>,
//     data: SlotMap<K, V>,
// }

// impl<K: Key, V> Default for DirTree<K, V> {
//     fn default() -> Self {
//         DirTree {
//             children: Vec::new(),
//             data: SlotMap::with_key(),
//         }
//     }
// }

// impl<K: Key, V> DirTree<K, V> {
//     pub fn new(children: Vec<Node<K>>, data: SlotMap<K, V>) -> Self {
//         DirTree { children, data }
//     }
// }

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
}

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
    pub fn swap_remove(&mut self, id: &K) -> &K {}
}
