use std::io::BufRead;

fn score_line(line: &str) -> u64 {
    let matches = |lhs, rhs| {
        (lhs == '(' && rhs == ')')
            || (lhs == '[' && rhs == ']')
            || (lhs == '{' && rhs == '}')
            || (lhs == '<' && rhs == '>')
    };

    let get_points = |c| match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    };
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if !matches(stack.pop().unwrap_or('_'), c) {
                    return get_points(c);
                }
            }
            _ => continue,
        }
    }
    0
}

fn main() {
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .map(|line| score_line(&line.unwrap()))
            .sum::<u64>()
    );
}
