mod iterator_ext;
mod rearrangement;
mod simple;
mod stacks;

use std::collections::VecDeque;

use color_eyre::eyre::{Context, Result};
use iterator_ext::IteratorNextExt;
use itertools::Itertools;
use thiserror::Error;

use self::{iterator_ext::IntoIteratorExt, rearrangement::Rearrangement, stacks::Stacks};

#[derive(Clone)]
pub(crate) struct Input {
    stacks: Stacks,
    rearrangements: Vec<Rearrangement>,
}

pub(crate) type Output = String;

#[derive(Error, Debug)]
enum InputParsingError {
    #[error("Could not parse drawing+instructions seperated by double newlines.")]
    NotADrawingAndInstructions,
}

pub(crate) fn parse(s: &str) -> Result<Input> {
    let mut split = s.split("\n\n");
    if let (Some(drawing), Some(instructions), None) = split.next3() {
        let input = Input {
            stacks: drawing.parse()?,
            rearrangements: instructions
                .lines()
                .flat_map_eyre_results(|i| format!("parsing instruction {i}"), |line| line.parse())
                .wrap_err("parsing instructions")?
                .collect_vec(),
        };
        Ok(input)
    } else {
        Err(InputParsingError::NotADrawingAndInstructions.into())
    }
}

pub(crate) fn part1(reference: &Input) -> Output {
    let mut input = (*reference).clone();

    for rearrangement in input.rearrangements {
        for _ in 0..rearrangement.amount {
            let from_stack = input
                .stacks
                .by_id_mut(rearrangement.from)
                .expect("stack ids for 'from' are not wrong");
            let crat = from_stack
                .pop_front()
                .expect("we never move from an empty stack");
            let to_stack = input
                .stacks
                .by_id_mut(rearrangement.to)
                .expect("stack ids for 'to' are not wrong");

            to_stack.push_front(crat);
        }
    }
    input.stacks.top_crates()
}

pub(crate) fn part2(reference: &Input) -> Output {
    let mut input = (*reference).clone();

    for rearrangement in input.rearrangements {
        let mut crane = VecDeque::new();

        for _ in 0..rearrangement.amount {
            let from_stack = input
                .stacks
                .by_id_mut(rearrangement.from)
                .expect("stack ids for 'from' are not wrong");
            let crat = from_stack
                .pop_front()
                .expect("we never move from an empty stack");

            crane.push_front(crat);
        }

        for _ in 0..rearrangement.amount {
            let to_stack = input
                .stacks
                .by_id_mut(rearrangement.to)
                .expect("stack ids for 'to' are not wrong");

            to_stack.push_front(
                crane
                    .pop_front()
                    .expect("We pop as many elements as we push"),
            );
        }
    }
    input.stacks.top_crates()
}

pub(crate) const INPUT_STR: &str = include_str!("_input");

#[cfg(test)]
const EXAMPLE_STR: &str = include_str!("_example");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR).unwrap()), "CMZ".to_owned())
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(&parse(INPUT_STR).unwrap()), "NTWZZWHFV".to_owned())
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR).unwrap()), "MCD".to_owned())
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&parse(INPUT_STR).unwrap()), "BRZGFVBTJ".to_owned())
    }
}
