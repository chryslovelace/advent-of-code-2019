use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOONS: Vec<Moon> = {
        let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
        re.captures_iter(include_str!("input.txt"))
            .map(|cap| Moon {
                pos: [
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].parse().unwrap(),
                ],
                vel: [0; 3],
            })
            .collect()
    };
}

#[derive(Clone)]
struct Moon {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl Moon {
    fn update_velocity(&mut self, other: &mut Moon) {
        use std::cmp::Ordering::*;

        for i in 0..3 {
            let delta = match self.pos[i].cmp(&other.pos[i]) {
                Less => 1,
                Equal => 0,
                Greater => -1,
            };
            self.vel[i] += delta;
            other.vel[i] -= delta;
        }
    }

    fn update_position(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }

    fn total_energy(&self) -> isize {
        self.pos.iter().map(|n| n.abs()).sum::<isize>()
            * self.vel.iter().map(|n| n.abs()).sum::<isize>()
    }
}

fn step(moons: &mut [Moon]) {
    for (i, j) in (0..moons.len()).combinations(2).map(|v| (v[0], v[1])) {
        let (left, right) = moons.split_at_mut(i + 1);
        left[i].update_velocity(&mut right[j - i - 1]);
    }
    for moon in moons {
        moon.update_position();
    }
}

fn part1() {
    let mut moons = MOONS.clone();
    for _ in 0..1000 {
        step(&mut moons);
    }
    let total_energy: isize = moons.iter().map(|m| m.total_energy()).sum();
    println!("{}", total_energy);
}

fn main() {
    part1();
}
