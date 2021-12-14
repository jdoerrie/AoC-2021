use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lines_iter = stdin.lock().lines();
    let template = lines_iter.next().unwrap().unwrap();
    assert!(template.find('_').is_none());
    let padded_template = format!("_{}_", template);
    let rules = lines_iter
        .filter_map(|line| {
            line.unwrap().split_once(" -> ").map(|(lhs, rhs)| {
                (
                    (lhs.chars().next().unwrap(), lhs.chars().nth(1).unwrap()),
                    rhs.chars().next().unwrap(),
                )
            })
        })
        .collect::<HashMap<_, _>>();

    let mut counts = HashMap::new();
    for window in padded_template
        .chars()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
    {
        *counts.entry((window[0], window[1])).or_insert(0) += 1;
    }

    for _ in 0..10 {
        let mut new_counts = HashMap::new();
        for (pair, count) in counts {
            match rules.get(&pair) {
                Some(&c) => {
                    *new_counts.entry((pair.0, c)).or_insert(0) += count;
                    *new_counts.entry((c, pair.1)).or_insert(0) += count;
                }
                None => *new_counts.entry(pair).or_insert(0) += count,
            }
        }

        counts = new_counts;
    }

    let mut char_counts = HashMap::new();
    for (pair, count) in counts {
        *char_counts.entry(pair.0).or_insert(0) += count;
        *char_counts.entry(pair.1).or_insert(0) += count;
    }
    char_counts.remove(&'_');

    let (min_count, max_count) = char_counts.iter().fold(
        (u64::MAX, u64::MIN),
        |(min_count, max_count), (_, &count)| (min(min_count, count), max(max_count, count)),
    );

    println!(
        "({} - {}) / 2 = {}",
        max_count,
        min_count,
        (max_count - min_count) / 2
    );
}
