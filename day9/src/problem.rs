use itertools::Itertools;

use self::{
    domain::{Input, Movement, Output},
    vector::ORIGIN,
};

mod vector;

mod domain;
mod parsing;
mod snake;

pub(crate) use self::parsing::parse;

fn snake_walk(input: &[Movement], len: i32) -> usize {
    let (tails, _) = input.iter().fold(
        (vec![ORIGIN], snake::Snake::of_length(len)),
        |(mut tails, snake), &movement| {
            let new_snake = (0..movement.length).fold(snake, |snake, _: i32| {
                let new_snake = snake.mov(movement.direction);
                // println!("Snake:\n{}", new_snake);
                tails.push(new_snake.tail());
                new_snake
            });
            (tails, new_snake)
        },
    );
    tails.into_iter().unique().count()
}

pub(crate) fn part1(input: &Input) -> Output {
    snake_walk(input, 2)
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
