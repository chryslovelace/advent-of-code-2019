use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use num::integer::lcm;

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
    pos: [i32; 3],
    vel: [i32; 3],
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

    fn total_energy(&self) -> i32 {
        self.pos.iter().map(|n| n.abs()).sum::<i32>()
            * self.vel.iter().map(|n| n.abs()).sum::<i32>()
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
    let total_energy: i32 = moons.iter().map(|m| m.total_energy()).sum();
    println!("{}", total_energy);
}

fn axis_cycle(axis: &[i32]) -> usize {
    use packed_simd::*;
    let mut pos = i32x4::from_slice_aligned(axis);
    let mut vel = i32x4::splat(0);
    for i in 0.. {
        vel += pos.lt(shuffle!(pos, [1, 2, 3, 0])).select(i32x4::splat(1), i32x4::splat(0));
        vel += pos.gt(shuffle!(pos, [1, 2, 3, 0])).select(i32x4::splat(-1), i32x4::splat(0));
        vel += pos.lt(shuffle!(pos, [2, 3, 0, 1])).select(i32x4::splat(1), i32x4::splat(0));
        vel += pos.gt(shuffle!(pos, [2, 3, 0, 1])).select(i32x4::splat(-1), i32x4::splat(0));
        vel += pos.lt(shuffle!(pos, [3, 0, 1, 2])).select(i32x4::splat(1), i32x4::splat(0));
        vel += pos.gt(shuffle!(pos, [3, 0, 1, 2])).select(i32x4::splat(-1), i32x4::splat(0));
        pos += vel;
        if pos == i32x4::from_slice_aligned(axis) && vel == i32x4::splat(0) { return i + 1; }
    }
    unreachable!()
}

fn part2() {
    let axes: Vec<Vec<_>> = (0..3).map(|i| MOONS.iter().map(|moon| moon.pos[i]).collect()).collect();
    let cycle = axes.iter().map(|axis| axis_cycle(axis)).fold1(lcm).unwrap();
    println!("{}", cycle);
}

fn main() {
    part1();
    part2();
}
