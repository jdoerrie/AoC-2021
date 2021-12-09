use std::io::BufRead;

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

    let is_low_point = |(i, j): (usize, usize)| {
        (i == 0 || grid[i][j] < grid[i - 1][j])
            && (i == m - 1 || grid[i][j] < grid[i + 1][j])
            && (j == 0 || grid[i][j] < grid[i][j - 1])
            && (j == n - 1 || grid[i][j] < grid[i][j + 1])
    };

    println!(
        "{}",
        (0..m * n)
            .map(|i| (i / n, i % n))
            .filter(|&coords| is_low_point(coords))
            .map(|(i, j)| grid[i][j] + 1)
            .sum::<u32>()
    );
}
