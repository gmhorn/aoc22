use anyhow::{anyhow, Context, Result};
use std::str::FromStr;

fn main() -> Result<()> {
    let lines: Vec<_> = include_str!("../../data/day02.txt").lines().collect();
    let score: Result<Vec<u32>> = lines.iter().map(|&l| score_round_one(l)).collect();
    println!("{}", score.unwrap().iter().sum::<u32>());

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn beats(&self, other: Self) -> bool {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => true,
            (Shape::Paper, Shape::Rock) => true,
            (Shape::Scissors, Shape::Paper) => true,
            (_, _) => false,
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(anyhow!("'{}' is not a valid value", s)),
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn from_round(you: Shape, me: Shape) -> Self {
        let i_win = me.beats(you);
        let u_win = you.beats(me);

        match (i_win, u_win) {
            (true, false) => Self::Win,
            (false, true) => Self::Lose,
            (_, _) => Self::Draw,
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("'{}' is not a valid outcome", s)),
        }
    }
}

pub fn score_round_one(round: &str) -> Result<u32> {
    let (you, me) = round
        .split_once(' ')
        .ok_or(anyhow!("'{}' is not a valid round", round))?;
    let you: Shape = you.parse()?;
    let me: Shape = me.parse()?;
    let outcome = Outcome::from_round(you, me);

    Ok(me.value() + outcome.value())
}
