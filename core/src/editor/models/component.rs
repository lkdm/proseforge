use crate::data::Id;

use super::content::ContentId;

pub struct ComponentId(Id);

#[derive(strum_macros::EnumString)]
pub enum ComponentKind {
    Draft,
    Part,
    Chapter,
    Scene,
    Character,
    Location,
    Note,
    Outline,
}

/// A component is a part of a project.
pub struct Component {
    id: ComponentId,
    kind: ComponentKind,
    children: Vec<ComponentId>,
    parent: Option<ComponentId>,

    summary: Option<ContentId>,
    document: Option<ContentId>,
    // TODO: comments, maybe by line-location, etc
    // comments: Vec<ContentId>,
}
