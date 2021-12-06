use std::io::BufRead;

fn main() {
    let mut fishes = [0usize; 9];
    for fish in std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|token| token.parse::<usize>().unwrap())
    {
        fishes[fish] += 1;
    }

    for _ in 0..80 {
        fishes = [
            fishes[1],             // fishes[0]
            fishes[2],             // fishes[1]
            fishes[3],             // fishes[2]
            fishes[4],             // fishes[3]
            fishes[5],             // fishes[4]
            fishes[6],             // fishes[5]
            fishes[0] + fishes[7], // fishes[6]
            fishes[8],             // fishes[7]
            fishes[0],             // fishes[8]
        ];
    }

    println!("{}", fishes.iter().sum::<usize>());
}
