use std::collections::HashMap;
use std::io::BufRead;

type Player = usize;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    scores: [i32; 2],
    pos: [i32; 2],
    next: Player,
}

fn mod_with(i: i32, n: i32) -> i32 {
    if i % n == 0 {
        n
    } else {
        i % n
    }
}

fn num_wins(state: State, cache: &mut HashMap<State, [u64; 2]>) -> [u64; 2] {
    if let Some(&wins) = cache.get(&state) {
        wins
    } else if state.scores[0] >= 21 && state.next == 1 {
        [1, 0]
    } else if state.scores[1] >= 21 && state.next == 0 {
        [0, 1]
    } else {
        let wins = (0..27)
            .map(|i| {
                let dies: i32 = i % 3 + (i / 3) % 3 + (i / 9) + 3;
                let mut state = state;
                state.pos[state.next] = mod_with(state.pos[state.next] + dies, 10);
                state.scores[state.next] += state.pos[state.next];
                state.next = 1 - state.next;
                num_wins(state, cache)
            })
            .fold([0, 0], |[acc_0, acc_1], [win_0, win_1]| {
                [acc_0 + win_0, acc_1 + win_1]
            });
        cache.insert(state, wins);
        wins
    }
}

fn main() {
    let pos: [i32; 2] = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().split_once(": ").unwrap().1.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut cache = HashMap::new();
    let result = num_wins(
        State {
            scores: [0; 2],
            pos,
            next: 0,
        },
        &mut cache,
    );
    println!("max({:?}) = {}", result, result.iter().max().unwrap());
}
