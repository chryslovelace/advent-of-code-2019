use lazy_static::lazy_static;

lazy_static!{
    static ref PROGRAM: Vec<usize> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
}

fn run(program: &mut [usize]) {
    let mut ip = 0;
    loop {
        match program[ip] {
            1 => program[program[ip+3]] = program[program[ip+1]] + program[program[ip+2]],
            2 => program[program[ip+3]] = program[program[ip+1]] * program[program[ip+2]],
            99 => return,
            _ => panic!("unknown opcode")
        }
        ip += 4;
    }
}

fn part1() {
    let mut program = PROGRAM.clone();
    program[1] = 12;
    program[2] = 2;
    run(&mut program);
    println!("{}", program[0]);
}

fn part2() {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = PROGRAM.clone();
            program[1] = noun;
            program[2] = verb;
            run(&mut program);
            if program[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}

fn main() {
    part1();
    part2();
}