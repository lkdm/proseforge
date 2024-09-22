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
    collections::VecDeque,
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

enum Node<T> {
    Root { children: Vec<Node<T>> },
    Branch { id: T, children: Vec<Node<T>> },
    Leaf { id: T },
}

struct DirTree {
    tree: Nodes,
    data_by_id: T,
}

// enum Node<K: Equivalent<K> + Hash + Eq + Copy, V> {
//     Root {
//         id: K,
//         children: IndexMap<K, Node<K, V>>,
//     },
//     Leaf {
//         id: K,
//         inner: Arc<V>,
//         parent: Weak<Node<K, V>>,
//     },
//     Branch {
//         id: K,
//         inner: Arc<V>,
//         children: IndexMap<K, Node<K, V>>,
//         parent: Weak<Node<K, V>>,
//     },
// }

// struct NodePath<K>(VecDeque<K>);

// impl<K> NodePath<K> {
//     /// Creates a new empty NodePath.
//     pub fn new() -> Self {
//         NodePath(VecDeque::new())
//     }
//     /// Adds a key to the left end of the path.
//     pub fn push_left(&mut self, key: K) {
//         self.0.push_front(key);
//     }
//     /// Adds a key to the right end of the path.
//     pub fn push_right(&mut self, key: K) {
//         self.0.push_back(key);
//     }

//     /// Consumes and removes a key from the left end of the path.
//     pub fn pop_left(&mut self) -> Option<K> {
//         self.0.pop_front()
//     }

//     /// Returns a reference to the keys in the path.
//     pub fn get_path(&self) -> &VecDeque<K> {
//         &self.0
//     }

//     /// Returns the length of the path.
//     pub fn len(&self) -> usize {
//         self.0.len()
//     }

//     /// Checks if the path is empty.
//     pub fn is_empty(&self) -> bool {
//         self.0.is_empty()
//     }
// }

// #[bon]
// impl<K: Equivalent<K> + Hash + Eq + Copy, V> Node<K, V> {
//     /// Returns the id
//     pub fn id(&self) -> &K {
//         match self {
//             Node::Branch { id, .. } | Node::Leaf { id, .. } | Node::Root { id, .. } => id,
//         }
//     }

//     /// Returns a path to the current Node.
//     pub fn path(&self) -> NodePath<K> {
//         let mut path = NodePath::new();
//         let mut current = self;

//         loop {
//             match current {
//                 Node::Leaf { parent, .. } | Node::Branch { parent, .. } => {
//                     path.push_left(current.id().clone());
//                     // Borrow the parent
//                     if let Some(parent) = parent.upgrade() {
//                         current = &*parent.copy();
//                     } else {
//                         break;
//                     }
//                 }
//                 Node::Root { .. } => break,
//             }
//         }
//         path
//     }

//     /// Creates a new branch node.
//     #[builder]
//     fn new_branch(
//         inner: V,
//         children: Option<IndexMap<K, Node<K, V>>>,
//         parent: Option<Weak<RefCell<Node<K, V>>>>,
//     ) -> Self {
//         match children {
//             Some(c) => Node::Branch {
//                 inner: Arc::new(inner),
//                 children: c,
//                 parent,
//             },
//             _ => Node::Branch {
//                 inner: Arc::new(inner),
//                 children: IndexMap::new(),
//                 parent,
//             },
//         }
//     }
//     /// Creates a new leaf node.
//     #[builder]
//     pub fn new_leaf(inner: V, parent: Weak<RefCell<Node<K, V>>>) -> Self {
//         Node::Leaf {
//             inner: Arc::new(inner),
//             parent,
//         }
//     }
//     /// Adds a child last in the branch.
//     fn add_child(&mut self, key: K, node: Node<K, V>) {
//         match self {
//             Node::Leaf { .. } => {
//                 panic!("Cannot add a child to a leaf node");
//             }
//             Node::Branch { children, .. } => {
//                 children.insert(key, node);
//             }
//         }
//     }
//     /// Adds the node as a child, after a predecessor.
//     ///
//     /// [Documentation](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html#method.insert_before)
//     fn insert_before(&mut self, key: K, node: Node<K, V>, index: usize) {
//         match self {
//             Node::Branch { children, .. } => children.insert_before(index, key, node),
//             _ => panic!("Cannot add a child to a leaf node."),
//         };
//     }
//     /// Swaps entry with last, and pops it off. Essentially removing the entry.
//     ///
//     /// Returns a Key, Value pair.
//     ///
//     /// [Documentation](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html#method.swap_remove_entry)
//     fn swap_remove_entry(&mut self, key: &K) -> Option<(K, Node<K, V>)> {
//         match self {
//             Node::Branch { children, .. } => children.swap_remove_entry(key),
//             Node::Leaf { .. } => panic!("Cannot remove child from leaf node."),
//             _ => None,
//         }
//     }

//     /// Returns a reference to a node by key
//     fn get_node(&self, key: K) -> &Node<K, V> {
//         todo!()
//     }
//     /// Return number of children on this level.
//     fn children_count(&self) -> usize {
//         match self {
//             Node::Branch { children, .. } => children.len(),
//             _ => 0,
//         }
//     }
// }

// pub fn

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
