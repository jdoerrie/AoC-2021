use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::{self, BufRead};

type Point = (i32, i32);
struct Line(Point, Point);
struct LineIter(Point, Point);

impl Iterator for LineIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.0 .0, self.0 .1);
        let dx = self.1 .0 - self.0 .0;
        let dy = self.1 .1 - self.0 .1;
        self.0 .0 += max(min(dx, 1), -1);
        self.0 .1 += max(min(dy, 1), -1);
        if point != self.0 {
            Some(point)
        } else {
            None
        }
    }
}

impl Line {
    fn iter(&self) -> LineIter {
        let p1 = (self.0 .0, self.0 .1);
        let mut p2 = (self.1 .0, self.1 .1);
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        p2.0 += max(min(dx, 1), -1);
        p2.1 += max(min(dy, 1), -1);
        LineIter(p1, p2)
    }
}

fn parse_line(line: String) -> Line {
    let (p1, p2) = line.split_once(" -> ").unwrap();
    let (x1, y1) = p1.split_once(',').unwrap();
    let (x2, y2) = p2.split_once(',').unwrap();
    Line(
        (x1.parse().unwrap(), y1.parse().unwrap()),
        (x2.parse().unwrap(), y2.parse().unwrap()),
    )
}

fn main() {
    let mut map: HashMap<Point, i32> = HashMap::new();
    for point in io::stdin()
        .lock()
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .filter(|Line((x1, y1), (x2, y2))| {
            x1 == x2 || y1 == y2 || (x2 - x1).abs() == (y2 - y1).abs()
        })
        .flat_map(|line| line.iter())
    {
        *map.entry(point).or_default() += 1;
    }
    println!("{}", map.iter().filter(|(_, &count)| count > 1).count());
}
