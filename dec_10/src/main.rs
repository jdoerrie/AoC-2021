use std::io::BufRead;

enum Result {
    Complete,
    Incomplete(String),
    Corrupt(char),
}

fn check_line(line: &str) -> Result {
    let get_matching = |c| match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => c,
    };

    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if get_matching(c) != stack.pop().unwrap_or(c) {
                    return Result::Corrupt(c);
                }
            }
            _ => continue,
        }
    }

    if stack.is_empty() {
        Result::Complete
    } else {
        Result::Incomplete(stack.iter().rev().map(|&c| get_matching(c)).collect())
    }
}

fn main() {
    let mut scores = std::io::stdin()
        .lock()
        .lines()
        .filter_map(|line| match check_line(&line.unwrap()) {
            Result::Incomplete(str) => Some(str),
            _ => None,
        })
        .map(|str| {
            str.chars().fold(0usize, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    }
            })
        })
        .collect::<Vec<_>>();
    let num_scores = scores.len();
    println!("{}", scores.select_nth_unstable(num_scores / 2).1);
}
