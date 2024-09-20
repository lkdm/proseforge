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

// Make a tree with indexmap as backing storage
use indexmap::{map::*, Equivalent};

// TODO: IndexMap is one-dimensional.

struct Project {
    drafts: DirectoryTree<DraftId, DraftNode>,
    notes: DirectoryTree<NoteId, NoteNode>,
}

enum Node<K: Equivalent<K>, T> {
    Leaf {
        inner: Arc<T>,
    },
    Branch {
        inner: Arc<T>,
        children: IndexMap<K, Node<K, T>>,
        parent: Option<Weak<RefCell<Node<K, T>>>>,
    },
}

/// DirectoryTree
///
/// A generic data structure for a tree with nodes that are ordered arbitrarily. This is perfect for
/// a directory-file tree that allows the user to define an arbitrary order.
///
/// The key-value pairs have a consistent order that is determined by the sequence of insertion and removal calls on the map.
///
/// Index map is a hash table where the iteration order of key-value pairs is independent of the hash values of the keys.
///
/// Keys must impl Equivalent
struct DirectoryTree<K: Equivalent<K>, T>(IndexMap<K, Node<K, T>>);

impl<K: Equivalent<K>, V> DirectoryTree<K, V> {
    /// Creates a new empty DirectoryTree.
    pub fn new() -> Self {
        DirectoryTree(IndexMap::new())
    }
}
