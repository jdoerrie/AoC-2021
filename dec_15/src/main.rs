use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::BufRead;

fn main() {
    let costs = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let m = costs.len();
    let n = costs[0].len();

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), (0, 0)));
    let mut dists = HashMap::new();
    while let Some((Reverse(cost), pos)) = heap.pop() {
        if pos == (5 * m - 1, 5 * n - 1) {
            println!("{}", cost);
            break;
        }

        if dists.contains_key(&pos) {
            continue;
        }
        dists.insert(pos, cost);

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = (pos.0 as isize + dx) as usize;
            let y = (pos.1 as isize + dy) as usize;
            if x >= 5 * m || y >= 5 * n {
                continue;
            }

            let curr_cost = (costs[x % m][y % n] + ((x / m) as u32) + ((y / n) as u32) - 1) % 9 + 1;
            heap.push((Reverse(cost + curr_cost), (x, y)));
        }
    }
}
