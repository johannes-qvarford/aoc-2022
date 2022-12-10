use color_eyre::eyre::Result;
use itertools::Itertools;

use self::vector::{Vector, DOWN, LEFT, ORIGIN, RIGHT, UP};

mod vector;

pub(crate) type Input = Vec<Movement>;

pub(crate) type Output = usize;

#[derive(Clone, Copy)]
pub(crate) struct Movement {
    direction: Vector,
    length: i32,
}

fn parse_direction(s: &str) -> Vector {
    match s {
        "L" => LEFT,
        "R" => RIGHT,
        "U" => UP,
        "D" => DOWN,
        _ => unreachable!("Direction is always one of L, R, U or D."),
    }
}

pub(crate) fn parse(s: &str) -> Result<Input> {
    let instructions = s.lines().map(|line| {
        let mut split = line.split(' ');
        match (split.next(), split.next()) {
            (Some(direction), Some(length)) => Movement {
                direction: parse_direction(direction),
                length: length.parse::<i32>().expect("length should be a number"),
            },
            _ => unreachable!("Line should be a direction and length separated by a space"),
        }
    });
    Ok(instructions.collect_vec())
}

mod snake;

pub(crate) fn part1(input: &Input) -> Output {
    let (tails, _) = input.iter().fold(
        (vec![ORIGIN], snake::Snake::new()),
        |(mut tails, snake), &movement| {
            let new_snake = (0..movement.length).fold(snake, |snake, _: i32| {
                let new_snake = snake.mov(movement.direction);
                //println!("Snake:\n{}", new_snake);
                tails.push(new_snake.tail);
                new_snake
            });
            (tails, new_snake)
        },
    );

    tails.into_iter().unique().count()
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
        assert_eq!(part1(&parse(EXAMPLE_STR).unwrap()), 13)
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1(&parse(INPUT_STR).unwrap()), 6494)
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
