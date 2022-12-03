use anyhow::{anyhow, Context, Result};
use aoc22::Timer;
use std::collections::HashSet;

fn main() -> Result<()> {
    let timer = Timer::tick();
    let data: Vec<_> = include_str!("../../data/day03.txt").lines().collect();

    let answer_1: u32 = data
        .iter()
        .map(|&bag| compartments(bag))
        .map(|(c1, c2)| {
            common_element(&[c1, c2])
                .context(format!("no common element for '{}' and '{}'", c1, c2))
                .and_then(value_of)
        })
        .collect::<Result<Vec<u32>>>()?
        .iter()
        .sum();
    println!("{}", answer_1);

    let answer_2: u32 = data
        .chunks(3)
        .into_iter()
        .map(|bags| {
            common_element(bags)
                .context(format!("no common element for '{:?}'", bags))
                .and_then(value_of)
        })
        .collect::<Result<Vec<u32>>>()?
        .iter()
        .sum();
    println!("{}", answer_2);

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

fn compartments(bag: &str) -> (&str, &str) {
    bag.split_at(bag.len() / 2)
}

fn common_element(bags: &[&str]) -> Option<char> {
    let all_chars: HashSet<_> = ('a'..='z').chain('A'..='Z').collect();
    bags.iter()
        .fold(all_chars, |acc, &bag| {
            let bag: HashSet<_> = bag.chars().collect();
            acc.intersection(&bag).copied().collect()
        })
        .into_iter()
        .next()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_of() {
        assert_eq!(value_of('p').unwrap(), 16_u32);
        assert_eq!(value_of('L').unwrap(), 38_u32);
    }

    #[test]
    fn test_common_element() {
        let a = "pmCn";
        let b = "Czyx";
        let c = "abCd";
        assert_eq!(Some('C'), common_element(&[a, b, c]));
    }
}
