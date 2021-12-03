use std::io::{self, BufRead};

struct Command {
    position: i32,
    depth: i32,
}

fn parse_command(line: &str) -> Command {
    let mut tokens = line.split(' ');
    let command = tokens.next().unwrap();
    let value: i32 = tokens.next().unwrap().parse().unwrap();
    match command {
        "forward" => Command {
            position: value,
            depth: 0,
        },
        "down" => Command {
            position: 0,
            depth: value,
        },
        "up" => Command {
            position: 0,
            depth: -value,
        },
        _ => Command {
            position: 0,
            depth: 0,
        },
    }
}

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .map(|line| parse_command(line.unwrap().as_str()))
        .fold(
            Command {
                position: 0,
                depth: 0,
            },
            |a, b| Command {
                position: a.position + b.position,
                depth: a.depth + b.depth,
            },
        );
    println!("{}", result.position * result.depth);
}
