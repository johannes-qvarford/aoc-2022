use std::{
    cmp::max,
    fmt::Display,
    ops::{Index, IndexMut},
    vec::IntoIter,
};

use color_eyre::eyre::Result;
use itertools::Itertools;

pub(crate) type Input = String;

pub(crate) type Output = i32;

pub(crate) fn parse(s: &str) -> Result<Input> {
    Ok(s.to_owned())
}

type Position = (i32, i32);

struct Grid<T> {
    side_length: i32,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn new(side_length: i32, data: Vec<T>) -> Grid<T> {
        Grid { side_length, data }
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

pub(crate) fn part1(input: &Input) -> Output {
    let width: i32 = input
        .find('\n')
        .expect("There are multiple lines in the input")
        .try_into()
        .expect("Should fit in 32 bits.");
    let grid_vec: Vec<i32> = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            c.to_digit(10)
                .expect("Only characters left are digits")
                .try_into()
                .expect("Digits are never negative")
        })
        .collect_vec();
    let size: i32 = grid_vec
        .len()
        .try_into()
        .expect("Grid length should fit in 31 bits");
    let height = size / width;

    let grid = Grid::new(width, grid_vec);

    let visibility_vec = vec![false; size as usize];
    let mut visibility = Grid::new(width, visibility_vec);

    let scans = [
        ((0, 0), (1, 0), (0, 1), width),
        ((0, 0), (0, 1), (1, 0), height),
        ((width - 1, 0), (-1, 0), (0, 1), width),
        ((0, height - 1), (0, -1), (1, 0), height),
    ];

    // println!("original grid:\n{}", grid);

    for (_, ((x, y), (jx, jy), (ix, iy), len)) in scans.into_iter().enumerate() {
        for i in 0..len {
            let mut highest = -1;
            for j in 0..len {
                let cx = x + (jx * j) + (ix * i);
                let cy = y + (jy * j) + (iy * i);
                let gi = (cx, cy);

                // println!("Grid index: ({}, {})", gi.0, gi.1);
                let new_highest = max(highest, grid[gi]);

                if new_highest > highest {
                    highest = new_highest;
                    visibility[gi] = true;
                }
            }
        }
        // println!("visibility grid scan ({}):\n{}", scan_index, visibility);
    }

    // println!("visibility grid afterwards:\n{}", visibility);

    visibility
        .into_iter()
        .filter(|&b| b)
        .count()
        .try_into()
        .expect("Number of visible trees fits in 31 bits.")
}

pub(crate) fn part2(_input: &Input) -> Output {
    42
}

pub(crate) const INPUT_STR: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE_STR: &str = include_str!("_example");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR).unwrap()), 21)
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1(&parse(INPUT_STR).unwrap()), 1827)
    }

    #[test]
    fn part2_test_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR).unwrap()), 42)
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2(&parse(INPUT_STR).unwrap()), 42)
    }
}
