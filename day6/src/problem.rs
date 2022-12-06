use color_eyre::eyre::Result;
use itertools::Itertools;

pub(crate) type Input = String;

pub(crate) type Output = usize;

pub(crate) fn parse(s: &str) -> Result<Input> {
    Ok(s.to_owned())
}

pub(crate) fn part1(input: &Input) -> Output {
    let entry = input.chars().tuple_windows::<(char, char, char, char)>()
        .enumerate()
        .find(|&(_, (a, b, c, d))| [a, b, c, d].into_iter().all_unique())
        .expect("There should be at least one unique sequence of 4 chars");
    entry.0 + 4
}

pub(crate) fn part2(input: &Input) -> Output {
    10
}

pub(crate) const INPUT_STR: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE_STR: &str = include_str!("_example");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR).unwrap()), 7)
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(&parse(INPUT_STR).unwrap()), 1892)
    }

    #[test]
    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR).unwrap()), todo!())
    }

    #[test]
    #[ignore]
    fn part2_test() {
        assert_eq!(part2(&parse(INPUT_STR).unwrap()), Default::default())
    }
}
