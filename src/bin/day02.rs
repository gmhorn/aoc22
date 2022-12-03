use anyhow::{anyhow, Context, Result};
use aoc22::Timer;

fn main() -> Result<()> {
    let timer = Timer::tick();

    let rounds: Vec<_> = include_str!("../../data/day02.txt").lines().collect();
    let score_1: u32 = rounds
        .iter()
        .map(|&round| score_strategy_one(round))
        .collect::<Result<Vec<u32>>>()?
        .iter()
        .sum();
    println!("{}", score_1);

    let score_2: u32 = rounds
        .iter()
        .map(|&round| score_strategy_two(round))
        .collect::<Result<Vec<u32>>>()?
        .iter()
        .sum();
    println!("{}", score_2);

    timer.tock();
    Ok(())
}

fn score_strategy_one(round: &str) -> Result<u32> {
    let (you, me) = round
        .split_once(' ')
        .context(format!("contex'{}' is not a valid round", round))?;
    let you = match you {
        "A" => Ok(Shape::Rock),
        "B" => Ok(Shape::Paper),
        "C" => Ok(Shape::Scissors),
        _ => Err(anyhow!("'{}' is not a valid hand", you)),
    }?;
    let me = match me {
        "X" => Ok(Shape::Rock),
        "Y" => Ok(Shape::Paper),
        "Z" => Ok(Shape::Scissors),
        _ => Err(anyhow!("'{}' is not a valid hand", me)),
    }?;

    Ok((me.outcome(&you) as u32) + (me as u32))
}

fn score_strategy_two(round: &str) -> Result<u32> {
    let (you, outcome) = round
        .split_once(' ')
        .context(format!("contex'{}' is not a valid round", round))?;
    let you = match you {
        "A" => Ok(Shape::Rock),
        "B" => Ok(Shape::Paper),
        "C" => Ok(Shape::Scissors),
        _ => Err(anyhow!("'{}' is not a valid hand", you)),
    }?;
    let outcome = match outcome {
        "X" => Ok(Outcome::Lose),
        "Y" => Ok(Outcome::Draw),
        "Z" => Ok(Outcome::Win),
        _ => Err(anyhow!("'{}' is not a valid outcome", outcome)),
    }?;

    let me = you.find_outcome(outcome);
    Ok((outcome as u32) + (me as u32))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn beats(&self, other: &Self) -> bool {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => true,
            (Shape::Paper, Shape::Rock) => true,
            (Shape::Scissors, Shape::Paper) => true,
            (_, _) => false,
        }
    }

    pub fn outcome(&self, other: &Self) -> Outcome {
        if self.beats(other) {
            Outcome::Win
        } else if other.beats(self) {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }

    pub fn find_outcome(&self, outcome: Outcome) -> Self {
        *[Self::Rock, Self::Paper, Self::Scissors]
            .iter()
            .find(|other| other.outcome(self) == outcome)
            .expect("to find a hand that results in the given outcome")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}
