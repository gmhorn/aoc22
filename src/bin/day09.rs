use anyhow::{Context, Result};
use aoc22::Timer;

fn main() -> Result<()> {
    let timer = Timer::tick();

    let data = include_str!("../../data/day09.txt");

    timer.tock();
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

pub enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}