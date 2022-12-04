use anyhow::{Context, Result};
use aoc22::Timer;
use std::str::FromStr;

fn main() -> Result<()> {
    let timer = Timer::tick();

    let assignments: Vec<_> = include_str!("../../data/day04.txt")
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_once(',')
                .context(format!("'{}' is not a valid line", line))?;
            let a: Assignment = a.parse()?;
            let b: Assignment = b.parse()?;

            Ok((a, b))
        })
        .collect::<Result<_>>()?;

    let answer_one = assignments
        .iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count();
    println!("{}", answer_one);

    let answer_two = assignments
        .iter()
        .filter(|(a, b)| Assignment::overlaps(a, b))
        .count();
    println!("{}", answer_two);

    timer.tock();
    Ok(())
}

struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(a: &Self, b: &Self) -> bool {
        a.contains(b)
            || b.contains(a)
            || (a.start <= b.start && b.start <= a.end)
            || (b.start <= a.start && a.start <= b.end)
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

        Ok(Assignment::new(start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_overlap() {
        assert_eq!(true, Assignment::overlaps(&Assignment::new(5, 7), &Assignment::new(7, 9)));
        assert_eq!(true, Assignment::overlaps(&Assignment::new(2, 8), &Assignment::new(3, 7)));
        assert_eq!(true, Assignment::overlaps(&Assignment::new(6, 6), &Assignment::new(4, 6)));
        assert_eq!(true, Assignment::overlaps(&Assignment::new(2, 6), &Assignment::new(4, 8)));

        assert_eq!(true, Assignment::overlaps(&Assignment::new(7, 9), &Assignment::new(5, 7)));
        assert_eq!(true, Assignment::overlaps(&Assignment::new(3, 7), &Assignment::new(2, 8)));
        assert_eq!(true, Assignment::overlaps(&Assignment::new(4, 6), &Assignment::new(6, 6)));
        assert_eq!(true, Assignment::overlaps(&Assignment::new(4, 8), &Assignment::new(2, 6)));

        assert_eq!(false, Assignment::overlaps(&Assignment::new(2, 6), &Assignment::new(7, 8)));
        assert_eq!(false, Assignment::overlaps(&Assignment::new(7, 8), &Assignment::new(2, 6)));
    }
}
