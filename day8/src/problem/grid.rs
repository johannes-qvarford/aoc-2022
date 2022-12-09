use std::fmt::Display;

use std::ops::IndexMut;

use std::ops::Index;

use std::vec::IntoIter;

pub(crate) type Position = (i32, i32);

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
        &self.data[usize::try_from(index.0 + (index.1 * self.side_length))
            .expect("grid index is 0 or positive")]
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.data[usize::try_from(index.0 + (index.1 * self.side_length))
            .expect("grid index is 0 or positive")]
    }
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.side_length {
            for j in 0..self.side_length {
                let s = if self[(i, j)] { 'X' } else { 'O' };
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
                let s = self[(i, j)];
                s.fmt(f)?
            }
            '\n'.fmt(f)?
        }
        Ok(())
    }
}
