use std::fmt::Display;
use std::iter::Sum;

use std::ops::Add;

use std::ops::Neg;

use std::ops::AddAssign;
use std::ops::Sub;

#[derive(PartialEq, PartialOrd, Debug, Eq, Ord, Clone, Copy)]
pub(crate) struct Space(pub(crate) i32);
impl Space {
    pub(crate) fn bytes(bytes: i32) -> Space {
        Space(bytes)
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes", self.0)
    }
}

impl Neg for Space {
    type Output = Space;

    fn neg(self) -> Self::Output {
        Space(-self.0)
    }
}

impl AddAssign<Space> for Space {
    fn add_assign(&mut self, rhs: Space) {
        self.0 += rhs.0;
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

impl Sub<Space> for Space {
    type Output = Space;

    fn sub(self, rhs: Space) -> Self::Output {
        Space(self.0 - rhs.0)
    }
}