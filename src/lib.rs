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

/// A project defines the
struct Project {
    draft: DirectoryTree<DraftId, DraftNode>,
    notes: DirectoryTree<NoteId, NoteNode>,
}

// Make a tree with indexmap as backing storage
use indexmap::{map::*, Equivalent};

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
struct DirectoryTree<K: Equivalent<K>, V>(IndexMap<K, V>);

impl<K: Equivalent<K>, V> DirectoryTree<K, V> {
    /// Creates a new empty DirectoryTree.
    pub fn new() -> Self {
        DirectoryTree(IndexMap::new())
    }
}
