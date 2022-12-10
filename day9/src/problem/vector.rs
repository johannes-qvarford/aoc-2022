use std::fmt::Display;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Vector {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub(crate) const ORIGIN: Vector = Vector { x: 0, y: 0 };
pub(crate) const RIGHT: Vector = Vector { x: 1, y: 0 };
pub(crate) const UP: Vector = Vector { x: 0, y: 1 };
pub(crate) const LEFT: Vector = Vector { x: -1, y: 0 };
pub(crate) const DOWN: Vector = Vector { x: 0, y: -1 };

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Vector {
    fn from((x, y): (i32, i32)) -> Self {
        Vector { x, y }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
