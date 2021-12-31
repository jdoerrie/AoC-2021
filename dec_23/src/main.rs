use std::cmp::max;
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::BufRead;

const ROWS: usize = 7;
const COLS: usize = 13;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Amph {
    A,
    B,
    C,
    D,
}

impl Amph {
    fn all() -> [Amph; 4] {
        [Amph::A, Amph::B, Amph::C, Amph::D]
    }
}

const fn get_col(amph: Amph) -> usize {
    match amph {
        Amph::A => 3,
        Amph::B => 5,
        Amph::C => 7,
        Amph::D => 9,
    }
}

const fn get_cost(amph: Amph) -> usize {
    match amph {
        Amph::A => 1,
        Amph::B => 10,
        Amph::C => 100,
        Amph::D => 1000,
    }
}

type Coord = [usize; 2];

const HALL_ROW: usize = 1;
const ROOM_ROWS: [usize; 4] = [2, 3, 4, 5];

const VALIDS: [Coord; 23] = [
    [HALL_ROW, 1],
    [HALL_ROW, 2],
    // [HALL_ROW, 3],
    [HALL_ROW, 4],
    // [HALL_ROW, 5],
    [HALL_ROW, 6],
    // [HALL_ROW, 7],
    [HALL_ROW, 8],
    // [HALL_ROW, 9],
    [HALL_ROW, 10],
    [HALL_ROW, 11],
    [ROOM_ROWS[0], get_col(Amph::A)],
    [ROOM_ROWS[0], get_col(Amph::B)],
    [ROOM_ROWS[0], get_col(Amph::C)],
    [ROOM_ROWS[0], get_col(Amph::D)],
    [ROOM_ROWS[1], get_col(Amph::A)],
    [ROOM_ROWS[1], get_col(Amph::B)],
    [ROOM_ROWS[1], get_col(Amph::C)],
    [ROOM_ROWS[1], get_col(Amph::D)],
    [ROOM_ROWS[2], get_col(Amph::A)],
    [ROOM_ROWS[2], get_col(Amph::B)],
    [ROOM_ROWS[2], get_col(Amph::C)],
    [ROOM_ROWS[2], get_col(Amph::D)],
    [ROOM_ROWS[3], get_col(Amph::A)],
    [ROOM_ROWS[3], get_col(Amph::B)],
    [ROOM_ROWS[3], get_col(Amph::C)],
    [ROOM_ROWS[3], get_col(Amph::D)],
];

