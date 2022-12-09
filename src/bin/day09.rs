use anyhow::{anyhow, Context, Error, Result};
use aoc22::Timer;
use std::{collections::HashSet, str::FromStr};

fn main() -> Result<()> {
    let timer = Timer::tick();

    let mut rope = Rope::default();
    let mut tail_positions = HashSet::new();
    tail_positions.insert(rope.tail());

    for line in include_str!("../../data/day09.txt").lines() {
        let motion: Motion = line.parse()?;

        for _ in 0..motion.steps {
            rope.step(&motion.dir);
            tail_positions.insert(rope.tail());
        }
    }
    println!("{}", tail_positions.len());

    timer.tock();
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default)]
pub struct Rope {
    head: Position,
    tail: Position,
}

impl Rope {
    pub fn head(&self) -> Position {
        self.head
    }

    pub fn tail(&self) -> Position {
        self.tail
    }

    pub fn step(&mut self, dir: &Direction) {
        match dir {
            /*
               Consider all the possible initial tail positions given a
               head position (remembering H and T can overlap in center)
               and then consider the distance after the move
                 T T T      1 2 3
                 T H T  --> 0 1 2
                 T T T      1 2 3
               Then the only time the tail needs to move is if its in the
               rightmost column (delta-x >= 2). In all of those cases, the
               tail moves to the same row as the head and advances left
               by one.
            */
            Direction::Left => {
                self.head.x -= 1;
                if self.head.x.abs_diff(self.tail.x) >= 2 {
                    self.tail.y = self.head.y;
                    self.tail.x -= 1;
                }
            }
            // Same analysis as above, just mirrored
            Direction::Right => {
                self.head.x += 1;
                if self.head.x.abs_diff(self.tail.x) >= 2 {
                    self.tail.y = self.head.y;
                    self.tail.x += 1;
                }
            }
            // Same analysis as above, with x/y reversed
            Direction::Up => {
                self.head.y += 1;
                if self.head.y.abs_diff(self.tail.y) >= 2 {
                    self.tail.x = self.head.x;
                    self.tail.y += 1;
                }
            }
            Direction::Down => {
                self.head.y -= 1;
                if self.head.y.abs_diff(self.tail.y) >= 2 {
                    self.tail.x = self.head.x;
                    self.tail.y -= 1;
                }
            }
        }
    }
}

pub struct Motion {
    pub steps: u32,
    pub dir: Direction,
}

impl Motion {
    pub const fn new(steps: u32, dir: Direction) -> Self {
        Self { steps, dir }
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Motion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s
            .split_once(' ')
            .context(format!("'{}' is not a valid movement", s))?;

        let steps: u32 = steps.parse()?;
        let dir = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(anyhow!("'{}' not a valid direction", dir)),
        };

        Ok(Self { steps, dir })
    }
}
