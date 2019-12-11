use aoc2019::intcode::*;
use num::bigint::BigInt;
use num::traits::One;
use std::iter;

const INPUT: &'static str = include_str!("input.txt");

fn part1() {
    let mut program = Program::new(INPUT);
    let mut output = LastOutput::new();
    program.run(&mut iter::once(BigInt::one()), &mut output);
    println!("{}", output);
}

fn part2() {
    let mut program = Program::new(INPUT);
    let mut output = LastOutput::new();
    program.run(&mut iter::once(2.into()), &mut output);
    println!("{}", output);
}

fn main() {
    part1();
    part2();
}
