use aoc2019::intcode::day7 as intcode;
use crossbeam::scope;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::sync::mpsc;

lazy_static! {
    static ref PROGRAM: Vec<isize> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
}

fn part1() {
    let signal = (0..5)
        .permutations(5)
        .map(|settings| {
            settings.into_iter().fold(0, |input, setting| {
                let mut output = intcode::LastOutput(0);
                intcode::run(
                    &mut PROGRAM.clone(),
                    &mut vec![setting, input].into_iter(),
                    &mut output,
                );
                output.0
            })
        })
        .max()
        .unwrap();
    println!("{}", signal);
}

struct LoopInput {
    phase_setting: Option<isize>,
    channel: mpsc::Receiver<isize>,
}

impl LoopInput {
    fn new(phase_setting: isize, channel: mpsc::Receiver<isize>) -> Self {
        LoopInput {
            phase_setting: Some(phase_setting),
            channel,
        }
    }
}

impl intcode::Input for LoopInput {
    fn get_input(&mut self) -> isize {
        self.phase_setting
            .take()
            .or_else(|| self.channel.recv().ok())
            .unwrap()
    }
}

struct LoopOutput {
    channel: mpsc::Sender<isize>,
    last_output: isize,
}

impl LoopOutput {
    fn new(channel: mpsc::Sender<isize>) -> Self {
        LoopOutput {
            last_output: 0,
            channel,
        }
    }
}

impl intcode::Output for LoopOutput {
    fn send_output(&mut self, data: isize) {
        self.last_output = data;
        self.channel.send(data).unwrap();
    }
}

fn part2() {
    let signal = (5..10)
        .permutations(5)
        .map(|settings| {
            let (mut inputs, mut outputs): (Vec<_>, Vec<_>) = settings
                .iter()
                .map(|&setting| {
                    let (sender, reciever) = mpsc::channel();
                    (LoopInput::new(setting, reciever), LoopOutput::new(sender))
                })
                .unzip();
            inputs.rotate_right(1);
            outputs[4].channel.send(0).unwrap();
            scope(|s| {
                for (input, output) in inputs.iter_mut().zip(&mut outputs) {
                    s.spawn(move |_| intcode::run(&mut PROGRAM.clone(), input, output));
                }
            })
            .unwrap();
            outputs[4].last_output
        })
        .max()
        .unwrap();
    println!("{}", signal);
}

fn main() {
    part1();
    part2();
}
