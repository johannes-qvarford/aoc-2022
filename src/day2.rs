use anyhow::{Context, Result};
use nom::*;
use nom::{
    bytes::complete::tag,
    character::complete::*,
    combinator::{eof, iterator},
    error::ErrorKind,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn shape_score(&self) -> i32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    fn parse_opponent(c: char) -> Rps {
        match c {
            'A' => Rps::Rock,
            'B' => Rps::Paper,
            'C' => Rps::Scissors,
            _ => {
                println!("Character is {c}");
                unimplemented!()
            }
        }
    }

    fn parse_player(c: char) -> Rps {
        match c {
            'X' => Rps::Rock,
            'Y' => Rps::Paper,
            'Z' => Rps::Scissors,
            _ => {
                println!("Character is {c}");
                unimplemented!()
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Match {
    player: Rps,
    opponent: Rps,
}

impl Match {
    fn total_score(&self) -> i32 {
        let round_score = match (self.player, self.opponent) {
            (Rps::Rock, Rps::Scissors) | (Rps::Paper, Rps::Rock) | (Rps::Scissors, Rps::Paper) => 6,
            (a, b) if a == b => 3,
            _ => 0,
        };
        self.player.shape_score() + round_score
    }
}

type Res<'a, T> = IResult<&'a str, T>;

fn rps_player_parser(input: &str) -> Res<Rps> {
    let (rem, c) = one_of("XYZ")(input)?;
    Ok((rem, Rps::parse_player(c)))
}

fn rps_opponent_parser(input: &str) -> Res<Rps> {
    let (rem, c) = one_of("ABC")(input)?;
    Ok((rem, Rps::parse_opponent(c)))
}

fn match_parser(input: &str) -> IResult<&str, Match> {
    let pairs = separated_pair(rps_opponent_parser, tag(" "), rps_player_parser);
    nom::combinator::map(pairs, |(a, b)| Match {
        player: b,
        opponent: a,
    })(input)
}

fn parser(input: &str) -> IResult<&str, Vec<Match>> {
    let terminator = eof.or(tag("\n"));
    let mut it = iterator(input, terminated(match_parser, terminator));
    let matches: Vec<Match> = it.collect();
    let (remaining_input, ()) = it.finish()?;
    Ok((remaining_input, matches))
}

fn day2_1(matches: Vec<Match>) -> i32 {
    matches.into_iter().map(|m| m.total_score()).sum()
}

#[cfg(test)]
mod test {
    use crate::base::read_input;

    use super::*;

    #[test]
    fn parse_match_test() {
        assert_eq!(
            match_parser("A Z"),
            Ok((
                "",
                Match {
                    player: Rps::Scissors,
                    opponent: Rps::Rock
                }
            ))
        )
    }

    #[test]
    fn parse_matches_test() {
        assert_eq!(
            parser("A Y\nB Z"),
            Ok((
                "",
                vec![
                    Match {
                        player: Rps::Paper,
                        opponent: Rps::Rock
                    },
                    Match {
                        player: Rps::Scissors,
                        opponent: Rps::Paper
                    }
                ]
            ))
        )
    }

    #[test]
    fn parse_matches_test_newline() {
        assert_eq!(
            parser("A Y\nB Z\n"),
            Ok((
                "",
                vec![
                    Match {
                        player: Rps::Paper,
                        opponent: Rps::Rock
                    },
                    Match {
                        player: Rps::Scissors,
                        opponent: Rps::Paper
                    }
                ]
            ))
        )
    }

    #[test]
    fn day2_1_example() {
        assert_eq!(day2_1(parser("A Y\nB X\nC Z\n").unwrap().1), 15)
    }

    #[test]
    fn day2_1_test() {
        assert_eq!(day2_1(parser(&read_input(2)).unwrap().1), 11378)
    }
}
