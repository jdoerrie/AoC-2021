use std::io::{self, BufRead};

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

struct Submarine {
    pos_x: i32,
    depth: i32,
    aim: i32,
}

fn parse_command(line: &str) -> Command {
    let mut tokens = line.split(' ');
    let command = tokens.next().unwrap();
    let x: i32 = tokens.next().unwrap().parse().unwrap();
    match command {
        "forward" => Command::Forward(x),
        "down" => Command::Down(x),
        "up" => Command::Up(x),
        _ => Command::Forward(0),
    }
}

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .map(|line| parse_command(line.unwrap().as_str()))
        .fold(
            Submarine {
                pos_x: 0,
                depth: 0,
                aim: 0,
            },
            |accum, command| match command {
                Command::Down(x) => Submarine {
                    aim: accum.aim + x,
                    ..accum
                },
                Command::Up(x) => Submarine {
                    aim: accum.aim - x,
                    ..accum
                },
                Command::Forward(x) => Submarine {
                    pos_x: accum.pos_x + x,
                    depth: accum.depth + accum.aim * x,
                    aim: accum.aim,
                },
            },
        );
    println!("{}", result.pos_x * result.depth);
}
