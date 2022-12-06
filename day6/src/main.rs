use crate::problem::{parse, part1, part2, INPUT_STR};
use color_eyre::eyre::Result;

mod problem;

fn main() -> Result<()> {
    color_eyre::install()?;
    let parsed = parse(INPUT_STR)?;

    println!("part1: {:?}", part1(&parsed));
    println!("part2: {:?}", part2(&parsed, 14));
    Ok(())
}
