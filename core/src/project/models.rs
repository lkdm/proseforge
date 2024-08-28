use std::{collections::HashMap, path::PathBuf};

use strum_macros::EnumString;

use crate::{
    data::{Id, Timestamp},
    editor::models::ContentId,
};

struct Content {
    id: ContentId,
    title: Title,
    data: String,

    created_at: Timestamp,
    saved_at: Option<Timestamp>,
    modified_at: Option<Timestamp>,
}

struct Title(String);
struct ComponentId(Id);

#[derive(strum_macros::EnumString)]
enum ComponentKind {
    Draft,
    Part,
    Chapter,
    Scene,
    Character,
    Location,
    Note,
    Research,
    Outline,
}

struct Component {
    id: ComponentId,
    kind: ComponentKind,

    document: Option<ContentId>,
    summary: Option<ContentId>,
    note: Option<ContentId>,

    children: Vec<ComponentId>,
    parent: Option<ComponentId>,
}

struct Project {
    root_components: Vec<ComponentId>,
    components_by_id: HashMap<ComponentId, Component>,
    content_by_id: HashMap<ContentId, Content>,
}
