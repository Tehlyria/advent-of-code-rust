use crate::intcode::{IntCode, State};

#[aoc_generator(day9)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(",").map(|it| it.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(v: &Vec<i64>) -> i64 {
    let mut vm = IntCode::new(v.clone());

    loop {
        match vm.run() {
            State::Waiting => vm.input(1),
            State::Write(n) => return n,
            _ => {}
        }
    }
}

#[aoc(day9, part2)]
pub fn part2(v: &Vec<i64>) -> i64 {
    let mut vm = IntCode::new(v.clone());

    loop {
        match vm.run() {
            State::Waiting => vm.input(2),
            State::Write(n) => return n,
            _ => {}
        }
    }
}
