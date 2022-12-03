use anyhow::Result;
use aoc22::Timer;

fn main() -> Result<()> {
    let timer = Timer::tick();

    let lines: Vec<_> = include_str!("../../data/day01.txt").lines().collect();
    let mut cals: Vec<u32> = lines
        .split(|l| l.is_empty())
        .map(|snacks| {
            snacks
                .iter()
                .map(|snack| snack.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    cals.sort();
    cals.reverse();

    println!("{}", cals.first().unwrap());
    println!("{}", cals.iter().take(3).sum::<u32>());

    timer.tock();
    Ok(())
}
