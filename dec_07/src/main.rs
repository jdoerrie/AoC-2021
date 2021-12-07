use std::cmp::min;
use std::io::BufRead;

fn main() {
    let crabs: Vec<i32> = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|crab| crab.parse().unwrap())
        .collect();
    let mean_lo = crabs.iter().sum::<i32>() / crabs.len() as i32;
    let mean_hi = mean_lo + 1;
    let dist = |lhs: i32, rhs: i32| {
        let abs = (lhs - rhs).abs();
        abs * (abs + 1) / 2
    };
    println!(
        "{}",
        min(
            crabs.iter().map(|&crab| dist(crab, mean_lo)).sum::<i32>(),
            crabs.iter().map(|&crab| dist(crab, mean_hi)).sum::<i32>(),
        )
    );
}
