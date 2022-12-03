use color_eyre::eyre::{Context, Result};
use rucksack::Rucksack;

mod compartment;
mod item_type;
mod rucksack;

fn parse(s: &str) -> Result<Vec<Rucksack>> {
    let result: Result<Vec<_>, _> = s
        .lines()
        .map(|line| line.try_into().wrap_err("parsing a rucksack"))
        .collect();
    let rucksacks = result.wrap_err("parsing rucksacks")?;
    Ok(rucksacks)
}

fn part1(rucksacks: Vec<Rucksack>) -> i32 {
    rucksacks
        .into_iter()
        .map(|rucksack| rucksack.misplaced_item().priority())
        .sum()
}

const INPUT: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE: &str = include_str!("_example");

fn main() -> Result<()> {
    color_eyre::install()?;
    let parsed = parse(INPUT)?;

    println!("part1: {}", part1(parsed.clone()));
    //println!("part2: {}", part2(parsed));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example_one_lowercase_character() {
        assert_eq!(part1(parse("ahai").unwrap()), 1)
    }

    #[test]
    fn part1_example_multiple_copies_of_the_same_character() {
        assert_eq!(part1(parse("aaahaaij").unwrap()), 1)
    }

    #[test]
    fn part1_example_uppercase_character() {
        assert_eq!(part1(parse("AHAI").unwrap()), 27)
    }

    #[test]
    fn part1_example_one_line() {
        assert_eq!(part1(parse("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap()), 16)
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(parse(EXAMPLE).unwrap()), 157)
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(parse(INPUT).unwrap()), 8176)
    }
}
