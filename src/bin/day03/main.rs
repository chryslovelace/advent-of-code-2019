use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

struct Move {
    dir: char,
    len: u32,
}

lazy_static! {
    static ref PATHS: (Vec<Move>, Vec<Move>) = {
        fn parse_line(line: &str) -> Vec<Move> {
            line.split(',')
                .map(|s| Move {
                    dir: s.chars().next().unwrap(),
                    len: s[1..].parse().unwrap(),
                })
                .collect()
        }

        let mut lines = include_str!("input.txt").lines();
        (
            parse_line(lines.next().unwrap()),
            parse_line(lines.next().unwrap()),
        )
    };
}

fn points(path: &[Move]) -> HashMap<(i32, i32), u32> {
    let mut points = HashMap::new();
    let mut steps = 0;
    let mut current = (0, 0);
    for m in path {
        for _ in 0..m.len {
            match m.dir {
                'R' => current.0 += 1,
                'L' => current.0 -= 1,
                'U' => current.1 += 1,
                'D' => current.1 -= 1,
                _ => unreachable!(),
            }
            steps += 1;
            points.entry(current).or_insert(steps);
        }
    }
    points
}

fn part1() {
    let points = (
        points(&PATHS.0).keys().cloned().collect::<HashSet<_>>(),
        points(&PATHS.1).keys().cloned().collect::<HashSet<_>>(),
    );
    let dist = points
        .0
        .intersection(&points.1)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();
    println!("{}", dist);
}

fn part2() {
    let points = (points(&PATHS.0), points(&PATHS.1));
    let dist = points
        .0
        .iter()
        .filter_map(|(p, steps)| points.1.get(p).map(|s| s + steps))
        .min()
        .unwrap();
    println!("{}", dist);
}

fn main() {
    part1();
    part2();
}
