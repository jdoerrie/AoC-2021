use std::cmp::max;
use std::cmp::min;
use std::io::BufRead;

#[derive(PartialEq, Debug, Clone)]
enum State {
    Empty,
    Full,
    Mixed(Box<[Octant; 8]>),
}

#[derive(PartialEq, Debug, Clone)]
struct Cuboid {
    coords: [[i64; 2]; 3],
}

impl Cuboid {
    fn vol(&self) -> usize {
        self.coords
            .iter()
            .map(|[min, max]| (max - min) as usize)
            .product()
    }

    fn intersection(&self, cuboid: &Cuboid) -> Cuboid {
        Cuboid {
            coords: self
                .coords
                .iter()
                .zip(cuboid.coords.iter())
                .map(|([l_min, l_max], [r_min, r_max])| {
                    assert!(l_min <= l_max);
                    assert!(r_min <= r_max);
                    if l_max <= r_min || r_max <= l_min {
                        [0; 2]
                    } else {
                        [*max(l_min, r_min), *min(l_max, r_max)]
                    }
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    fn x_min(&self) -> i64 {
        self.coords[0][0]
    }

    fn x_mid(&self) -> i64 {
        self.coords[0].iter().sum::<i64>() / 2
    }

    fn x_max(&self) -> i64 {
        self.coords[0][1]
    }

    fn y_min(&self) -> i64 {
        self.coords[1][0]
    }

    fn y_mid(&self) -> i64 {
        self.coords[1].iter().sum::<i64>() / 2
    }

    fn y_max(&self) -> i64 {
        self.coords[1][1]
    }

    fn z_min(&self) -> i64 {
        self.coords[2][0]
    }

    fn z_mid(&self) -> i64 {
        self.coords[2].iter().sum::<i64>() / 2
    }

    fn z_max(&self) -> i64 {
        self.coords[2][1]
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Octant {
    state: State,
    cuboid: Cuboid,
}

impl Octant {
    fn vol(&self) -> usize {
        match &self.state {
            State::Empty => 0,
            State::Full => self.cuboid.vol(),
            State::Mixed(octants) => octants.iter().map(|oct| oct.vol()).sum(),
        }
    }

    fn collapse(&mut self) {
        if let State::Mixed(octants) = &mut self.state {
            octants.iter_mut().for_each(|oct| oct.collapse());
            if octants.iter().all(|oct| matches!(oct.state, State::Empty)) {
                self.state = State::Empty;
            } else if octants.iter().all(|oct| matches!(oct.state, State::Full)) {
                self.state = State::Full;
            }
        }
    }

    fn split(&self) -> [Octant; 8] {
        let c = &self.cuboid;

        (0..8)
            .map(|i| Octant {
                state: self.state.clone(),
                cuboid: Cuboid {
                    coords: [
                        [
                            if i & 4 == 0 { c.x_min() } else { c.x_mid() },
                            if i & 4 == 0 { c.x_mid() } else { c.x_max() },
                        ],
                        [
                            if i & 2 == 0 { c.y_min() } else { c.y_mid() },
                            if i & 2 == 0 { c.y_mid() } else { c.y_max() },
                        ],
                        [
                            if i & 1 == 0 { c.z_min() } else { c.z_mid() },
                            if i & 1 == 0 { c.z_mid() } else { c.z_max() },
                        ],
                    ],
                },
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn set(&mut self, cuboid: &Cuboid, is_on: bool) {
        let common = self.cuboid.intersection(cuboid);
        if common == self.cuboid {
            self.state = if is_on { State::Full } else { State::Empty };
        } else if common.vol() != 0 {
            let is_noop = (is_on && matches!(self.state, State::Full))
                || (!is_on && matches!(self.state, State::Empty));
            match &mut self.state {
                State::Mixed(octants) => octants.iter_mut().for_each(|oct| oct.set(cuboid, is_on)),
                _ => {
                    if !is_noop {
                        let mut octants = self.split();
                        octants.iter_mut().for_each(|oct| oct.set(cuboid, is_on));
                        self.state = State::Mixed(Box::new(octants));
                    }
                }
            }
        }
        self.collapse();
    }
}

fn parse_line(line: &str) -> (Cuboid, bool) {
    let (lhs, rhs) = line.split_once(" ").unwrap();
    (
        Cuboid {
            coords: rhs
                .split(',')
                .filter_map(|token| token.split_once('='))
                .filter_map(|(_, rng)| rng.split_once(".."))
                .map(|(min, max)| [min.parse().unwrap(), max.parse::<i64>().unwrap() + 1])
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        },
        lhs == "on",
    )
}

fn main() {
    let mut octant = Octant {
        state: State::Empty,
        cuboid: Cuboid {
            coords: [[-50, 51], [-50, 51], [-50, 51]],
        },
    };
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .for_each(|(cube, is_on)| octant.set(&cube, is_on));
    println!("{}", octant.vol());
}
