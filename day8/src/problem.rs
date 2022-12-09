use std::cmp::max;

use color_eyre::eyre::Result;
use itertools::Itertools;

use self::grid::{Grid, Position};

pub(crate) type Input = Grid<i32>;

pub(crate) type Output = i32;

pub(crate) fn parse(s: &str) -> Result<Input> {
    let width: i32 = s
        .find('\n')
        .expect("There are multiple lines in the input")
        .try_into()
        .expect("Should fit in 32 bits.");
    let grid_vec: Vec<i32> = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            c.to_digit(10)
                .expect("Only characters left are digits")
                .try_into()
                .expect("Digits are never negative")
        })
        .collect_vec();

    let grid = Grid::new(width, grid_vec);

    Ok(grid)
}

mod grid;

pub(crate) fn part1(input: &Input) -> Output {
    let side_length = input.side_length();

    let visibility_vec = vec![false; input.size()];
    let mut visibility = grid::Grid::new(side_length, visibility_vec);

    fn p(x: i32, y: i32) -> Position {
        Position::from((x, y))
    }
    let scans = [
        (p(0, 0), p(1, 0), p(0, 1), side_length),
        (p(0, 0), p(0, 1), p(1, 0), side_length),
        (p(side_length - 1, 0), p(-1, 0), p(0, 1), side_length),
        (p(0, side_length - 1), p(0, -1), p(1, 0), side_length),
    ];

    for (_, (start, jp, ip, len)) in scans.into_iter().enumerate() {
        for i in 0..len {
            let mut highest = -1;
            for j in 0..len {
                let current: Position = start + (jp * j) + (ip * i);

                let new_highest = max(highest, input[current]);

                if new_highest > highest {
                    highest = new_highest;
                    visibility[current] = true;
                }
            }
        }
    }

    visibility
        .into_iter()
        .filter(|&b| b)
        .count()
        .try_into()
        .expect("Number of visible trees fits in 31 bits.")
}

pub(crate) fn part2(input: &Input) -> Output {
    let side_length = input.side_length();
    fn p(x: i32, y: i32) -> Position {
        Position::from((x, y))
    }
    let scans = [p(1, 0), p(0, 1), p(-1, 0), p(0, -1)];

    let treehouse_vec = vec![0; input.size()];
    let mut treehouse = Grid::new(side_length, treehouse_vec);

    for i in 0..side_length {
        for j in 0..side_length {
            let viewpoint: Position = (i, j).into();
            let viewpoint_height = input[viewpoint];
            let mut scenic_score = 1;
            for scan in scans {
                'scan: for k in 1..side_length {
                    let tree_position = viewpoint + (scan * k);
                    if !input.in_bounds(tree_position) {
                        scenic_score *= k - 1;
                        break 'scan;
                    }
                    let tree_height = input[tree_position];

                    if tree_height >= viewpoint_height {
                        scenic_score *= k;
                        break 'scan;
                    }
                }
            }
            treehouse[viewpoint] = scenic_score;
        }
    }

    treehouse
        .into_iter()
        .max()
        .expect("At least one tree in the grid")
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
        assert_eq!(part2(&parse(EXAMPLE_STR).unwrap()), 8)
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2(&parse(INPUT_STR).unwrap()), 335580)
    }
}
