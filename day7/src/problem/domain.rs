pub(crate) mod space;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub(crate) struct DirectoryName(String);

impl DirectoryName {
    pub(crate) fn is_parent(&self) -> bool {
        self.0 == ".."
    }

    pub(crate) fn is_root(&self) -> bool {
        self.0 == "/"
    }
}

impl From<String> for DirectoryName {
    fn from(s: String) -> Self {
        DirectoryName(s)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Path(Vec<DirectoryName>);

impl Path {
    pub(crate) fn add_segment(&mut self, name: DirectoryName) {
        if name.is_parent() {
            self.0.pop();
        } else if !name.is_root() {
            self.0.push(name);
        }
    }

    pub(crate) fn root() -> Path {
        Self(vec![])
    }

    pub(crate) fn with_segment(&self, name: DirectoryName) -> Path {
        let mut new = self.clone();
        new.add_segment(name);
        new
    }
}

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
    pub(crate) uncomputed_directories: Vec<Path>,
}
