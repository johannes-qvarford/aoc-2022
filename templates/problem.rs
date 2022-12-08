use arraydeque::{ArrayDeque, Wrapping};
use color_eyre::eyre::Result;
use itertools::Itertools;

pub(crate) type Input = i32;

pub(crate) type Output = usize;

pub(crate) fn parse(s: &str) -> Result<Input> {
    Ok(42)
}

pub(crate) fn part1(input: &Input) -> Output {
    Ok(42)
}

pub(crate) fn part2(input: &Input) -> Output {
    Ok(42)
}

pub(crate) const INPUT_STR: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE_STR: &str = include_str!("_example");

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn part1_test_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR).unwrap()), 42)
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1(&parse(INPUT_STR).unwrap()), 42)
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
