use std::io::BufRead;

const GRID_X: usize = 10;
const GRID_Y: usize = 10;
const NEXT: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
fn main() {
    let mut grid: Vec<Vec<_>> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut step = 0;
    loop {
        step += 1;
        let mut flashing: Vec<_> = (0..GRID_X * GRID_Y)
            .map(|i| (i / GRID_Y, i % GRID_Y))
            .filter(|&(i, j)| {
                grid[i][j] += 1;
                grid[i][j] == 10
            })
            .collect();

        let mut flash_count = 0;
        while !flashing.is_empty() {
            flash_count += flashing.len();
            flashing = flashing
                .iter()
                .flat_map(|&(i, j)| {
                    NEXT.iter().filter_map(move |&(dx, dy)| {
                        let x: usize = (i as isize + dx) as usize;
                        let y: usize = (j as isize + dy) as usize;
                        if x < GRID_X && y < GRID_Y {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
                })
                .filter(|&(i, j)| {
                    grid[i][j] += 1;
                    grid[i][j] == 10
                })
                .collect();
        }
        (0..GRID_X * GRID_Y)
            .map(|i| (i / GRID_Y, i % GRID_Y))
            .for_each(|(i, j)| {
                if grid[i][j] >= 10 {
                    grid[i][j] = 0;
                }
            });

        if flash_count == GRID_X * GRID_Y {
            println!("{}", step);
            break;
        }
    }
}
