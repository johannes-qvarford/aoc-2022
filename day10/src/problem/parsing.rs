use itertools::Itertools;

use super::{
    domain::{Addx, Instruction},
    Input,
};

pub(crate) fn parse(_s: &str) -> Input {
    _s.lines()
        .map(|line| {
            let mut split = line.split(' ');
            match (split.next(), split.next()) {
                (Some(_addx), Some(amount)) => Instruction::Addx(Addx(
                    amount
                        .parse()
                        .expect("addx instruction contains amount to add."),
                )),
                (Some(_noop), None) => Instruction::Noop,
                _ => unreachable!("Instruction has at least one word"),
            }
        })
        .collect_vec()
        .into_boxed_slice()
}
