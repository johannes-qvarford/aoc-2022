use crate::problem::{parse, part1, part2, INPUT_STR};
use color_eyre::eyre::Result;

mod problem;

fn main() -> Result<()> {
    let parsed = parse(INPUT_STR)?;

    println!("part1: {:?}", part1(&parsed));
    println!("part2: {:?}", part2(&parsed));

    Ok(())
}
