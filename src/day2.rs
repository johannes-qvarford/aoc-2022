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

    fn beats(&self) -> Rps {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }

    fn beaten_by(&self) -> Rps {
        match self {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
            Rps::Scissors => Rps::Rock,
        }
    }

    fn parse(c: char) -> Rps {
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
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Choice {
    X,
    Y,
    Z
}

impl Choice {
    fn parse(c: char) -> Choice {
        match c {
            'X' => Choice::X,
            'Y' => Choice::Y,
            'Z' => Choice::Z,
            _ => {
                println!("Character is {c}");
                unimplemented!()
            }
        }
    }
}

impl From<Choice> for Rps {
    fn from(c: Choice) -> Self {
        match c {
            Choice::X => Rps::Rock,
            Choice::Y => Rps::Paper,
            Choice::Z => Rps::Scissors,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Match {
    player: Choice,
    opponent: Rps,
}

impl Match {
    fn star1_total_score(&self) -> i32 {
        let player_rps: Rps = self.player.into();
        let round_score = 
            if player_rps.beats() == self.opponent { 6 }
            else if self.opponent.beats() == player_rps { 0 }
            else { 3 };
        player_rps.shape_score() + round_score
    }

    fn star2_total_score(&self) -> i32 {
        match self.player {
            Choice::X => 0 + self.opponent.beats().shape_score(),
            Choice::Y => 3 + self.opponent.shape_score(),
            Choice::Z => 6 + self.opponent.beaten_by().shape_score(),
        }
    }
}

type Res<'a, T> = IResult<&'a str, T>;

fn rps_player_parser(input: &str) -> Res<Choice> {
    let (rem, c) = one_of("XYZ")(input)?;
    Ok((rem, Choice::parse(c)))
}

fn rps_opponent_parser(input: &str) -> Res<Rps> {
    let (rem, c) = one_of("ABC")(input)?;
    Ok((rem, Rps::parse(c)))
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
    matches.into_iter().map(|m| m.star1_total_score()).sum()
}

fn day2_2(matches: Vec<Match>) -> i32 {
    matches.into_iter().map(|m| m.star2_total_score()).sum()
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
                    player: Choice::Z,
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
                        player: Choice::Y,
                        opponent: Rps::Rock
                    },
                    Match {
                        player: Choice::Z,
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
                        player: Choice::Y,
                        opponent: Rps::Rock
                    },
                    Match {
                        player: Choice::Z,
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
        assert_eq!(day2_1(parser(&read_input(2)).unwrap().1), 11386)
    }

    #[test]
    fn day2_2_example() {
        assert_eq!(day2_2(parser("A Y\nB X\nC Z\n").unwrap().1), 12)
    }

    #[test]
    fn day2_2_test() {
        assert_eq!(day2_2(parser(&read_input(2)).unwrap().1), 13600)
    }
}
