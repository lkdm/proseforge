use binder::Binder;
use bon::bon;
use serde::{Deserialize, Serialize};
use slotmap::{Key, SlotMap};
use std::{collections::BTreeMap, fmt::Debug};
use tree::Node;
pub mod binder;
pub mod draft;
pub mod tree;

#[derive(Serialize, Deserialize)]
struct Project {
    title: String,
    // drafts: Vec, // documents: BTreeMap<u32, String>,
}
