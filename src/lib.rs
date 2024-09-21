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

#[derive(Clone, Debug)]
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
            Some(children) => Self::Branch { value, children },
            _ => Self::Leaf { value },
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
#[derive(Debug)]
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
    fn push_node(&mut self, value: Node<K, V>, parent_key: Option<K>) -> Option<K> {
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
    fn insert_node(&mut self, value: Node<K, V>, parent_key: Option<K>, index: usize) -> Option<K> {
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

    /// Explicitly convert a Leaf into a Branch and vice versa.
    fn node_convert(&mut self, key: &K) {
        if let Some(node) = self.nodes.get_mut(*key) {
            *node = match node {
                Node::Leaf { value } => Node::Branch {
                    value: value.clone(),
                    children: Vec::new(),
                },
                Node::Branch { value, children } => Node::Leaf {
                    value: value.clone(),
                },
            };
        }
    }
    /// Push a value to the tree
    pub fn push(&mut self, value: &V, parent_key: Option<K>) -> Option<K> {
        let node: Node<K, V> = Node::builder().value(value.clone()).build();
        self.push_node(node, parent_key)
    }

    /// Insert a value at location
    pub fn insert(&mut self, value: &V, parent_key: Option<K>, index: usize) -> Option<K> {
        let node: Node<K, V> = Node::builder().value(value.clone()).build();
        self.insert_node(node, parent_key, index)
    }

    /// Given a key, get a value from the tree.
    pub fn get(&self, key: &K) -> Option<V> {
        let maybe_node = self.nodes.get(*key);
        match maybe_node {
            Some(node) => match node.clone() {
                Node::Branch { value, .. } => Some(value),
                Node::Leaf { value } => Some(value),
            },
            _ => None,
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
    type TestContent = String;

    type TestTree = Tree<NodeId, TestContent>;

    // tree.push(&"🦘".to_string(), earth);
    // tree.push(&"🦀".to_string(), earth);
    // tree.push(&"🚀".to_string(), earth);
    // tree.push(&"👨‍🚀".to_string(), earth);

    /// Setup an empty tree for testing
    fn setup_tree() -> TestTree {
        let mut tree: TestTree = Tree::builder().build();
        tree
    }

    /// Setup a tree with some example nodes for testing.
    fn setup_tree_with_nodes() -> (TestTree, BTreeMap<String, NodeId>) {
        let mut tree: TestTree = Tree::builder().build();
        // Keep track of parent nodes.
        let mut index_table: BTreeMap<String, NodeId> = BTreeMap::new();
        if let Some(id) = tree.push(&"🌎".to_string(), None) {
            index_table.insert("Earth".to_string(), id);
        }
        if let Some(id) = tree.push(&"🌕".to_string(), None) {
            index_table.insert("Moon".to_string(), id);
        }
        if let Some(id) = tree.push(&"🪐".to_string(), None) {
            index_table.insert("Saturn".to_string(), id);
        }
        (tree, index_table)
    }

    #[test]
    fn test_push() {
        let mut tree = setup_tree();

        let parent = tree.push(&"🌎".to_string(), None);
        dbg!(&tree);
        assert!(
            parent.is_some(),
            "Inserting a node at root should return a key."
        );

        if let Some(parent_id) = parent {
            let item = tree.nodes.get(parent_id);
            assert!(
                item.is_some(),
                "With a key, a node should be able to be retrived from the tree."
            );

            assert_eq!(
                tree.get(&parent_id),
                Some("🌎".to_string()),
                "With a key, a node should be able to be retrived from the tree."
            )
        }

        // Second, assert that tree contains item for index Parent.
        // Third, try pushing to Parent node.
        // Fourth, try sibling node at root.
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
