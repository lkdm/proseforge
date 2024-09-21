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

// TODO: IF you add a key, you will need `insert_with_key`.
// You will then need to use a function `sm.insert_with_key(|k| (k, 20));`
//
// So the Tree might become SlotMap<(K, Node<K, V>)>
// THIS MIGHT NOT BE NECESSARY. TRY TO IMPLEMENT IT WITHOUT IT.
//
// TODO: In addition, if we were to want reverse-lookup by stable id or title or tags,
// we would use secondary maps.

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
    fn new(key: K, value: V, children: Option<Vec<K>>) -> Self {
        match children {
            Some(children) => Self::Branch {
                key,
                value,
                children,
            },
            _ => Self::Leaf { key, value },
        }
    }
    fn push(&mut self, key: K) -> () {
        match self {
            Node::Branch { children, .. } => children.push(key),
            _ => (),
        }
    }
    /// Inserts an element at position `index` within the vector, shifting all elements after it to the right.
    fn insert(&mut self, index: usize, key: K) -> () {
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
    children: Vec<K>,
}

#[bon]
impl<K: Key, V: Clone> Tree<K, V> {
    /// Create a new tree
    #[builder]
    fn new(nodes: Option<SlotMap<K, Node<K, V>>>, root_nodes: Option<Vec<K>>) -> Self {
        match (nodes, root_nodes) {
            (Some(nodes), Some(root_nodes)) => Tree {
                nodes,
                children: root_nodes,
            },
            _ => Tree {
                nodes: SlotMap::with_key(),
                children: Vec::new(),
            },
        }
    }

    /// Push a node without caring about its index.
    fn push(&mut self, value: Node<K, V>, parent_key: Option<K>) -> Option<K> {
        match parent_key {
            Some(parent_key) => {
                let key = self.nodes.insert(value);
                let parent = self.nodes.get_mut(parent_key);
                match parent {
                    Some(parent) => {
                        parent.push(key);
                        Some(key)
                    }
                    None => None, // Noop. TODO: this is a different None to the one below. Should be handled differently.
                }
            }
            None => {
                let key = self.nodes.insert(value);
                self.children.push(key);
                None // Parent is root.
            }
        }
    }

    /// Insert a node at index.
    fn insert(&mut self, value: Node<K, V>, parent_key: Option<K>, index: usize) -> Option<K> {
        match parent_key {
            Some(parent_key) => {
                let key = self.nodes.insert(value);
                let parent = self.nodes.get_mut(parent_key);
                match parent {
                    Some(parent) => {
                        parent.insert(index, key);
                        Some(key)
                    }
                    None => None, // Noop. TODO: this is a different None to the one below. Should be handled differently.
                }
            }
            None => {
                let key = self.nodes.insert(value);
                self.children.insert(index, key);
                None // Parent is root.
            }
        }
    }

    // Inserts a branch, containing a value
}

// /// Moves a node with key K to a given parent and index
// fn move(&mut self, key: K, parent_key: K, index: u32) {
//     let node_option = self.get(key);
//     let parent_option = self.get_mut(parent_key);
//     match node {
//         Some(node) =>
//     }
// }
// get children
// new_key_type! {
//     struct NoteKey;
// }

#[cfg(test)]
mod tests {
    use super::*;

    new_key_type! {
        struct NodeId;
    }
    type TestContent = u32;

    type TestTree = Tree<NodeId, TestContent>;

    fn setup() -> TestTree {
        Tree::builder().build()
    }

    #[test]
    fn test_tree() {
        let tree = setup();

        let node1 =

        tree.
    }

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
