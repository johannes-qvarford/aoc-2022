mod battle;
mod choice;
mod parser;
mod rps;

use battle::Match;

use crate::parser::parse;

fn part1(matches: Vec<Match>) -> i32 {
    matches.into_iter().map(|m| m.part1_total_score()).sum()
}

fn part2(matches: Vec<Match>) -> i32 {
    matches.into_iter().map(|m| m.part2_total_score()).sum()
}

const INPUT: &str = include_str!("_input");

fn main() {
    let parsed = parse(INPUT).unwrap().1;

    println!("part1: {}", part1(parsed.clone()));
    println!("part2: {}", part2(parsed));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(parse("A Y\nB X\nC Z\n").unwrap().1), 15)
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(parse(INPUT).unwrap().1), 11386)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(parse("A Y\nB X\nC Z\n").unwrap().1), 12)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(parse(INPUT).unwrap().1), 13600)
    }
}
