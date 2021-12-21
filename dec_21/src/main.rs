use std::io::BufRead;

struct Die {
    count: i32,
}

fn mod_with(i: i32, n: i32) -> i32 {
    if i % n == 0 {
        n
    } else {
        i % n
    }
}

impl Die {
    fn new() -> Self {
        Die { count: 0 }
    }

    fn roll(&mut self) -> [i32; 3] {
        self.count += 3;
        [
            mod_with(self.count - 2, 100),
            mod_with(self.count - 1, 100),
            mod_with(self.count, 100),
        ]
    }

    fn count(&self) -> i32 {
        self.count
    }
}

fn main() {
    let mut pos = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().split_once(": ").unwrap().1.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut scores = vec![0; 2];

    let mut die = Die::new();
    loop {
        for i in 0..2 {
            pos[i] = mod_with(pos[i] + die.roll().iter().sum::<i32>(), 10);
            scores[i] += pos[i];
            if scores[i] >= 1000 {
                println!(
                    "{} * {} = {}",
                    scores[1 - i],
                    die.count(),
                    scores[1 - i] * die.count()
                );
                return;
            }
        }
    }
}
