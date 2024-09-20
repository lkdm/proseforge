use bon::{bon, builder};
use slotmap::{new_key_type, Key, SecondaryMap, SlotMap};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    sync::{Arc, Weak},
};
// Make a tree with indexmap as backing storage
use indexmap::{map::*, Equivalent};
use std::hash::Hash;

// Node gets serialised into this for storage in db
struct NodeModel {
    id: u32,
    parent_id: u32,
    order: u32,
}

enum Node<K, V> {
    Branch { key: K, children: Vec<K>, value: V },
    Leaf { key: K, value: V },
}
/// An user-ordered tree structure
///
/// Represented as a flat list of nodes, by key, with references to their children.
pub(crate) struct Tree<K: Key, V>(SlotMap<K, Node<K, V>>);

new_key_type! {
    struct NoteKey;
}
struct NoteFile {
    name: String,
    note_id: u32,
}
struct Project {
    note_files: Tree<NoteKey, NoteFile>,
    // notes: BTreeMap<u32, Note>,
}
