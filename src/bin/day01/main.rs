use lazy_static::lazy_static;

lazy_static! {
    static ref MODULES: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
}

fn fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn fuel_modified(mass: u32) -> u32 {
    let mut total_fuel = 0;
    let mut last_fuel = fuel(mass);
    while last_fuel > 0 {
        total_fuel += last_fuel;
        last_fuel = fuel(last_fuel);
    }
    total_fuel
}

fn part1() {
    let total_fuel = MODULES.iter().map(|&mass| fuel(mass)).sum::<u32>();
    println!("{}", total_fuel);
}

fn part2() {
    let total_fuel = MODULES.iter().map(|&mass| fuel_modified(mass)).sum::<u32>();
    println!("{}", total_fuel);
}

fn main() {
    part1();
    part2();
}
