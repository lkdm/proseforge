use std::path::PathBuf;

use strum_macros::EnumString;

use crate::data::{ChapterId, DocumentId, DraftId, NoteId, PartId, ProjectId};

pub struct Title(String);

enum DraftComponent {
    Scene(SceneId),
    Chapter(ChapterId),
    Part(PartId),
}
pub struct Draft {
    id: DraftId,
    title: Title,
    components: Vec<DraftComponent>,
    note: Option<NoteId>,
}

enum PartComponent {
    Chapter(ChapterId),
    Scene(SceneId),
}

pub struct Part {
    id: PartId,
    title: Title,
    components: Vec<PartComponent>,
    note: Option<NoteId>,
}

enum ChapterComponent {
    Scene(SceneId),
}

pub struct Chapter {
    id: ChapterId,
    title: Title,
    components: Vec<ChapterComponent>,
    note: Option<NoteId>,
}

pub struct Scene {
    id: SceneId,
    title: Title,
    note: Option<NoteId>,
}

#[derive(EnumString)]
pub enum ProjectKind {
    Novel,
    ShortStory,
    Screenplay,
    StagePlay,
    Poem,
    Article,
    Essay,
    BlogPost,
    Thesis,
    Other,
}

pub struct Note {
    id: NoteId,
    document: DocumentId,
}

pub struct Scene {
    id: SceneId,
    document: DocumentId,
    note: NoteId,
    title: Title,
}

pub struct Project {
    id: ProjectId,
    kind: ProjectKind,

    drafts: Vec<DraftId>,
    notes: Vec<NoteId>,

    path: Option<PathBuf>,
}
