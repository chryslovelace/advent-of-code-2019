use aoc2019::intcode;
use itertools::Itertools;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use num::bigint::BigInt;
use num::traits::ToPrimitive;

const INPUT: &'static str = include_str!("input.txt");

struct StateInner {
    panels: HashMap<(isize, isize), bool>,
    robot_position: (isize, isize),
    robot_direction: (isize, isize),
    next_output_is_color: bool,
}

#[derive(Clone)]
struct State(Rc<RefCell<StateInner>>);

impl State {
    fn new() -> Self {
        State(Rc::new(RefCell::new(StateInner {
            panels: HashMap::new(),
            robot_position: (0, 0),
            robot_direction: (0, 1),
            next_output_is_color: true
        })))
    }
}

impl intcode::Input for State {
    fn get_input(&mut self) -> BigInt {
        let state = self.0.borrow();
        if *state.panels.get(&state.robot_position).unwrap_or(&false) {
            1.into()
        } else {
            0.into()
        }
    }
}

impl intcode::Output for State {
    fn send_output(&mut self, data: BigInt) {
        let mut state = self.0.borrow_mut();
        let n = data.to_isize().unwrap();
        if state.next_output_is_color {
            let pos = state.robot_position;
            state.panels.insert(pos, n == 1);
            state.next_output_is_color = false;
        } else {
            let next_dir = match (state.robot_direction, n) {
                ((0, 1), 0) | ((0, -1), 1) => (-1, 0),
                ((0, 1), 1) | ((0, -1), 0) => (1, 0),
                ((1, 0), 0) | ((-1, 0), 1) => (0, 1),
                ((1, 0), 1) | ((-1, 0), 0) => (0, -1),
                _ => panic!()
            };
            state.robot_position.0 += next_dir.0;
            state.robot_position.1 += next_dir.1;
            state.robot_direction = next_dir;
            state.next_output_is_color = true;
        }
    }
}

fn part1() {
    let mut program = intcode::Program::new(INPUT);
    let state = State::new();
    program.run(&mut state.clone(), &mut state.clone());
    println!("{}", state.0.borrow().panels.len());
}

fn part2() {
    let mut program = intcode::Program::new(INPUT);
    let state = State::new();
    state.0.borrow_mut().panels.insert((0, 0), true);
    program.run(&mut state.clone(), &mut state.clone());
    let panels = &state.0.borrow().panels;
    let (min_x, max_x) = panels.iter().filter(|(_, v)| **v).map(|((x, _), _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = panels.iter().filter(|(_, v)| **v).map(|((_, y), _)| y).minmax().into_option().unwrap();
    for y in (*min_y..=*max_y).rev() {
        for x in *min_x..=*max_x {
            if *panels.get(&(x, y)).unwrap_or(&false) {
                print!("██");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn main() {
    part1();
    part2();
}