fn make_path(mut src: Coord, dst: Coord) -> Vec<Coord> {
    let mut path = Vec::with_capacity(15);
    path.push(src);

    if src == dst {
        return path;
    }

    // Adjust columns.
    if src[1] != dst[1] {
        if src[0] != HALL_ROW {
            while src[0] != HALL_ROW {
                src[0] -= 1;
                path.push(src);
            }

            path.push(src);
        }

        while src[1] != dst[1] {
            if src[1] < dst[1] {
                src[1] += 1;
            } else {
                src[1] -= 1;
            }
            path.push(src);
        }

        path.push(src);
    }

    if src[0] != dst[0] {
        while src[0] != dst[0] {
            if src[0] < dst[0] {
                src[0] += 1;
            } else {
                src[0] -= 1;
            }
            path.push(src);
        }

        path.push(src);
    }

    path.dedup();
    path
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct State {
    pos_a: [Coord; 4],
    pos_b: [Coord; 4],
    pos_c: [Coord; 4],
    pos_d: [Coord; 4],
}

impl State {
    fn get_est_cost(&self) -> usize {
        Amph::all()
            .into_iter()
            .flat_map(|amph| {
                self.get_pos(amph).iter().map(move |pos| {
                    (if get_col(amph) != pos[1] {
                        pos[0] - HALL_ROW + 1 + max(get_col(amph), pos[1])
                            - min(get_col(amph), pos[1])
                    } else {
                        0
                    }) * get_cost(amph)
                })
            })
            .sum()
    }

    fn get_pos_mut(&mut self, amph: Amph) -> &mut [Coord; 4] {
        match amph {
            Amph::A => &mut self.pos_a,
            Amph::B => &mut self.pos_b,
            Amph::C => &mut self.pos_c,
            Amph::D => &mut self.pos_d,
        }
    }

    fn get_pos(&self, amph: Amph) -> &[Coord; 4] {
        match amph {
            Amph::A => &self.pos_a,
            Amph::B => &self.pos_b,
            Amph::C => &self.pos_c,
            Amph::D => &self.pos_d,
        }
    }

    fn is_done(&self) -> bool {
        Amph::all().into_iter().all(|amph| {
            self.get_pos(amph)
                .iter()
                .enumerate()
                .all(|(i, coord)| coord == &[ROOM_ROWS[i], get_col(amph)])
        })
    }

    fn check_move(
        &self,
        amph: Amph,
        [src_x, src_y]: Coord,
        [dst_x, dst_y]: Coord,
    ) -> Option<usize> {
        debug_assert!(VALIDS.iter().any(|&v| v == [src_x, src_y]));
        debug_assert!(VALIDS.iter().any(|&v| v == [dst_x, dst_y]));
        // Disallow moves that don't change the column.
        if src_y == dst_y {
            return None;
        }

        // Don't allow moves from the correct final position.
        for (i, &room) in ROOM_ROWS.iter().enumerate() {
            if [src_x, src_y] == [room, get_col(amph)]
                && (i + 1..ROOM_ROWS.len()).all(|j| {
                    self.get_pos(amph)
                        .iter()
                        .any(|&p| p == [ROOM_ROWS[j], get_col(amph)])
                })
            {
                return None;
            }
        }

        // Can't stay in Hall. Must go in correct column.
        if src_x == HALL_ROW && (dst_x == HALL_ROW || dst_y != get_col(amph)) {
            return None;
        }

        // If starting and ending in room, final room needs to be the destination.
        if src_x != HALL_ROW && dst_x != HALL_ROW && dst_y != get_col(amph) {
            return None;
        }

        let blocked = Amph::all()
            .iter()
            .flat_map(|&amph| self.get_pos(amph).iter().zip(std::iter::repeat(amph)))
            .collect::<HashMap<_, _>>();

        if blocked.contains_key(&[dst_x, dst_y]) {
            return None;
        }

        // If going into the final room, the other must be occupied by same amph.
        if dst_y == get_col(amph) {
            for row in ROOM_ROWS.iter().skip(dst_x - 1) {
                if match blocked.get(&[*row, dst_y]) {
                    Some(a) => *a != amph,
                    _ => true,
                } {
                    return None;
                }
            }
        }

        let path = make_path([src_x, src_y], [dst_x, dst_y]);
        debug_assert!(path[0] == [src_x, src_y]);
        debug_assert!(path[path.len() - 1] == [dst_x, dst_y]);

        if path.iter().skip(1).any(|c| blocked.contains_key(c)) {
            None
        } else {
            Some(get_cost(amph) * (path.len() - 1))
        }
    }

    fn get_next(&self) -> Vec<(State, usize)> {
        VALIDS
            .into_iter()
            .flat_map(|pos| {
                Amph::all().into_iter().flat_map(move |amph| {
                    self.get_pos(amph)
                        .iter()
                        .enumerate()
                        .map(move |(i, p)| (amph, i, p, pos))
                })
            })
            .filter_map(|(amph, i, &p, pos)| {
                if let Some(cost) = self.check_move(amph, p, pos) {
                    let mut state = self.clone();
                    state.get_pos_mut(amph)[i] = pos;
                    state.get_pos_mut(amph).sort_unstable();
                    Some((state, cost))
                } else {
                    None
                }
            })
            .collect()
    }

    fn print(&self) {
        let mut burrow: [[u8; COLS]; ROWS] = [
            "#############".as_bytes().try_into().unwrap(),
            "#...........#".as_bytes().try_into().unwrap(),
            "###.#.#.#.###".as_bytes().try_into().unwrap(),
            "  #.#.#.#.#  ".as_bytes().try_into().unwrap(),
            "  #.#.#.#.#  ".as_bytes().try_into().unwrap(),
            "  #.#.#.#.#  ".as_bytes().try_into().unwrap(),
            "  #########  ".as_bytes().try_into().unwrap(),
        ];

        Amph::all().into_iter().for_each(|amph| {
            self.get_pos(amph).iter().for_each(|&[i, j]| {
                burrow[i][j] = match amph {
                    Amph::A => b'A',
                    Amph::B => b'B',
                    Amph::C => b'C',
                    Amph::D => b'D',
                };
            })
        });

        for row in burrow {
            println!("{}", std::str::from_utf8(&row).unwrap());
        }
    }
}

enum Node {
    Seen,
    Unex(usize),
}

fn main() {
    let amphs = std::io::stdin()
        .lock()
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            let row = row.unwrap();
            row.chars()
                .enumerate()
                .filter_map(move |(j, c)| match c {
                    'A' => Some((Amph::A, [i, j])),
                    'B' => Some((Amph::B, [i, j])),
                    'C' => Some((Amph::C, [i, j])),
                    'D' => Some((Amph::D, [i, j])),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .fold(HashMap::new(), |mut acc, (amph, pos)| {
            acc.entry(amph).or_insert_with(Vec::new).push(pos);
            acc
        });

    let state = State {
        pos_a: amphs.get(&Amph::A).unwrap().clone().try_into().unwrap(),
        pos_b: amphs.get(&Amph::B).unwrap().clone().try_into().unwrap(),
        pos_c: amphs.get(&Amph::C).unwrap().clone().try_into().unwrap(),
        pos_d: amphs.get(&Amph::D).unwrap().clone().try_into().unwrap(),
    };

    let mut heap = BinaryHeap::new();
    let mut costs = HashMap::new();
    let est_cost = state.get_est_cost();
    heap.push((Reverse(est_cost), state.clone()));
    costs.insert(state, Node::Unex(est_cost));

    while let Some((Reverse(cost), state)) = heap.pop() {
        println!("Popping");
        state.print();
        println!("cost: {}\n", cost);
        if state.is_done() {
            state.print();
            println!("Done: {}", cost);
            return;
        }

        if matches!(costs.insert(state.clone(), Node::Seen), Some(Node::Seen)) {
            continue;
        }

        for (next, energy) in state.get_next() {
            let new_cost = cost + energy + next.get_est_cost() - state.get_est_cost();
            if match costs.get(&next) {
                Some(Node::Seen) => false,
                Some(Node::Unex(c)) => *c > new_cost,
                None => true,
            } {
                heap.push((Reverse(new_cost), next.clone()));
                costs.insert(next, Node::Unex(new_cost));
            }
        }
    }
}
