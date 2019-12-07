fn param_count(opcode: isize) -> usize {
    match opcode {
        1 | 2 | 7 | 8 => 3,
        3 | 4 => 1,
        5 | 6 => 2,
        _ => 0,
    }
}

fn is_write_param(opcode: isize, param_idx: usize) -> bool {
    match opcode {
        1 | 2 | 7 | 8 => param_idx == 2,
        3 => param_idx == 0,
        _ => false,
    }
}

pub trait Input {
    fn get_input(&mut self) -> Option<isize>;
}

impl<T: Iterator<Item = isize>> Input for T {
    fn get_input(&mut self) -> Option<isize> {
        self.next()
    }
}

pub trait Output {
    fn send_output(&mut self, data: isize);
}

impl Output for Vec<isize> {
    fn send_output(&mut self, data: isize) {
        self.push(data)
    }
}

pub fn run<I: Input, O: Output>(program: &mut [isize], input: &mut I, output: &mut O) {
    let mut ip = 0;
    let mut params = [0; 3];
    loop {
        let inst = program[ip];
        ip += 1;
        let opcode = inst % 100;
        let mut modes = inst / 100;
        for i in 0..param_count(opcode) {
            params[i] = match modes % 10 {
                _ if is_write_param(opcode, i) => program[ip],
                0 => program[program[ip] as usize],
                1 => program[ip],
                _ => panic!("unknown parameter mode"),
            };
            ip += 1;
            modes /= 10;
        }

        match opcode {
            1 => program[params[2] as usize] = params[0] + params[1],
            2 => program[params[2] as usize] = params[0] * params[1],
            3 => program[params[0] as usize] = input.get_input().expect("missing input"),
            4 => output.send_output(params[0]),
            5 => {
                if params[0] != 0 {
                    ip = params[1] as usize
                }
            }
            6 => {
                if params[0] == 0 {
                    ip = params[1] as usize
                }
            }
            7 => program[params[2] as usize] = (params[0] < params[1]) as isize,
            8 => program[params[2] as usize] = (params[0] == params[1]) as isize,
            99 => return,
            _ => panic!("unknown opcode"),
        }
    }
}
