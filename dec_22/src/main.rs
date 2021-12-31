use std::cmp::max;
use std::cmp::min;
use std::io::BufRead;

#[derive(PartialEq, Debug, Clone)]
enum State {
    Empty,
    Full,
    Mixed(Box<[Split; 2]>),
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
struct Split {
    state: State,
    cuboid: Cuboid,
}

impl Split {
    fn vol(&self) -> usize {
        match &self.state {
            State::Empty => 0,
            State::Full => self.cuboid.vol(),
            State::Mixed(splits) => splits.iter().map(Split::vol).sum(),
        }
    }

    fn collapse(&mut self) {
        if let State::Mixed(splits) = &mut self.state {
            splits.iter_mut().for_each(|split| split.collapse());
            if splits
                .iter()
                .all(|split| matches!(split.state, State::Empty))
            {
                self.state = State::Empty;
            } else if splits
                .iter()
                .all(|split| matches!(split.state, State::Full))
            {
                self.state = State::Full;
            }
        }
    }

    fn split(&self, axis: usize) -> [Split; 2] {
        let c = &self.cuboid;

        (0..2)
            .map(|i| Split {
                state: self.state.clone(),
                cuboid: Cuboid {
                    coords: [
                        [
                            if axis == 0 && i == 0 {
                                c.x_mid()
                            } else {
                                c.x_min()
                            },
                            if axis == 0 && i == 1 {
                                c.x_mid()
                            } else {
                                c.x_max()
                            },
                        ],
                        [
                            if axis == 1 && i == 0 {
                                c.y_mid()
                            } else {
                                c.y_min()
                            },
                            if axis == 1 && i == 1 {
                                c.y_mid()
                            } else {
                                c.y_max()
                            },
                        ],
                        [
                            if axis == 2 && i == 0 {
                                c.z_mid()
                            } else {
                                c.z_min()
                            },
                            if axis == 2 && i == 1 {
                                c.z_mid()
                            } else {
                                c.z_max()
                            },
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
                State::Mixed(splits) => {
                    splits.iter_mut().for_each(|split| split.set(cuboid, is_on))
                }
                _ => {
                    if !is_noop {
                        let axis = common
                            .coords
                            .iter()
                            .zip(self.cuboid.coords)
                            .enumerate()
                            .filter(|(_, (_, [min, max]))| *max > (min + 1))
                            .map(|(i, ([l_min, l_max], [r_min, r_max]))| {
                                let res = ((r_max - r_min) / (l_max - l_min), i);
                                // println!("[{}, {}], [{}, {}], {:?}", l_min, l_max, r_min, r_max, res);
                                res
                            })
                            .max()
                            .unwrap()
                            .1;

                        let mut splits = self.split(axis);
                        // println!("       {:?},\nSelf   {:?}\nCommon {:?}", cuboid, self.cuboid, common);
                        // println!("Axis: {}", axis);
                        // println!("-----------------------------------------------------------------------");
                        // assert_eq!(splits[0].cuboid.vol() + splits[1].cuboid.vol(), self.cuboid.vol());
                        splits.iter_mut().for_each(|split| split.set(cuboid, is_on));
                        self.state = State::Mixed(Box::new(splits));
                    }
                }
            }
        }
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
    let len = 1 << 16;
    let mut split = Split {
        state: State::Empty,
        cuboid: Cuboid {
            coords: [[-len, len + 1], [-len, len + 1], [-len, len + 1]],
        },
    };
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .for_each(|(cube, is_on)| {
            println!("{:?}", (is_on, &cube));
            split.set(&cube, is_on);
            split.collapse();
        });
    println!("{}", split.vol());
}
