use std::convert::TryInto;
use std::io::{self, BufRead};

#[allow(clippy::unusual_byte_groupings)]
const BINGO_MASKS: &[u32] = &[
    // rows
    0b_00000_00000_00000_00000_11111u32,
    0b_00000_00000_00000_11111_00000u32,
    0b_00000_00000_11111_00000_00000u32,
    0b_00000_11111_00000_00000_00000u32,
    0b_11111_00000_00000_00000_00000u32,
    // columns
    0b_00001_00001_00001_00001_00001u32,
    0b_00010_00010_00010_00010_00010u32,
    0b_00100_00100_00100_00100_00100u32,
    0b_01000_01000_01000_01000_01000u32,
    0b_10000_10000_10000_10000_10000u32,
];

struct Board {
    numbers: [i32; 25],
    marked_mask: u32,
}

impl Board {
    fn sum_unmarked(&self) -> i32 {
        self.numbers
            .iter()
            .enumerate()
            .map(|(i, &n)| {
                if self.marked_mask & (1u32 << i) == 0 {
                    n
                } else {
                    0
                }
            })
            .sum()
    }

    fn mark(&mut self, num: i32) -> bool {
        let marked_mask: u32 = self
            .numbers
            .iter()
            .enumerate()
            .map(|(i, &n)| if n == num { 1u32 << i } else { 0 })
            .sum();
        self.marked_mask |= marked_mask;
        marked_mask != 0
    }

    fn is_bingo(&self) -> bool {
        BINGO_MASKS
            .iter()
            .any(|&mask| mask & self.marked_mask == mask)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines();
    let bingo_numbers: Vec<i32> = lines_iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut bingo_boards = Vec::new();
    let mut bingo_buffer = Vec::new();
    for line in lines_iter {
        bingo_buffer.extend(
            line.unwrap()
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap()),
        );
        if bingo_buffer.len() == 25 {
            bingo_boards.push(Board {
                numbers: bingo_buffer.try_into().unwrap(),
                marked_mask: 0,
            });
            bingo_buffer = Vec::new();
        }
    }

    for num in bingo_numbers {
        for board in &mut bingo_boards {
            board.mark(num);
            if board.is_bingo() {
                println!("{}", num * board.sum_unmarked());
                return;
            }
        }
    }
}
