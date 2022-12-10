mod domain;
mod parsing;

use itertools::Itertools;

use self::domain::InstructionIteratorExt;
pub(crate) use self::domain::{Input, Output};
pub(crate) use self::parsing::parse;

pub(crate) fn part1(_input: &Input) -> Output {
    let states = _input.iter().cloned().execute_instructions().collect_vec();

    states
        .into_iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .take(6)
        .inspect(|(index, state)| println!("State {:03} = {}", index, *state))
        .map(|(index, state)| state.value_during_cycle() * (index as i32 + 1))
        .sum()
}

pub(crate) fn part2(_input: &Input) -> Output {
    42
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::{EXAMPLE_STR, INPUT_STR, MINI_EXAMPLE_STR};

    #[test]
    fn part1_test_mini_example() {
        let values = parse(MINI_EXAMPLE_STR)
            .into_iter()
            .cloned()
            .execute_instructions()
            .map(|state| state.value_during_cycle())
            .collect_vec();
        assert_eq!(values, vec![1, 1, 1, 4, 4])
    }

    #[test]
    fn part1_test_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR)), 13140)
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1(&parse(INPUT_STR)), 13220)
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
