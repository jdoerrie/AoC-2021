use std::io::BufRead;

type SeaFloor = Vec<Vec<Tile>>;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    East = '>' as isize,
    South = 'v' as isize,
    Empty = '.' as isize,
}

fn move_sea(sea: &mut SeaFloor, dir: Tile) -> bool {
    let [dx, dy] = match dir {
        Tile::East => [0, 1],
        Tile::South => [1, 0],
        _ => [0, 0],
    };

    let mut new_sea = sea.clone();
    for i in 0..sea.len() {
        let next_i = (i + dx) % sea.len();
        for j in 0..sea[i].len() {
            let next_j = (j + dy) % sea[i].len();
            if sea[i][j] == dir && sea[next_i][next_j] == Tile::Empty {
                new_sea[i][j] = Tile::Empty;
                new_sea[next_i][next_j] = dir;
            }
        }
    }

    let changed = *sea != new_sea;
    *sea = new_sea;
    changed
}

fn main() {
    let mut sea: SeaFloor = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '>' => Tile::East,
                    'v' => Tile::South,
                    _ => Tile::Empty,
                })
                .collect()
        })
        .collect();

    let mut n_steps = 1;
    while move_sea(&mut sea, Tile::East) | move_sea(&mut sea, Tile::South) {
        n_steps += 1;
    }

    println!("{}", n_steps);
}
