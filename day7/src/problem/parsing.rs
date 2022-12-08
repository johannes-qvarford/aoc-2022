use nom::branch::alt;

use nom::error::{ErrorKind, ParseError};
use nom::multi::many1;
use nom::{multi::separated_list1, IResult};

use nom::sequence::separated_pair;

use nom::bytes::complete::tag;

use nom::sequence::preceded;

use nom::character::complete::i32;

use nom::combinator::map;

use std::str;

use super::domain::{DirectoryName, Interaction, Node, Space};

pub(crate) type MyResult<'a, T> = IResult<&'a str, T>;

fn filename_character(s: &str) -> MyResult<char> {
    match s.chars().next() {
        Option::Some(c) if c.is_alphabetic() || c == '.' || c == '/' => Ok((&s[1..], c)),
        Option::Some(_c) => Err(nom::Err::Error(ParseError::from_error_kind(
            s,
            ErrorKind::Alpha.to_owned(),
        ))),
        Option::None => Err(nom::Err::Error(ParseError::from_error_kind(
            s,
            ErrorKind::Eof,
        ))),
    }
}

pub(crate) fn parse_directory_name(s: &str) -> MyResult<DirectoryName> {
    let array = many1(filename_character);
    map(array, |chars| DirectoryName(chars.iter().collect()))(s)
}

pub(crate) fn parse_file(i: &str) -> MyResult<Node> {
    let filename = many1(filename_character);
    let pair = separated_pair(i32, tag(" "), filename);
    map(pair, |(space, _)| Node::File(Space(space)))(i)
}

pub(crate) fn parse_directory(i: &str) -> MyResult<Node> {
    let result = preceded(tag("dir "), parse_directory_name);
    map(result, |dn| Node::Directory(dn))(i)
}

pub(crate) fn parse_node(i: &str) -> MyResult<Node> {
    alt((parse_directory, parse_file))(i)
}

pub(crate) fn parse_cd(i: &str) -> MyResult<Interaction> {
    let cd = map(parse_directory_name, |dn| Interaction::Cd(dn));
    preceded(tag("cd "), cd)(i)
}

pub(crate) fn parse_ls(i: &str) -> MyResult<Interaction> {
    let node_lines = separated_list1(tag("\n"), parse_node);
    let (rest, nodes) = preceded(tag("ls\n"), node_lines)(i)?;
    Ok((rest, Interaction::Ls(nodes)))
}

pub(crate) fn parse_interaction(i: &str) -> MyResult<Interaction> {
    alt((parse_ls, parse_cd))(i)
}

pub(crate) fn parse_interactions(i: &str) -> MyResult<Vec<Interaction>> {
    separated_list1(tag("\n"), preceded(tag("$ "), parse_interaction))(i)
}
