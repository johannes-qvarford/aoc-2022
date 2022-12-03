use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{eof, iterator},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

use crate::{battle::Match, choice::Choice, rps::Rps};

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

pub fn parse(input: &str) -> IResult<&str, Vec<Match>> {
    let terminator = eof.or(tag("\n"));
    let mut it = iterator(input, terminated(match_parser, terminator));
    let matches: Vec<Match> = it.collect();
    let (remaining_input, ()) = it.finish()?;
    Ok((remaining_input, matches))
}

#[cfg(test)]
mod test {
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
            parse("A Y\nB Z"),
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
            parse("A Y\nB Z\n"),
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
}
