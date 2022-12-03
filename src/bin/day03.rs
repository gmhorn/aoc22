use std::collections::HashSet;

use anyhow::{anyhow, Result};
use aoc22::Timer;

fn main() -> Result<()> {
    let timer = Timer::tick();

    let data: Vec<_> = include_str!("../../data/day03.txt").lines().collect();

    // let data = vec![
    //     "vJrwpWtwJgWrhcsFMMfFFhFp",
    //     "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
    //     "PmmdzqPrVvPwwTWBwg",
    //     "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
    //     "ttgJtRGJQctTZtZT",
    //     "CrZsJsPPZsGzwwsLwLmpwMDw",
    // ];

    let answer_1: u32 = data
        .iter()
        .map(|&bag| compartments(bag))
        .map(|(c1, c2)| {
            common_element(c1, c2)
                .ok_or(anyhow!("no common element for '{}' and '{}'", c1, c2))
                .and_then(value_of)
        })
        .collect::<Result<Vec<u32>>>()?
        .iter()
        .sum();
    println!("{}", answer_1);

    
    timer.tock();
    Ok(())
}

fn value_of(c: char) -> Result<u32> {
    match c {
        'a'..='z' => Ok((c as u32) - ('a' as u32) + 1),
        'A'..='Z' => Ok((c as u32) - ('A' as u32) + 27),
        _ => Err(anyhow!("'{}' not a valid char", c)),
    }
}

fn common_element(bag1: &str, bag2: &str) -> Option<char> {
    let bag1: HashSet<char> = bag1.chars().collect();
    let bag2: HashSet<char> = bag2.chars().collect();
    bag1.intersection(&bag2).next().copied()
}

fn compartments(bag: &str) -> (&str, &str) {
    bag.split_at(bag.len() / 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_of() {
        assert_eq!(value_of('p').unwrap(), 16_u32);
        assert_eq!(value_of('L').unwrap(), 38_u32);
    }
}
