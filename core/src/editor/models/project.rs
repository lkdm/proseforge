use std::collections::HashMap;

use crate::data::Id;

use super::{
    component::{Component, ComponentId},
    content::{Content, ContentId},
    Title,
};

pub struct ProjectId(Id);

/// A project is a collection of components and content.
///
struct Project {
    id: ProjectId,
    title: Title,
    root_components: Vec<ComponentId>,
    components_by_id: HashMap<ComponentId, Component>,
    content_by_id: HashMap<ContentId, Content>,
}
