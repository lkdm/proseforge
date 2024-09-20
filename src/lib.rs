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

#[derive(Clone)]
enum Node<K, V> {
    /// Branch is analagous to a directory
    Branch { value: V, children: Vec<K> },
    /// Leaf is analagous to a file
    Leaf { value: V },
}

#[bon]
impl<K, V> Node<K, V> {
    /// Create a new Node
    ///
    /// Ideally you should use a bon builder for this
    #[builder]
    fn new(value: V, children: Option<Vec<K>>) -> Self {
        match children {
            Some(children) => {
                Self::Branch { value, children }
            }
            _ => Self::Leaf { value }
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
/// V: Node
pub(crate) struct Tree<K: Key, V> {
    /// SlotMap of key -> Node, with each Node containing a Vec of keys as its children
    nodes: SlotMap<K, Node<K, V>>,
    /// Reference children at the root.
    children: Vec<K>
}

#[bon]
impl<K: Key, V: Clone> Tree<K, V> {

    /// Create a new tree
    #[builder]
    fn new(nodes: Option<SlotMap<K, Node<K, V>>>, root_nodes: Option<Vec<K>>) -> Self {
        match (nodes, root_nodes) {
            (Some(nodes), Some(root_nodes)) => Tree {
                nodes,
                children: Some(root_nodes)
            },
            _ => Tree {
                nodes: SlotMap::with_key(),
                children: Vec::new()
            }}
        }
    }

    /// Given a node, insert it
    fn insert(&mut self, value: Node<K, V>) -> K {
        match Some(self.root_node) {
            Some(root_node) => self.nodes.insert(value) // TODO: Insert at root_node
            None => {
                let key = self.nodes.insert(value);
                self.root_node = key;
            }
        }

    }
    fn get(&self, key: K) -> Option<&Node<K, V>> {
        self.nodes.get(key)
    }
    fn get_mut(&mut self, key: K) -> Option<&mut Node<K, V>> {
        self.nodes.get_mut(key)
    }
    /// Moves a node with key K to a given parent and index
    fn move(&mut self, key: K, parent_key: K, index: u32) {
        let node_option = self.get(key);
        let parent_option = self.get_mut(parent_key);
        match node {
            Some(node) =>
        }
    }
    // get children

}

// new_key_type! {
//     struct NoteKey;
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // /// Tests methods related to Nodes.
    // fn test_node() {
    //     type TestNode = Node<u32, String, char>;

    //     // Create a basic hierarchy
    //     // parent_node
    //     //   - key for child_node
    //     //   - key for child_node2
    //     let child_node: TestNode = Node::builder().key(0).leaf('a').build();
    //     let child_node2: TestNode = Node::builder().key(1).leaf('b').build();
    //     let parent_node: TestNode = Node::builder()
    //         .key(2)
    //         .branch("directory1".into())
    //         .children(vec![0, 1])
    //         .build();

    //     assert_eq!(parent_node.len(), 2);
    // }
    #[test]
    /// Tests serialisation and deserialisation into a format compatible for a persistent storage medium.
    fn test_node_serialisation() {}
}
