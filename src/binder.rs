use crate::{draft::Draft, tree::Tree};

/// The binder keeps track of the project's documents, and structure.
pub struct Binder {
    binder_items: Vec<BinderItem>,
}

/// BinderItem
///
/// A binder is a root-level directory that dictates rules for its conents.
pub enum BinderItem {
    Draft(Draft),
    Notes(Tree),
    Bin(Tree),
}

impl BinderItem {
    /// Compiles a binder to an output, depending on its structure
    fn compile(&self) -> Option<&str> {
        match self {
            BinderItem::Draft(_) => {
                todo!("Not implemented")
            }
            BinderItem::Notes(_) | BinderItem::Bin(_) => {
                None // Noop
            }
        }
    }
    /// Whether the binder is compilable
    pub fn can_compile(&self) -> bool {
        match self {
            BinderItem::Bin(_) | BinderItem::Notes(_) => false,
            _ => true,
        }
    }
}
