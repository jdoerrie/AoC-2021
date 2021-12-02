use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let depths: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let counts: u32 = depths
        .iter()
        .zip(depths.iter().skip(1))
        .map(|(curr, next)| (curr < next) as u32)
        .sum();
    println!("{}", counts);
}
