use smallvec::SmallVec;
use std::iter;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROGRAM: Vec<isize> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
}

fn param_count(opcode: isize) -> usize {
    match opcode {
        1 | 2 => 3,
        3 | 4 => 1,
        _ => 0
    }
}

fn run(program: &mut [isize], inputs: impl IntoIterator<Item = isize>) -> Vec<isize> {
    let mut ip = 0;
    let mut params = SmallVec::<[isize; 3]>::new();
    let mut inputs = inputs.into_iter();
    let mut outputs = Vec::new();
    loop {
        let inst = program[ip];
        ip += 1;
        let opcode = inst % 100;
        let mut modes = inst / 100;
        for _ in 0..param_count(opcode) {
            params.push(match modes % 10 {
                0 => program[program[ip] as usize],
                1 => program[ip],
                _ => panic!("unknown parameter mode")
            });
            ip += 1;
            modes /= 10;
        }

        match opcode {
            1 => program[params[2] as usize] = params[0] + params[1],
            2 => program[params[2] as usize] = params[0] * params[1],
            3 => program[params[0] as usize] = inputs.next().expect("missing input"),
            4 => outputs.push(params[0]),
            99 => return outputs,
            _ => panic!("unknown opcode"),
        }
        params.clear();
    }
}

fn part1() {
    let mut program = PROGRAM.clone();
    let output = run(&mut program, iter::once(1));
    println!("{:?}", output);
}

fn main() {
    part1();
}