use bon::bon;
use slotmap::{Key, SlotMap};
use std::fmt::Debug;

type Id = usize;

struct Node;

struct Tree {
    children: Vec<Node>;
}

/// Binder
///
/// A binder is a root-level directory that dictates rules for its conents.
/// For example, a Novel's immediate children may be chapters, and compile to a particular format.
enum Binder {
    Novel(Tree),
    NovelWithParts(Tree),
    ShortStory(Tree),
    Notes(Tree),
    Bin(Tree),
}

impl Binder {
    /// Compiles a binder to an output, depending on its structure
    fn compile(&self) -> Option<&str> {
        match self {
            Binder::Novel(_) | Binder::NovelWithParts(_) | Binder::ShortStory(_) => {
                todo!("Not implemented")
            }
            Binder::Notes(_) | Binder::Bin(_) => {
                None // Noop
            }
        }
    }
}
