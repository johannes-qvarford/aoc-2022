mod pair;
mod section_assignment;

use color_eyre::eyre::{Context, Result};
use pair::Pair;

trait IntoIteratorExt: IntoIterator {
    fn flat_map_eyre_results<U, M, F>(
        self,
        mut msg: M,
        mut f: F,
    ) -> Result<<Vec<U> as IntoIterator>::IntoIter>
    where
        Self: Sized,
        M: FnMut(usize) -> String,
        F: FnMut(Self::Item) -> Result<U>,
    {
        let r: Result<Vec<U>> = self
            .into_iter()
            .enumerate()
            .map(|(index, v)| f(v).wrap_err_with(|| msg(index)))
            .collect();
        let vec = r?;
        Ok(vec.into_iter())
    }
}

impl<T> IntoIteratorExt for T where T: IntoIterator {}

fn parse(s: &str) -> Result<Vec<Pair>> {
    let pairs = s
        .lines()
        .flat_map_eyre_results(
            |i| format!("parsing a pair at index {i}"),
            |line| line.parse::<Pair>(),
        )
        .wrap_err("parsing pairs")?
        .collect();
    Ok(pairs)
}

fn part1(pairs: &[Pair]) -> usize {
    let overlaps = pairs
        .iter()
        .filter(|pair| pair.one_range_fully_contains_the_other())
        .count();
    overlaps
}

fn part2(pairs: &[Pair]) -> usize {
    let overlaps = pairs.iter().filter(|pair| pair.overlaps()).count();
    overlaps
}

const INPUT: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE: &str = include_str!("_example");

fn main() -> Result<()> {
    color_eyre::install()?;
    let parsed = parse(INPUT)?;

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE).unwrap()), 2)
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(&parse(INPUT).unwrap()), 550)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), 4)
    }
}
