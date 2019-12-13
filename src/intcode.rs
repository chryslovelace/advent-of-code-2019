use lazy_static::lazy_static;
use num::bigint::BigInt;
use num::traits::{One, Signed, ToPrimitive, Zero};
use std::collections::BTreeMap;
use std::fmt;

pub mod day7;

fn param_count(opcode: usize) -> usize {
    match opcode {
        1 | 2 | 7 | 8 => 3,
        3 | 4 | 9 => 1,
        5 | 6 => 2,
        _ => 0,
    }
}

fn is_write_param(opcode: usize, param_idx: usize) -> bool {
    match opcode {
        1 | 2 | 7 | 8 => param_idx == 2,
        3 => param_idx == 0,
        _ => false,
    }
}

pub trait Input {
    fn get_input(&mut self) -> BigInt;
}

impl<T: Iterator<Item = BigInt>> Input for T {
    fn get_input(&mut self) -> BigInt {
        self.next().unwrap()
    }
}

pub trait Output {
    fn send_output(&mut self, data: BigInt);
}

pub struct LastOutput(pub BigInt);

impl LastOutput {
    pub fn new() -> Self {
        Self(BigInt::zero())
    }
}

impl Output for LastOutput {
    fn send_output(&mut self, data: BigInt) {
        self.0 = data;
    }
}

impl Output for Vec<BigInt> {
    fn send_output(&mut self, data: BigInt) {
        self.push(data)
    }
}

impl fmt::Display for LastOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct Program {
    pub memory: BTreeMap<BigInt, BigInt>,
}

impl Program {
    pub fn new(s: &str) -> Self {
        Program {
            memory: s
                .trim()
                .split(',')
                .enumerate()
                .map(|(k, v)| (k.into(), v.parse().unwrap()))
                .collect(),
        }
    }

    pub fn get(&self, idx: &BigInt) -> &BigInt {
        lazy_static! {
            static ref ZERO: BigInt = BigInt::zero();
        }
        if idx.is_negative() {
            panic!("attempted to read negative address");
        }
        self.memory.get(idx).unwrap_or(&*ZERO)
    }

    pub fn set(&mut self, idx: BigInt, data: BigInt) {
        if idx.is_negative() {
            panic!("attempted to write negative address");
        }
        self.memory.insert(idx, data);
    }

    pub fn run<I: Input, O: Output>(&mut self, input: &mut I, output: &mut O) {
        let mut ip = BigInt::zero();
        let mut rb = BigInt::zero();
        let mut params = [BigInt::zero(), BigInt::zero(), BigInt::zero()];
        loop {
            let inst = self.get(&ip).to_usize().unwrap();
            ip += 1;
            let opcode = inst % 100;
            let mut mode = inst / 100;
            for i in 0..param_count(opcode) {
                params[i] = match mode % 10 {
                    0 if is_write_param(opcode, i) => self.get(&ip).clone(),
                    0 => self.get(self.get(&ip)).clone(),
                    1 => self.get(&ip).clone(),
                    2 if is_write_param(opcode, i) => &rb + self.get(&ip),
                    2 => self.get(&(&rb + self.get(&ip))).clone(),
                    _ => panic!("unknown parameter mode"),
                };
                ip += 1;
                mode /= 10;
            }

            match opcode {
                1 => self.set(params[2].clone(), &params[0] + &params[1]),
                2 => self.set(params[2].clone(), &params[0] * &params[1]),
                3 => self.set(params[0].clone(), input.get_input()),
                4 => output.send_output(params[0].clone()),
                5 => {
                    if params[0] != BigInt::zero() {
                        ip = params[1].clone()
                    }
                }
                6 => {
                    if params[0] == BigInt::zero() {
                        ip = params[1].clone()
                    }
                }
                7 => self.set(
                    params[2].clone(),
                    if params[0] < params[1] {
                        BigInt::one()
                    } else {
                        BigInt::zero()
                    },
                ),
                8 => self.set(
                    params[2].clone(),
                    if params[0] == params[1] {
                        BigInt::one()
                    } else {
                        BigInt::zero()
                    },
                ),
                9 => rb += &params[0],
                99 => return,
                _ => panic!("unknown opcode"),
            }
        }
    }
}
