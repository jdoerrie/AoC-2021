use std::io::BufRead;

#[derive(Debug)]
enum Number {
    Regular(u8),
    Pair(Box<(Number, Number)>),
}

enum Side {
    Left,
    Right,
}

fn add(num: &mut Number, n: u8, side: Side) {
    match num {
        Number::Regular(x) => *num = Number::Regular(*x + n),
        Number::Pair(pair) => add(
            match side {
                Side::Left => &mut pair.0,
                Side::Right => &mut pair.1,
            },
            n,
            side,
        ),
    }
}

fn explode_helper(num: &mut Number, depth: usize) -> Option<(u8, u8)> {
    match num {
        Number::Regular(_) => None,
        Number::Pair(pair) => match depth {
            0..=3 => {
                if let Some((x, y)) = explode_helper(&mut pair.0, depth + 1) {
                    add(&mut pair.1, y, Side::Left);
                    Some((x, 0))
                } else if let Some((x, y)) = explode_helper(&mut pair.1, depth + 1) {
                    add(&mut pair.0, x, Side::Right);
                    Some((0, y))
                } else {
                    None
                }
            }

            4 => match (&pair.0, &pair.1) {
                (&Number::Regular(x), &Number::Regular(y)) => {
                    *num = Number::Regular(0);
                    Some((x, y))
                }
                _ => None,
            },
            _ => None,
        },
    }
}

fn explode(num: &mut Number) -> bool {
    explode_helper(num, 0).is_some()
}

fn split(num: &mut Number) -> bool {
    match num {
        Number::Regular(0..=9) => false,
        Number::Regular(x) => {
            *num = Number::Pair(Box::new((
                Number::Regular(*x / 2),
                Number::Regular((*x + 1) / 2),
            )));
            true
        }
        Number::Pair(pair) => split(&mut pair.0) || split(&mut pair.1),
    }
}

fn plus(lhs: Number, rhs: Number) -> Number {
    reduce(Number::Pair(Box::new((lhs, rhs))))
}

fn reduce(mut num: Number) -> Number {
    loop {
        if explode(&mut num) {
            continue;
        }
        if split(&mut num) {
            continue;
        }
        break;
    }

    num
}

fn parse_number(data: &[u8]) -> Number {
    match data[0] {
        b'[' => {
            let mut depth = 0;
            let comma_pos = data[1..]
                .iter()
                .position(|&b| {
                    depth += match b {
                        b'[' => 1,
                        b']' => -1,
                        _ => 0,
                    };

                    depth == 0 && b == b','
                })
                .unwrap()
                + 1;

            Number::Pair(Box::new((
                parse_number(&data[1..comma_pos]),
                parse_number(&data[comma_pos + 1..data.len() - 1]),
            )))
        }
        _ => {
            assert_eq!(data.len(), 1);
            Number::Regular((data[0] as char).to_digit(10).unwrap() as u8)
        }
    }
}

fn to_string(num: &Number) -> String {
    match num {
        Number::Regular(x) => format!("{}", x),
        Number::Pair(pair) => format!("[{},{}]", to_string(&pair.0), to_string(&pair.1)),
    }
}

fn magnitude(num: &Number) -> u64 {
    match num {
        Number::Regular(x) => *x as u64,
        Number::Pair(pair) => 3 * magnitude(&pair.0) + 2 * magnitude(&pair.1),
    }
}

fn main() {
    let num = std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_number(line.unwrap().as_bytes()))
        .reduce(plus)
        .unwrap();
    println!("mag({}) = {}", to_string(&num), magnitude(&num));
}
