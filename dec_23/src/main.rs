use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::BufRead;

const ROWS: usize = 5;
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
const ROOM_ROW1: usize = 2;
const ROOM_ROW2: usize = 3;

const VALIDS: [Coord; 15] = [
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
    [ROOM_ROW1, get_col(Amph::A)],
    [ROOM_ROW1, get_col(Amph::B)],
    [ROOM_ROW1, get_col(Amph::C)],
    [ROOM_ROW1, get_col(Amph::D)],
    [ROOM_ROW2, get_col(Amph::A)],
    [ROOM_ROW2, get_col(Amph::B)],
    [ROOM_ROW2, get_col(Amph::C)],
    [ROOM_ROW2, get_col(Amph::D)],
];

fn make_path(mut src: Coord, dst: Coord) -> Vec<Coord> {
    let mut path = Vec::with_capacity(11);
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
    pos_a: [Coord; 2],
    pos_b: [Coord; 2],
    pos_c: [Coord; 2],
    pos_d: [Coord; 2],
}

impl State {
    fn get_pos_mut(&mut self, amph: Amph) -> &mut [Coord; 2] {
        match amph {
            Amph::A => &mut self.pos_a,
            Amph::B => &mut self.pos_b,
            Amph::C => &mut self.pos_c,
            Amph::D => &mut self.pos_d,
        }
    }

    fn get_pos(&self, amph: Amph) -> &[Coord; 2] {
        match amph {
            Amph::A => &self.pos_a,
            Amph::B => &self.pos_b,
            Amph::C => &self.pos_c,
            Amph::D => &self.pos_d,
        }
    }

    fn is_done(&self) -> bool {
        Amph::all().into_iter().all(|amph| {
            self.get_pos(amph).iter().min() == Some(&[ROOM_ROW1, get_col(amph)])
                && self.get_pos(amph).iter().max() == Some(&[ROOM_ROW2, get_col(amph)])
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

        // Don't allow moves from the correct final position.
        if [src_x, src_y] == [ROOM_ROW2, get_col(amph)] {
            return None;
        }

        if [src_x, src_y] == [ROOM_ROW1, get_col(amph)]
            && self
                .get_pos(amph)
                .iter()
                .any(|&p| p == [ROOM_ROW2, get_col(amph)])
        {
            return None;
        }

        // Can't stay in Hall. Must go in correct column.
        if src_x == HALL_ROW && (dst_x == HALL_ROW || dst_y != get_col(amph)) {
            return None;
        }

        // If starting and ending in room, room needs to be the same or the destination.
        if src_x != HALL_ROW && dst_x != HALL_ROW && src_y != dst_y && dst_y != get_col(amph) {
            return None;
        }

        let blocked = Amph::all()
            .iter()
            .flat_map(|&amph| self.get_pos(amph).iter().zip(std::iter::repeat(amph)))
            .collect::<HashMap<_, _>>();

        // If going into the final room, the other spot can't be occupied by a different amph.
        if [dst_x, dst_y] == [ROOM_ROW1, get_col(amph)] {
            let tile = blocked.get(&[ROOM_ROW2, dst_y]);
            if tile.is_some() && *tile.unwrap() != amph {
                return None;
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

    let mut state = State {
        pos_a: amphs.get(&Amph::A).unwrap().clone().try_into().unwrap(),
        pos_b: amphs.get(&Amph::B).unwrap().clone().try_into().unwrap(),
        pos_c: amphs.get(&Amph::C).unwrap().clone().try_into().unwrap(),
        pos_d: amphs.get(&Amph::D).unwrap().clone().try_into().unwrap(),
    };

    state.pos_a.sort_unstable();
    state.pos_b.sort_unstable();
    state.pos_c.sort_unstable();
    state.pos_d.sort_unstable();

    let mut heap = BinaryHeap::new();
    let mut costs = HashMap::new();
    heap.push((Reverse(0), state.clone()));
    costs.insert(state, Node::Unex(0));

    while let Some((Reverse(cost), state)) = heap.pop() {
        if state.is_done() {
            state.print();
            println!("Done: {}", cost);
            return;
        }

        if matches!(costs.insert(state.clone(), Node::Seen), Some(Node::Seen)) {
            continue;
        }

        for (next, energy) in state.get_next() {
            let new_cost = cost + energy;
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
