use anyhow::Result;
use aoc22::Timer;

fn main() -> Result<()> {
    let timer = Timer::tick();

    let _data = include_str!("../../data/day03.txt");

    timer.tock();
    Ok(())
}
