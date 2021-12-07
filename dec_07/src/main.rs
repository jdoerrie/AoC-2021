use std::io::BufRead;

fn main() {
    let mut crabs: Vec<i32> = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|crab| crab.parse().unwrap())
        .collect();
    let num_crabs = crabs.len();
    let median = *crabs.select_nth_unstable(num_crabs / 2).1;
    println!(
        "{}",
        crabs.iter().map(|crab| (crab - median).abs()).sum::<i32>()
    );
}
