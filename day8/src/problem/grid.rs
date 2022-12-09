use std::fmt::Display;

use std::ops::Add;
use std::ops::IndexMut;

use std::ops::Index;

use std::ops::Mul;
use std::vec::IntoIter;

#[derive(Copy, Clone)]
pub(crate) struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Position {
    fn flat_index(&self, side_length: i32) -> usize {
        usize::try_from(self.x + (self.y * side_length)).expect("grid index is 0 or positive")
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position { x, y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Position;

    fn mul(self, rhs: i32) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub(crate) struct Grid<T> {
    side_length: i32,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub(crate) fn new(side_length: i32, data: Vec<T>) -> Grid<T> {
        Grid { side_length, data }
    }

    pub(crate) fn side_length(&self) -> i32 {
        self.side_length
    }

    pub(crate) fn into_iter(self) -> IntoIter<T> {
        self.data.into_iter()
    }
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self.data[index.flat_index(self.side_length)]
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.data[index.flat_index(self.side_length)]
    }
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.side_length {
            for j in 0..self.side_length {
                let s = if self[(i, j).into()] { 'X' } else { 'O' };
                s.fmt(f)?
            }
            '\n'.fmt(f)?
        }
        Ok(())
    }
}

impl Display for Grid<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.side_length {
            for j in 0..self.side_length {
                let s = self[(i, j).into()];
                s.fmt(f)?
            }
            '\n'.fmt(f)?
        }
        Ok(())
    }
}
