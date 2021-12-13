use std::cmp::max;
use std::collections::HashSet;
use std::io::BufRead;

enum Ins {
    Point(usize, usize),
    FoldX(usize),
    FoldY(usize),
}

fn parse_line(line: &str) -> Option<Ins> {
    if let Some((lhs, rhs)) = line.split_once(',') {
        return Some(Ins::Point(lhs.parse().unwrap(), rhs.parse().unwrap()));
    }

    match line.split_once('=') {
        Some((lhs, rhs)) => match lhs.chars().last().unwrap() {
            'x' => Some(Ins::FoldX(rhs.parse().unwrap())),
            'y' => Some(Ins::FoldY(rhs.parse().unwrap())),
            _ => None,
        },
        _ => None,
    }
}

fn main() {
    let mut points = HashSet::new();
    for ins in std::io::stdin()
        .lock()
        .lines()
        .filter_map(|line| parse_line(&line.unwrap()))
    {
        match ins {
            Ins::Point(x, y) => {
                points.insert((x, y));
            }
            Ins::FoldX(l) => {
                points = points
                    .iter()
                    .map(|&(x, y)| (if x < l { x } else { 2 * l - x }, y))
                    .collect();
            }
            Ins::FoldY(l) => {
                points = points
                    .iter()
                    .map(|&(x, y)| (x, if y < l { y } else { 2 * l - y }))
                    .collect();
            }
        }
    }

    let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), &(x, y)| {
        (max(max_x, x), max(max_y, y))
    });

    for y in 0..=max_y {
        println!(
            "{}",
            (0..=max_x)
                .map(|x| if points.contains(&(x, y)) { '#' } else { '.' })
                .collect::<String>()
        );
    }
}
