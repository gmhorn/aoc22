use aoc22::Timer;
use std::collections::HashSet;

fn main() {
    let timer = Timer::tick();
    let data = include_str!("../../data/day06.txt");

    println!("{}", find_unique(data, 4).expect("could not find substr"));
    println!("{}", find_unique(data, 14).expect("could not find substr"));

    timer.tock();
}

fn find_unique(msg: &str, len: usize) -> Option<usize> {
    for idx in len..msg.len() {
        let substr = &msg[idx - len..idx];
        if substr.chars().collect::<HashSet<_>>().len() == len {
            return Some(idx);
        }
    }
    None
}
