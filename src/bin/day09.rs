use anyhow::{anyhow, Context, Error, Result};
use aoc22::Timer;
use std::ops::Deref;
use std::{collections::HashSet, str::FromStr};

fn main() -> Result<()> {
    let timer = Timer::tick();

    let motions: Vec<Motion> = include_str!("../../data/day09.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_>>()?;

    let mut rope = Rope3::<2>::new();
    let mut tail_positions = HashSet::new();
    tail_positions.insert(rope.last().copied().unwrap());

    for line in include_str!("../../data/day09.txt").lines() {
        let motion: Motion = line.parse()?;

        for _ in 0..motion.steps {
            rope.step(&motion.dir);
            tail_positions.insert(rope.last().copied().unwrap());
        }
    }
    println!("{}", tail_positions.len());
    println!("{}", count_tail_positions::<2>(&motions));

    timer.tock();
    Ok(())
}

fn count_tail_positions<const KNOTS: usize>(motions: &[Motion]) -> usize {
    let mut rope = Rope3::<KNOTS>::new();
    let mut tail_positions = HashSet::new();

    tail_positions.insert(rope.last().copied().unwrap());

    for motion in motions {
        for _ in 0..motion.steps {
            rope.step(&motion.dir);
            tail_positions.insert(rope.last().copied().unwrap());
        }
    }

    tail_positions.len()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Rope3<const N: usize>([Position; N]);

impl<const N: usize> Rope3<N> {
    pub fn new() -> Self {
        Self([Position::default(); N])
    }

    pub fn tail(&self) -> Option<&Position> {
        self.0.last()
    }

    pub fn step(&mut self, dir: &Direction) {
        self.0.get_mut(0).map(|head| {
            match dir {
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
            };
        });

        self.0.iter_mut().reduce(|lead, follow| {
            Self::update_follower(lead, follow);
            follow
        });
    }

    fn update_follower(lead: &Position, follow: &mut Position) {
        let dx = lead.x - follow.x;
        let dy = lead.y - follow.y;

        match (dx, dy) {
            // If lead and follow are touching, no update needed.
            (-1..=1, -1..=1) => {}
            // Handle lead +- 2 along same rank as follow
            (-2 | 2, 0) => follow.x += dx / 2,
            (0, -2 | 2) => follow.y += dy / 2,
            // Handle L-shaped difference
            (-2 | 2, -1 | 1) => {
                follow.y = lead.y;
                follow.x += dx / 2;
            }
            (-1 | 1, -2 | 2) => {
                follow.x = lead.x;
                follow.y += dy / 2;
            }
            // Handle large diagonal jump. This can only happen in N>2 ropes, if the lead itself
            // had an L-shaped difference with it's lead.
            (-2 | 2, -2 | 2) => {
                follow.x += dx / 2;
                follow.y += dy / 2;
            }
            // Anything else is invalid! Panic sloppily.
            (dx, dy) => panic!("can't update follower for delta ({}, {})", dx, dy),
        }
    }
}

impl<const N: usize> Deref for Rope3<N> {
    type Target = [Position; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
