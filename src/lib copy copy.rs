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

struct Project {
    draft: DirectoryTree<DraftId, DraftNode>,
    notes: DirectoryTree<NoteId, NoteNode>, // notes
}

// Make a tree with indexmap as backing storage
use indexmap::{map::*, Equivalent};

// Index map is a hash table where the iteration order of key-value pairs is independent of the hash values of the keys.
//
// Keys must impl Equivalent
struct DirectoryTree<K: Equivalent<K>, V>(IndexMap<K, V>);
