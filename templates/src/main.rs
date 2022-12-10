mod input;
mod problem;

use crate::{
    input::INPUT_STR,
    problem::{parse, part1, part2},
};

fn main() {
    let parsed = parse(INPUT_STR);

    println!("part1: {:?}", part1(&parsed.clone()));
    println!("part2: {:?}", part2(&parsed));
}
