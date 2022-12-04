use anyhow::{Context, Result};
use aoc22::Timer;
use std::str::FromStr;

fn main() -> Result<()> {
    let timer = Timer::tick();
    let data = include_str!("../../data/day04.txt").lines();

    let answer_one = data
        .map(|line| {
            let (a, b) = line
                .split_once(',')
                .context(format!("'{}' is not a valid line", line))?;
            let a: Assignment = a.parse()?;
            let b: Assignment = b.parse()?;

            Ok((a, b))
        })
        .collect::<Result<Vec<_>>>()? // want to bail if any errors,
        .iter() // so need to collect to intermediary :(
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count();
    println!("{}", answer_one);

    timer.tock();
    Ok(())
}

struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .context(format!("'{}' is not a valid assignment", s))?;
        let start: u32 = start.parse()?;
        let end: u32 = end.parse()?;

        Ok(Assignment { start, end })
    }
}
