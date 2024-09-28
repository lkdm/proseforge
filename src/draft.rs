use crate::tree::Tree;

pub enum DraftKind {
    /// A RoughDraft is an unstructured usually first draft.
    /// The writer is encouraged to get words out on the page and not worry too much about perfecting details.
    RoughDraft,
    ShortStory,
    Anthology,
    Novel,
    NovelWithParts,
}

type DraftId = u32;

pub struct Draft {
    id: DraftId,
    kind: DraftKind,
    tree: Tree,
    title: String,
}
