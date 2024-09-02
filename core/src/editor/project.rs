use derive_more::derive::From;

use crate::types::Id;

use super::Title;

pub mod models;
pub mod ports;
pub mod services;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ProjectKind {
    /// A environment that is used for practicing the craft of writing.
    Practice,

    /// A single short story.
    ShortStory,

    /// A collection of short stories set in the same universe, or with a common theme.
    Anthology,

    /// A single novel.
    Novel,

    /// A novel with multiple parts.
    NovelWithParts,

    /// A collection of novels set in the same universe, or with a common theme.
    Series,

    /// A project used for journaling.
    Journal,
}

/// A project is a collection of components and content.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Project {
    id: Id,
    title: Title,
    kind: ProjectKind,
}
