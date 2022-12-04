mod pair;
mod section_assignment;

use color_eyre::eyre::{Context, Result};
use pair::Pair;

fn parse(s: &str) -> Result<Vec<Pair>> {
    let result: Result<Vec<_>, _> = s
        .lines()
        .enumerate()
        .map(|(index, line)| {
            line.parse()
                .wrap_err_with(|| format!("parsing a pair at index {index}"))
        })
        .collect();
    let pairs = result.wrap_err("parsing pairs")?;
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
