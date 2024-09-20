#[derive(Debug, Clone)]
struct Document {
    content: String,
    is_modified: bool,
}

#[derive(Debug, Clone)]
enum DraftNode {
    Leaf {
        title: String,
        summary: String,
        content: Document,
    },
    Branch {
        title: String,
        summary: String,
    },
}

#[derive(Debug, Clone)]
enum NoteNode {
    Leaf { title: String, content: Document },
    Branch { title: String },
}

type NoteId = u32;
type DraftId = u32;

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

use bon::{bon, builder};
// Make a tree with indexmap as backing storage
use indexmap::{map::*, Equivalent};
use std::hash::Hash;

// TODO: IndexMap is one-dimensional.

struct Project {
    drafts: Node<DraftId, DraftNode>,
    notes: Node<NoteId, NoteNode>,
}

enum Node<K: Equivalent<K> + Hash + Eq, V> {
    Leaf {
        inner: Arc<V>,
        parent: Weak<RefCell<Node<K, V>>>,
    },
    Branch {
        inner: Arc<V>,
        children: IndexMap<K, Node<K, V>>,
        parent: Option<Weak<RefCell<Node<K, V>>>>,
    },
}

#[bon]
impl<K: Equivalent<K> + Hash + Eq, V> Node<K, V> {
    /// Creates a new branch node.
    #[builder]
    fn new_branch(
        inner: V,
        children: Option<IndexMap<K, Node<K, V>>>,
        parent: Option<Weak<RefCell<Node<K, V>>>>,
    ) -> Self {
        match children {
            Some(c) => Node::Branch {
                inner: Arc::new(inner),
                children: c,
                parent,
            },
            _ => Node::Branch {
                inner: Arc::new(inner),
                children: IndexMap::new(),
                parent,
            },
        }
    }
    /// Creates a new leaf node.
    #[builder]
    pub fn new_leaf(inner: V, parent: Weak<RefCell<Node<K, V>>>) -> Self {
        Node::Leaf {
            inner: Arc::new(inner),
            parent,
        }
    }
    /// Adds a child last in the branch.
    fn add_child(&mut self, key: K, node: Node<K, V>) {
        match self {
            Node::Leaf { .. } => {
                panic!("Cannot add a child to a leaf node");
            }
            Node::Branch { children, .. } => {
                children.insert(key, node);
            }
        }
    }
    /// Adds the node as a child, after a predecessor.
    ///
    /// [Documentation](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html#method.insert_before)
    fn insert_before(&mut self, key: K, node: Node<K, V>, index: usize) {
        match self {
            Node::Branch { children, .. } => children.insert_before(index, key, node),
            _ => panic!("Cannot add a child to a leaf node."),
        };
    }
    /// Swaps entry with last, and pops it off. Essentially removing the entry.
    ///
    /// Returns a Key, Value pair.
    ///
    /// [Documentation](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html#method.swap_remove_entry)
    fn swap_remove_entry(&mut self, key: &K) -> Option<(K, Node<K, V>)> {
        match self {
            Node::Branch { children, .. } => children.swap_remove_entry(key),
            Node::Leaf { .. } => panic!("Cannot remove child from leaf node."),
            _ => None,
        }
    }

    /// Returns a reference to a node by key
    fn get_node(&self, key: K) -> &Node<K, V> {
        todo!()
    }
    /// Return number of children on this level.
    fn children_count(&self) -> usize {
        match self {
            Node::Branch { children, .. } => children.len(),
            _ => 0,
        }
    }
}

// /// DirectoryTree
// ///
// /// A generic data structure for a tree with nodes that are ordered arbitrarily. This is perfect for
// /// a directory-file tree that allows the user to define an arbitrary order.
// ///
// /// The key-value pairs have a consistent order that is determined by the sequence of insertion and removal calls on the map.
// ///
// /// Index map is a hash table where the iteration order of key-value pairs is independent of the hash values of the keys.
// ///
// /// Keys must impl Equivalent
// struct DirectoryTree<K: Equivalent<K>, T>(IndexMap<K, Node<K, T>>);

// impl<K: Equivalent<K>, V> DirectoryTree<K, V> {
//     /// Creates a new empty DirectoryTree.
//     pub fn new() -> Self {
//         DirectoryTree(IndexMap::new())
//     }
//     pub fn insert(&mut self, key: K, node: Node<K, V>) {
//         self.0.insert(key, node);
//         // TODO: Maintain the order of nodes
//     }
//     pub fn remove(&mut self, key: &K) -> Option<Node<K, V>> {
//         self.0.remove(key)
//         // TODO: Update parent references
//     }
//     pub fn get(&self, key: &K) -> Option<&Node<K, V>> {
//         self.0.get(key)
//     }
// }
