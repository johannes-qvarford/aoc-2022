use std::fmt::Display;
use std::iter::Sum;

use std::ops::Add;

use std::ops::AddAssign;
use std::ops::Neg;

#[derive(PartialEq, PartialOrd, Debug, Eq, Ord, Clone, Copy)]
pub(crate) struct Space(pub(crate) i32);

impl AddAssign<Space> for Space {
    fn add_assign(&mut self, rhs: Space) {
        self.0 += rhs.0;
    }
}

impl Neg for Space {
    type Output = Space;

    fn neg(self) -> Self::Output {
        Space(-self.0)
    }
}

impl Add<Space> for Space {
    type Output = Space;

    fn add(self, rhs: Space) -> Self::Output {
        Space(self.0 + rhs.0)
    }
}

impl Sum<Space> for Space {
    fn sum<I: Iterator<Item = Space>>(iter: I) -> Self {
        iter.fold(Space(0), Space::add)
    }
}

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
    File(Space),
}

#[derive(Clone, Debug)]
pub(crate) enum Interaction {
    Cd(DirectoryName),
    Ls(Vec<Node>),
}

#[derive(Clone)]
pub(crate) struct DirectoryContent {
    pub(crate) computed_space: Space,
    pub(crate) uncomputed_directories: Vec<DirectoryName>,
}
