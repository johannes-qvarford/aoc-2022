mod domain;
mod parsing;

pub(crate) use self::domain::{Input, Output};
pub(crate) use self::parsing::parse;

pub(crate) fn part1(_input: &Input) -> Output {
    42
}

pub(crate) fn part2(_input: &Input) -> Output {
    42
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::{EXAMPLE_STR, INPUT_STR};

    #[test]
    fn part1_test_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR)), 42)
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1(&parse(INPUT_STR)), 42)
    }

    #[test]
    fn part2_test_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR)), 42)
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2(&parse(INPUT_STR)), 42)
    }
}
