use bon::{bon, builder};
use slotmap::{new_key_type, Key, SecondaryMap, SlotMap};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    sync::{Arc, Weak},
};
use ulid::Ulid;
// Make a tree with indexmap as backing storage
use std::hash::Hash;

enum Node<K, B, L> {
    /// Branch is analagous to a directory
    Branch { key: K, value: B, children: Vec<K> },
    /// Leaf is analagous to a file
    Leaf { key: K, value: L },
}

#[bon]
impl<K, B, L> Node<K, B, L> {
    /// Create a new Node
    ///
    /// Ideally you should use a bon builder for this
    #[builder]
    fn new(key: K, branch: Option<B>, leaf: Option<L>, children: Option<Vec<K>>) -> Self {
        match (branch, leaf) {
            (Some(b), None) => Node::Branch {
                key,
                value: b,
                children: children.unwrap_or_else(Vec::new),
            },
            (None, Some(l)) => Node::Leaf { key, value: l },
            _ => panic!("A node must be either a Branch or a Leaf, not both or neither."),
        }
    }
    fn push(&mut self, key: K) -> () {
        match self {
            Node::Branch { children, .. } => children.push(key),
            _ => (),
        }
    }
    /// Inserts an element at position `index` within the vector, shifting all elements after it to the right.
    fn insert(&mut self, key: K, index: usize) -> () {
        match self {
            Node::Branch { children, .. } => children.insert(index, key),
            _ => (),
        }
    }
    fn remove(&mut self, index: usize) -> K {
        match self {
            Node::Branch { children, .. } => children.remove(index),
            _ => panic!("Cannot remove child because Node has no children."),
        }
    }
    fn len(&self) -> usize {
        match self {
            Node::Branch { children, .. } => children.len(),
            _ => 0,
        }
    }
}

/// An user-ordered tree structure
///
/// Represented as a flat list of nodes, by key, with references to their children.
///
/// K: Key
/// B: Branch
/// L: Leaf
pub(crate) struct Tree<K: Key, B, L>(SlotMap<K, Node<K, B, L>>);
impl<K: Key, B, L> Tree<K, B, L> {
    fn new() -> Self {
        Tree(SlotMap::with_key())
    }
}

new_key_type! {
    struct NoteKey;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests methods related to Nodes.
    fn test_node() {
        type TestNode = Node<u32, String, char>;

        // Create a basic hierarchy
        // parent_node
        //   - child_node
        //   - child_node2
        let child_node: TestNode = Node::builder().key(0).leaf('a').build();
        let child_node2: TestNode = Node::builder().key(1).leaf('b').build();
        let parent_node: TestNode = Node::builder()
            .key(2)
            .branch("directory1".into())
            .children(vec![0, 1])
            .build();

        assert_eq!(parent_node.len(), 2);
    }
    #[test]
    /// Tests serialisation and deserialisation into a format compatible for a persistent storage medium.
    fn test_node_serialisation() {}
}
