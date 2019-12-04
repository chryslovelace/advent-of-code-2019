use itertools::Itertools;
use std::collections::HashMap;

const RANGE: std::ops::RangeInclusive<usize> = 372037..=905157;

fn valid1(&n: &usize) -> bool {
    let s = n.to_string();
    n > 99999
        && n < 1000000
        && s.chars().tuple_windows().all(|(x, y)| x <= y)
        && s.chars().tuple_windows().any(|(x, y)| x == y)
}

fn valid2(n: &usize) -> bool {
    valid1(n)
        && n.to_string()
            .chars()
            .tuple_windows()
            .filter(|(x, y)| x == y)
            .fold(HashMap::new(), |mut map, (x, _)| {
                *map.entry(x).or_insert(0) += 1;
                map
            })
            .values()
            .any(|&x| x == 1)
}

fn part1() {
    let count = RANGE.filter(valid1).count();
    println!("{}", count);
}

fn part2() {
    let count = RANGE.filter(valid2).count();
    println!("{}", count);
}

fn main() {
    part1();
    part2();
}
