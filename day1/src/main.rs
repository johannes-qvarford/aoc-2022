#![allow(clippy::let_and_return)]

use anyhow::{Context, Result};

fn backpack_string_to_calorie_list(s: &str) -> Result<Vec<i32>> {
    let items = s
        .split('\n')
        .map(|row| row.parse::<i32>().context("Failed parsing calorie row"));

    items.collect()
}

type Backpack = Vec<i32>;
type Backpacks = Vec<Backpack>;

fn calorie_lists(backpacks: Backpacks) -> impl Iterator<Item = i32> {
    let calories = backpacks
        .into_iter()
        .map(|backpack| backpack.into_iter().sum());
    calories
}

fn parse(input: &str) -> Result<Backpacks> {
    let backpack_strings = input.split("\n\n");
    let backpacks_maybe: Result<Vec<Vec<i32>>> = backpack_strings
        .map(backpack_string_to_calorie_list)
        .collect();
    let backpacks = backpacks_maybe?;
    Ok(backpacks)
}

fn part1(backpacks: Backpacks) -> Result<i32> {
    let lists = calorie_lists(backpacks);
    Ok(lists.max().unwrap_or_default())
}

fn part2(backpacks: Backpacks) -> Result<i32> {
    let lists = calorie_lists(backpacks);
    let mut sorted = lists.collect::<Vec<_>>();
    sorted.sort_by(|a, b| b.cmp(a));
    let top_calories_sum = sorted.into_iter().take(3).sum();
    Ok(top_calories_sum)
}

const INPUT: &str = include_str!("_input");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(parse(INPUT).unwrap()).unwrap(), 73211);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(parse(INPUT).unwrap()).unwrap(), 213958);
    }
}

fn main() -> Result<()> {
    let parsed = parse(INPUT)?;

    println!("part1: {}", part1(parsed.clone())?);
    println!("part2: {}", part2(parsed)?);
    Ok(())
}
