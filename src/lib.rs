use binder::Binder;
use bon::bon;
use slotmap::{Key, SlotMap};
use std::{collections::BTreeMap, fmt::Debug};
use tree::Node;
pub mod binder;
pub mod draft;
pub mod tree;

struct Project {
    binder: Binder,
    content: BTreeMap<u32, String>,
}
