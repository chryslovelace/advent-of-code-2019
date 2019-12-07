use aoc2019::intcode;
use lazy_static::lazy_static;
use std::iter;

lazy_static! {
    static ref PROGRAM: Vec<isize> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
}

fn part1() {
    let mut program = PROGRAM.clone();
    let mut output = Vec::new();
    intcode::run(&mut program, &mut iter::once(1), &mut output);
    println!("{:?}", output);
}

fn part2() {
    let mut program = PROGRAM.clone();
    let mut output = Vec::new();
    intcode::run(&mut program, &mut iter::once(5), &mut output);
    println!("{:?}", output);
}

fn main() {
    part1();
    part2();
}
