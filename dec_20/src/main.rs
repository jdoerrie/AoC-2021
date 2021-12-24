use std::collections::HashSet;
use std::io::Read;

const NEXT: [(isize, isize); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (algo_str, image_str) = input.split_once("\n\n").unwrap();

    let algo: [u8; 512] = algo_str.as_bytes().try_into().unwrap();
    let mut image: HashSet<_> = image_str
        .split('\n')
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars().enumerate().filter_map(move |(j, c)| match c {
                '#' => Some((i as isize, j as isize)),
                _ => None,
            })
        })
        .collect();

    assert!(algo[0] == b'.' || algo[511] == b'.');
    for _ in 0..50 {
        image = (-105..205)
            .flat_map(|i| (-105..205).map(move |j| (i, j)))
            .filter(|(i, j)| {
                algo[NEXT
                    .iter()
                    .map(|(dx, dy)| (i + dx, j + dy))
                    .fold(0, |acc, pix| 2 * acc + (image.contains(&pix) as usize))]
                    == b'#'
            })
            .collect();
    }

    println!(
        "{}",
        image
            .into_iter()
            .filter(|&(i, j)| i >= -55 && i < 155 && j >= -55 && j < 155)
            .count()
    );
}
