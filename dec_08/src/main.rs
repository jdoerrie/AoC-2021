use std::collections::HashMap;
use std::io::BufRead;

fn decode(line: &str) -> usize {
    let (input, output) = line.split_once(" | ").unwrap();

    let lens: HashMap<_, _> = input.split(' ').map(|num| (num.len(), num)).collect();

    let one = lens.get(&2).unwrap();
    let seven = lens.get(&3).unwrap();
    let four = lens.get(&4).unwrap();
    let eight = lens.get(&7).unwrap();

    let mut counts = HashMap::new();
    for c in input.split(' ').flat_map(|num| num.chars()) {
        *counts.entry(c).or_insert(0) += 1;
    }

    let ee = *counts.iter().find(|(_, &c)| c == 4).unwrap().0;
    let bb = *counts.iter().find(|(_, &c)| c == 6).unwrap().0;
    let ff = *counts.iter().find(|(_, &c)| c == 9).unwrap().0;
    let cc = one.chars().find(|&c| c != ff).unwrap();
    let aa = seven.chars().find(|&c| c != cc && c != ff).unwrap();
    let dd = four.chars().find(|&c| ![bb, cc, ff].contains(&c)).unwrap();
    let gg = eight
        .chars()
        .find(|&c| ![aa, bb, cc, dd, ee, ff].contains(&c))
        .unwrap();

    let mut digits = [
        vec![aa, bb, cc, ee, ff, gg],
        vec![cc, ff],
        vec![aa, cc, dd, ee, gg],
        vec![aa, cc, dd, ff, gg],
        vec![bb, cc, dd, ff],
        vec![aa, bb, dd, ff, gg],
        vec![aa, bb, ee, dd, ff, gg],
        vec![aa, cc, ff],
        vec![aa, bb, cc, dd, ee, ff, gg],
        vec![aa, bb, cc, dd, ff, gg],
    ];

    for pattern in digits.iter_mut() {
        pattern.sort_unstable();
    }

    output
        .split(' ')
        .map(|num| {
            let mut chars: Vec<char> = num.chars().collect();
            chars.sort_unstable();
            digits.iter().position(|pattern| *pattern == chars).unwrap()
        })
        .fold(0, |accum, num| accum * 10 + num)
}

fn main() {
    let result: usize = std::io::stdin()
        .lock()
        .lines()
        .map(|line| decode(&line.unwrap()))
        .sum();
    println!("{}", result);
}
