use color_eyre::eyre::{Context, Result};
use group::Group;
use item_type::ItemType;
use rucksack::Rucksack;

mod compartment;
mod group;
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

fn part2(rucksacks: Vec<Rucksack>) -> Result<i32> {
    let results: Result<Vec<ItemType>> = rucksacks
        .chunks_exact(3)
        .map(|chunk| {
            let a = [chunk[0].clone(), chunk[1].clone(), chunk[2].clone()];
            Group { rucksacks: a }
        })
        .map(|group| group.badge().wrap_err("extracting badge from rucksack"))
        .collect();

    let sum = results
        .wrap_err("grouping rucksacks")?
        .iter()
        .map(|item_type| item_type.priority())
        .sum();

    Ok(sum)
}

const INPUT: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE: &str = include_str!("_example");

fn main() -> Result<()> {
    color_eyre::install()?;
    let parsed = parse(INPUT)?;

    println!("part1: {}", part1(parsed.clone()));
    println!("part2: {}", part2(parsed)?);
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(parse(EXAMPLE).unwrap()).unwrap(), 70)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(parse(INPUT).unwrap()).unwrap(), 2689)
    }
}
