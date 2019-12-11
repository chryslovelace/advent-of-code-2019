use itertools::Itertools;
use lazy_static::lazy_static;
use num::integer::gcd;
use std::cmp::{Eq, Ord, Ordering};
use std::collections::HashSet;

lazy_static! {
    static ref POSITIONS: HashSet<(isize, isize)> = {
        let mut positions = HashSet::new();
        for (y, row) in include_str!("input.txt").trim().lines().enumerate() {
            for (x, a) in row.bytes().enumerate() {
                if a == b'#' {
                    positions.insert((x as isize, y as isize));
                }
            }
        }
        positions
    };
}

fn dir(start: (isize, isize), end: (isize, isize)) -> (isize, isize) {
    let mut dir = (end.0 - start.0, end.1 - start.1);
    let gcd = gcd(dir.0, dir.1);
    if gcd > 1 {
        dir = (dir.0 / gcd, dir.1 / gcd);
    }
    dir
}

fn count_detected(position: (isize, isize)) -> usize {
    let mut detected = HashSet::new();
    for &candidate in POSITIONS.iter().filter(|&&c| c != position) {
        detected.insert(dir(position, candidate));
    }
    detected.len()
}

fn part1() {
    let most_detected = POSITIONS.iter().map(|&p| count_detected(p)).max().unwrap();
    println!("{}", most_detected);
}

#[derive(PartialEq, PartialOrd)]
struct OrdFloat(f64);

impl Eq for OrdFloat {}

impl Ord for OrdFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn angle(dir: (isize, isize)) -> f64 {
    let d = (dir.0 as f64).atan2(-dir.1 as f64).to_degrees();
    if d < 0f64 {
        d + 360f64
    } else {
        d
    }
}

fn part2() {
    let station = POSITIONS
        .iter()
        .max_by_key(|&&p| count_detected(p))
        .unwrap();

    let lasered = POSITIONS
        .iter()
        .filter(|&c| c != station)
        .sorted_by_key(|&&position| {
            let dir = dir(*station, position);
            let mut pass = 0;
            let mut curr = (station.0 + dir.0, station.1 + dir.1);
            while curr != position {
                if POSITIONS.contains(&curr) {
                    pass += 1;
                }
                curr.0 += dir.0;
                curr.1 += dir.1
            }
            (pass, OrdFloat(angle(dir)))
        })
        .nth(199)
        .unwrap();
    println!("{}", lasered.0 * 100 + lasered.1)
}

fn main() {
    part1();
    part2();
}
