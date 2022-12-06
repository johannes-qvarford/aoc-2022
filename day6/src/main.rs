use color_eyre::eyre::Result;
use crate::problem::{parse, INPUT_STR, part1, part2};

mod problem;

fn main() -> Result<()> {
    color_eyre::install()?;
    let parsed = parse(INPUT_STR)?;

    println!("part1: {:?}", part1(&parsed));
    println!("part2: {:?}", part2(&parsed));
    Ok(())
}