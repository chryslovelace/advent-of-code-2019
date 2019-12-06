use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref ORBITS: Vec<(&'static str, &'static str)> = include_str!("input.txt")
        .lines()
        .map(|s| (&s[0..3], &s[4..]))
        .collect();
}

fn part1() {
    let mut map = HashMap::new();
    for (a, b) in ORBITS.iter() {
        map.entry(a).or_insert_with(Vec::new).push(b);
    }
    let mut curr = vec!["COM"];
    let mut depth = 0;
    let mut checksum = 0;
    while curr.len() > 0 {
        checksum += curr.len() * depth;
        curr = curr
            .into_iter()
            .filter_map(|c| map.get(&c))
            .flatten()
            .map(|&&s| s)
            .collect();
        depth += 1;
    }
    println!("{}", checksum);
}

fn part2() {
    let mut map = HashMap::new();
    for (a, b) in ORBITS.iter() {
        map.entry(a).or_insert_with(Vec::new).push(b);
        map.entry(b).or_insert_with(Vec::new).push(a);
    }
    let mut dist = 0;
    let mut curr = vec!["YOU"];
    let mut visited: HashSet<&'static str> = HashSet::new();
    visited.insert("YOU");
    loop {
        curr = curr
            .into_iter()
            .filter_map(|c| map.get(&c))
            .flatten()
            .map(|&&s| s)
            .filter(|s| visited.insert(s))
            .collect();
        if curr.contains(&"SAN") {
            println!("{}", dist - 1);
            return;
        }
        dist += 1;
    }
}

fn main() {
    part1();
    part2();
}
