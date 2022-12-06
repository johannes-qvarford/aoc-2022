use crate::{
    allocation_tracker::StdoutTracker,
    problem::{parse, part1, part2, INPUT_STR},
};
use color_eyre::eyre::Result;
use tracking_allocator::AllocationRegistry;

mod allocation_tracker;
mod problem;

fn main() -> Result<()> {
    color_eyre::install()?;
    AllocationRegistry::set_global_tracker(StdoutTracker)
        .expect("no other global tracker should be set yet");

    let parsed = parse(INPUT_STR)?;

    println!("part1: {:?}", part1(&parsed));

    AllocationRegistry::enable_tracking();
    let r2 = part2::<14>(&parsed);
    AllocationRegistry::disable_tracking();
    println!("part2: {}", r2);

    Ok(())
}
