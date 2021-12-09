use std::io::BufRead;

fn mark_basin(i: usize, j: usize, grid: &[Vec<u32>], is_visited: &mut Vec<Vec<bool>>) -> usize {
    if is_visited[i][j] {
        return 0;
    }

    is_visited[i][j] = true;
    if grid[i][j] == 9 {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();

    let mut size = 1;
    if i != 0 {
        size += mark_basin(i - 1, j, grid, is_visited);
    }
    if i != m - 1 {
        size += mark_basin(i + 1, j, grid, is_visited);
    }
    if j != 0 {
        size += mark_basin(i, j - 1, grid, is_visited);
    }
    if j != n - 1 {
        size += mark_basin(i, j + 1, grid, is_visited);
    }
    size
}

fn main() {
    let grid: Vec<Vec<_>> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    let mut is_visited = vec![vec![false; n]; m];
    let mut basin_sizes: Vec<_> = (0..m * n)
        .map(|i| mark_basin(i / n, i % n, &grid, &mut is_visited))
        .collect();
    basin_sizes.sort_unstable();
    println!("{}", basin_sizes.iter().rev().take(3).product::<usize>());
}
