use color_eyre::eyre::Result;

pub(crate) type Input = ();

pub(crate) type Output = ();

pub(crate) fn parse(s: &str) -> Result<Input> {
    Ok(())
}

pub(crate) fn part1(input: &Input) -> Output {
}

pub(crate) fn part2(input: &Input) -> Output {
}

pub(crate) const INPUT_STR: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE_STR: &str = include_str!("_example");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR).unwrap()), todo!())
    }

    #[test]
    #[ignore]
    fn part1_test() {
        assert_eq!(part1(&parse(INPUT_STR).unwrap()), todo!())
    }

    #[test]
    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR).unwrap()), todo!())
    }

    #[test]
    #[ignore]
    fn part2_test() {
        assert_eq!(part2(&parse(EXAMPLE_STR).unwrap()), todo!())
    }
}
