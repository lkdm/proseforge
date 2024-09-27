use std::collections::BTreeMap;

use bon::Builder;

type PrivateKey = usize;

/// A value stored in the tree, that can be either B or L.
pub enum NodeValue<B: Copy, L: Copy> {
    Branch(B),
    Leaf(L),
}

#[derive(Clone, Debug)]
enum Node<B, L> {
    Leaf(Leaf<L>),
    Branch(Branch<B>),
}

#[derive(Clone, Debug, Builder)]
struct Leaf<V> {
    pub value: V,
}

impl<L: Clone + Into<B>, B: Clone> From<Leaf<L>> for Branch<B> {
    /// Converts a leaf into a branch.
    fn from(leaf: Leaf<L>) -> Self {
        Branch {
            value: leaf.value.into(),
            children: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Builder)]
struct Branch<V> {
    pub value: V,
    #[builder(default)]
    pub children: Vec<Option<PrivateKey>>,
}

impl<B: Clone + Into<L>, L: Clone> From<Branch<B>> for Leaf<L> {
    /// Converts a branch into a leaf.
    ///
    /// ⚠️ This is a lossy conversion– the `children` property will be dropped.
    fn from(branch: Branch<B>) -> Self {
        Leaf {
            value: branch.value.into(),
        }
    }
}

enum TreeError {
    KeyAlreadyExists,
    ParentNotFound,
    ParentNotBranch,
}

/// A fast tree structure that allows its contents to be ordered.
///
/// K: Public handle to reference a node. Usually whatever ID your database uses.
/// B: Branch type ie. Directory
/// L: Leaf type ie. File
#[derive(Clone, Debug, Builder)]
struct OrderedTree<K, B, L> {
    #[builder(default)]
    children: Vec<Option<Node<B, L>>>,
    #[builder(default)]
    by_id: BTreeMap<K, PrivateKey>,
}

impl<K: Ord + Clone + Copy, B: Clone + Copy, L: Clone + Copy> OrderedTree<K, B, L> {
    /// Given a public ID, get the internal nodekey.
    fn get_key(&self, id: &K) -> Option<&PrivateKey> {
        self.by_id.get(id)
    }

    /// Given a public ID, get the node.
    fn get_node(&self, key: &PrivateKey) -> Option<&Node<B, L>> {
        self.children.get(*key).and_then(|node| node.as_ref())
    }

    /// Compresses ordered vectors
    fn compress(&mut self) {
        todo!()
    }

    pub fn push_node(
        &mut self,
        node: Node<B, L>,
        id: &K,
        parent_id: Option<&K>,
    ) -> Result<(), TreeError> {
        // Check if the id already exists
        if self.by_id.contains_key(&id) {
            return Err(TreeError::KeyAlreadyExists);
        }
  
        // Get the parent key
        let parent_key = *self.get_key(parent_id).ok_or(TreeError::ParentNotFound)?;

        // Grab the new index that the child will be inserted at
        let new_index = &self.children.len().clone();

        // Ensure the parent is a Branch and get a mutable reference to it
        let parent_branch = match self.children.get_mut(parent_key) {
            Some(Some(Node::Branch(branch))) => branch,
            _ => return Err(TreeError::ParentNotBranch),
        };
        parent_branch.children.push(Some(new_index.clone()));

        // Update the mappings
        self.by_id.insert(*id, *new_index);
        self.children.push(Some(node));

        Ok(())
    }

    pub fn push_branch(&mut self, id: &K, value: &B, parent_id: &K) -> Result<(), TreeError> {
        let branch = Branch::builder().value(*value).build();
        let node = Node::Branch(branch);
        self.push_node(node, &id, parent_id)
    }

    fn insert_node(&mut self, node: Node<B, L>, id: &K, parent_id: &K, index: usize) {
        todo!()
    }

    /// Given an ID get the value
    pub fn get(&self, id: &K) -> Option<NodeValue<B, L>> {
        let key = self.get_key(id);
        if let Some(key) = key {
            let node = self.get_node(key);
            node.map(|node| match node {
                Node::Branch(b) => NodeValue::Branch(b.value),
                Node::Leaf(l) => NodeValue::Leaf(l.value),
            })
        } else {
            None
        }
    }

    /// Given an ID, remove a node
    pub fn remove(&mut self, id: &K) -> Option<Node<B, L>> {
        if let Some(&key) = self.by_id.get(id) {
            // Remove the node from the children vector
            let removed = self.children[key].take();

            // Remove the id from the by_id map
            self.by_id.remove(id);

            removed
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Id = String;
    
    #[derive(Clone, Copy)]
    struct Directory;
    #[derive(Clone, Copy)]
    struct File;

    type TestTree = OrderedTree<Id, Directory, File>;
    fn setup_tree() -> TestTree {
        OrderedTree::builder().build()
    }

    #[test]
    fn test_push() {
        let mut tree = setup_tree();
        let n1 = Branch::builder().value(File);
        tree.push_branch("Test", &n1, None)
    }
}
