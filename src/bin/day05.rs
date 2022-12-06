use anyhow::{Context, Result};
use aoc22::Timer;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn main() -> Result<()> {
    let timer = Timer::tick();

    let (setup, ops) = include_str!("../../data/day05.txt")
        .split_once("\n\n")
        .context("Input not in expected format")?;

    let mut stacks: Stacks = setup.parse()?;

    let ops: Vec<Op> = ops
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_>>()?;

    for op in ops {
        stacks.apply(op)?;
    }

    println!("{}", stacks.tops()?);

    timer.tock();
    Ok(())
}

#[derive(Debug, Default)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    pub fn add(&mut self, stack: &[char]) {
        self.0.push(stack.into());
    }

    pub fn apply(&mut self, op: Op) -> Result<()> {
        for _ in 0..op.count {
            let krate = self
                .0
                .get_mut(op.from - 1)
                .context(format!("Invalid from index '{}'", op.from))?
                .pop()
                .context(format!("Stack '{}' is exhausted", op.from))?;

            self.0
                .get_mut(op.to - 1)
                .context(format!("Invalid to index '{}'", op.to))?
                .push(krate);
        }

        Ok(())
    }

    pub fn tops(self) -> Result<String> {
        let mut res = String::new();

        for stack in self.0.iter() {
            let top = stack.get(stack.len() - 1).context("empty stack!")?;
            res.push(*top);
        }

        Ok(res)
    }
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = Stacks::default();

        let lines: Vec<_> = s.lines().rev().collect();
        let (&names, crates) = lines.split_first().context("invalid input")?;

        for (idx, char) in names.chars().enumerate() {
            if char.is_ascii_alphanumeric() {
                let stack = crates
                    .iter()
                    .filter_map(|&c| c.chars().skip(idx).next())
                    .filter(|c| c.is_ascii_alphanumeric())
                    .collect::<Vec<_>>();
                stacks.add(&stack);
            }
        }
        Ok(stacks)
    }
}

#[derive(Debug, PartialEq)]
struct Op {
    count: usize,
    from: usize,
    to: usize,
}

impl Op {
    pub const fn new(count: usize, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let caps = RE
            .captures(s)
            .context(format!("'{}' does not match regex", s))?;

        let count: usize = caps[1].parse()?;
        let from: usize = caps[2].parse()?;
        let to: usize = caps[3].parse()?;

        Ok(Op::new(count, from, to))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_op() {
        let line = "move 12 from 4 to 7";
        let op: Op = line.parse().unwrap();

        assert_eq!(op, Op::new(12, 4, 7));
    }
}
