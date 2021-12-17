use std::cmp::max;

struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn parse_target(line: &str) -> Target {
    let [x_min, x_max, y_min, y_max]: [i32; 4] = line
        .split(',')
        .filter_map(|token| token.split_once('='))
        .filter_map(|(_, range)| range.split_once(".."))
        .flat_map(|(lhs, rhs)| [lhs.parse().unwrap(), rhs.parse().unwrap()])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    assert!(0 <= x_min && x_min <= x_max);
    assert!(y_min <= y_max && y_max <= 0);
    Target {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let target = parse_target(line.trim());

    let mut num_points = 0;
    for x_start in 0..=target.x_max {
        for y_start in target.y_min..=-target.y_min {
            let mut x = 0;
            let mut y = 0;
            let mut dx = x_start;
            let mut dy = y_start;

            while y >= target.y_min {
                x += dx;
                y += dy;
                dx = max(dx - 1, 0);
                dy -= 1;

                if target.x_min <= x && x <= target.x_max && target.y_min <= y && y <= target.y_max
                {
                    num_points += 1;
                    break;
                }
            }
        }
    }

    println!("{}", num_points);
}
