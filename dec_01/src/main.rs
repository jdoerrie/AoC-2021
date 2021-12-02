use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let depths: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let depth_windows: Vec<i32> = depths[..]
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    let counts: u32 = depth_windows
        .iter()
        .zip(depth_windows.iter().skip(1))
        .map(|(curr, next)| (curr < next) as u32)
        .sum();
    println!("{}", counts);
}
