use bon::Builder;
use serde::{Serialize, Deserialize}

#[derive(Serialize, Deserialize)]
pub struct Tree(Vec<Node<String>>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Node<T> {
    Leaf(Leaf<T>),
    Branch(Branch<T>),
}

#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
struct Leaf<T> {
    pub value: T,
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

#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
struct Branch<T> {
    pub value: T,
    #[builder(default)]
    pub children: Vec<Branch<T>>,
}
