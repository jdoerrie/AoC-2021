use counter::Counter;
use std::collections::HashSet;
use std::io::Read;

type Coords = [i32; 3];
type Scan = Vec<Coords>;

fn variations(coords: &Coords) -> [Coords; 24] {
    let [x, y, z] = *coords;
    [
        // x, y, z: pos
        [x, y, z],
        [x, -y, -z],
        [-x, y, -z],
        [-x, -y, z],
        // x, z, y: neg
        [-x, -z, -y],
        [-x, z, y],
        [x, -z, y],
        [x, z, -y],
        // y, x, z: neg
        [-y, -x, -z],
        [-y, x, z],
        [y, -x, z],
        [y, x, -z],
        // y, z, x: pos
        [y, z, x],
        [y, -z, -x],
        [-y, z, -x],
        [-y, -z, x],
        // z, x, y: pos
        [z, x, y],
        [z, -x, -y],
        [-z, x, -y],
        [-z, -x, y],
        // z, y, x: neg
        [-z, -y, -x],
        [-z, y, x],
        [z, -y, x],
        [z, y, -x],
    ]
}

fn plus(lhs: &Coords, rhs: &Coords) -> Coords {
    let mut result = [0; 3];
    for ((rref, a), b) in result.iter_mut().zip(lhs).zip(rhs) {
        *rref = a + b;
    }
    result
}

fn minus(lhs: &Coords, rhs: &Coords) -> Coords {
    let mut result = [0; 3];
    for ((rref, a), b) in result.iter_mut().zip(lhs).zip(rhs) {
        *rref = a - b;
    }
    result
}

fn merge(lhs: &mut HashSet<Coords>, scan: &[Coords]) -> bool {
    for i in 0..24 {
        let (scan_origin, counts) = lhs
            .iter()
            .flat_map(|coord_lhs| {
                scan.iter()
                    .map(|coord_rhs| minus(coord_lhs, &variations(coord_rhs)[i]))
            })
            .collect::<Counter<_>>()
            .most_common_ordered()[0];
        if counts >= 12 {
            scan.iter()
                .map(|coord| plus(&variations(coord)[i], &scan_origin))
                .for_each(|coord| {
                    lhs.insert(coord);
                });
            return true;
        }
    }

    false
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let scans: Vec<Scan> = input
        .split("\n\n")
        .map(|scan| {
            scan.split('\n')
                .flat_map(|line| {
                    line.split(',')
                        .flat_map(|c| c.parse())
                        .collect::<Vec<_>>()
                        .try_into()
                })
                .collect()
        })
        .collect();

    let n = scans.len();
    let mut merged = scans[0].clone().into_iter().collect::<HashSet<_>>();
    let mut are_merged = HashSet::new();
    are_merged.insert(0);
    while are_merged.len() != n {
        for (i, scan) in scans.iter().enumerate().take(n) {
            if are_merged.contains(&i) {
                continue;
            }

            if merge(&mut merged, scan.as_slice()) {
                are_merged.insert(i);
            }
        }
    }

    println!("{}", merged.len());
}
