use std::io::BufRead;

fn main() {
    let bit_counts: Vec<(i32, i32)> =
        std::io::stdin()
            .lock()
            .lines()
            .fold(vec![], |mut acc, line| {
                for (i, c) in line.unwrap().chars().rev().enumerate() {
                    if i >= acc.len() {
                        acc.push((0, 0));
                    }
                    if c == '0' {
                        acc[i].0 += 1
                    } else {
                        acc[i].1 += 1
                    }
                }
                acc
            });

    let gamma: usize = bit_counts
        .iter()
        .enumerate()
        .map(|(bit, counts)| ((counts.0 < counts.1) as usize) << bit)
        .sum();
    let epsilon: usize = bit_counts
        .iter()
        .enumerate()
        .map(|(bit, counts)| ((counts.0 > counts.1) as usize) << bit)
        .sum();
    println!("{}", gamma * epsilon);
}
