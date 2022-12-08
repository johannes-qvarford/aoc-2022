use std::fmt::Display;

pub(crate) mod space;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub(crate) struct DirectoryName(pub(crate) String);

impl Display for DirectoryName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub(crate) const PARENT_DIRECTORY: &str = "..";

#[derive(Clone, Debug)]
pub(crate) enum Node {
    Directory(DirectoryName),
    File(space::Space),
}

#[derive(Clone, Debug)]
pub(crate) enum Interaction {
    Cd(DirectoryName),
    Ls(Vec<Node>),
}

#[derive(Clone)]
pub(crate) struct DirectoryContent {
    pub(crate) computed_space: space::Space,
    pub(crate) uncomputed_directories: Vec<DirectoryName>,
}
