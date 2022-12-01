use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("data/day1.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut cals: Vec<u32> = lines
        .split(|l| l.is_empty())
        .into_iter()
        .map(|snacks| {
            snacks
                .iter()
                .map(|snack| snack.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    cals.sort();
    cals.reverse();

    println!("{}", cals.first().unwrap());
    println!("{}", cals.iter().take(3).sum::<u32>());
}
