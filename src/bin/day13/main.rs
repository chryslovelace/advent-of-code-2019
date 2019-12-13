use aoc2019::intcode;
use num::bigint::BigInt;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Default)]
struct BlockCounter {
    last_x: Option<BigInt>,
    last_y: Option<BigInt>,
    blocks: HashSet<(BigInt, BigInt)>
}

impl intcode::Output for BlockCounter {
    fn send_output(&mut self, data: BigInt) {
        if self.last_x.is_none() {
            self.last_x = Some(data);
        } else if self.last_y.is_none() {
            self.last_y = Some(data);
        } else {
            let pos = (self.last_x.take().unwrap(), self.last_y.take().unwrap());
            if data == 2.into() {
                self.blocks.insert(pos);
            }
        }
    }
}

fn part1() {
    let mut program = intcode::Program::new(INPUT);
    let mut counter = BlockCounter::default();
    program.run(&mut std::iter::empty(), &mut counter);
    println!("{}", counter.blocks.len());
}

fn main() {
    part1();
}