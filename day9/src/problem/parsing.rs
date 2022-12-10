use super::{
    domain::Movement,
    vector::{Vector, DOWN, LEFT, RIGHT, UP},
    Input,
};

use color_eyre::eyre::Result;
use itertools::Itertools;

pub(crate) fn parse_direction(s: &str) -> Vector {
    match s {
        "L" => LEFT,
        "R" => RIGHT,
        "U" => UP,
        "D" => DOWN,
        _ => unreachable!("Direction is always one of L, R, U or D."),
    }
}

pub(crate) fn parse(s: &str) -> Result<Input> {
    let instructions = s.lines().map(|line| {
        let mut split = line.split(' ');
        match (split.next(), split.next()) {
            (Some(direction), Some(length)) => Movement {
                direction: parse_direction(direction),
                length: length.parse::<i32>().expect("length should be a number"),
            },
            _ => unreachable!("Line should be a direction and length separated by a space"),
        }
    });
    Ok(instructions.collect_vec())
}
