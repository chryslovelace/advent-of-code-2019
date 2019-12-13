use aoc2019::intcode;
use itertools::Itertools;
use num::bigint::BigInt;
use num::traits::ToPrimitive;
use std::collections::{HashSet, HashMap};
use std::rc::Rc;
use std::cell::RefCell;

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

#[derive(Default)]
struct StateInner {
    render_on: bool,
    max_x: i32,
    max_y: i32,
    last_x: Option<i32>,
    last_y: Option<i32>,
    score: i32,
    last_ball: Option<(i32, i32)>,
    target: Option<i32>,
    tiles: HashMap<(i32, i32), i32>,
}

impl StateInner {
    fn render(&self) {        
        println!("{}", self.score);
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                print!("{}", match self.tiles.get(&(x, y)).unwrap_or(&0) {
                    1 => "|",
                    2 => "#",
                    3 => "=",
                    4 => "O",
                    _ => " "
                });
            }
            println!("");
        }
    }
}

#[derive(Default, Clone)]
struct State(Rc<RefCell<StateInner>>);

impl intcode::Input for State {
    fn get_input(&mut self) -> BigInt {
        use std::cmp::Ordering::*;
        let state = self.0.borrow();
        if let Some((ball, _)) = state.tiles.iter().find(|(_, &v)| v == 4) {
            if let Some((paddle, _)) = state.tiles.iter().find(|(_, &v)| v == 3) {
                return match paddle.cmp(&ball.0) {
                    Less => 1,
                    Equal => 0,
                    Greater => -1
                }.into()
            }
        }
        0.into()
    }
}

impl intcode::Output for State {
    fn send_output(&mut self, data: BigInt) {
        let mut state = self.0.borrow_mut();
        let data: i32 = data.to_i32().unwrap();
        if state.last_x.is_none() {
            state.max_x = state.max_x.max(data);
            state.last_x = Some(data);
        } else if state.last_y.is_none() {
            state.max_y = state.max_y.max(data);
            state.last_y = Some(data);
        } else {
            let (x, y) = (state.last_x.take().unwrap(), state.last_y.take().unwrap());
            if x == -1 {
                state.score = data;
            } else {
                state.tiles.insert((x, y), data);
            }
            if state.render_on && data >= 3 { state.render(); }
        }
    }
}

fn part2() {
    let mut program = intcode::Program::new(INPUT);
    program.set(0.into(), 2.into());
    let mut state = State::default();
    state.0.borrow_mut().render_on = true;
    program.run(&mut state.clone(), &mut state);
    println!("{}", state.0.borrow().score);
}

fn main() {
    part1();
    part2();
}