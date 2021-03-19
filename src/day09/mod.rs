use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{IntCode, State};

#[aoc_generator(day9)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').map(|it| it.parse().unwrap()).collect()
}

fn run_vm(ram: &[i64], inp: i64) -> i64 {
    let mut vm = IntCode::new(ram);

    loop {
        match vm.run() {
            State::Waiting => vm.input(inp),
            State::Write(n) => return n,
            _ => {}
        }
    }
}

#[aoc(day9, part1)]
pub fn part1(v: &[i64]) -> i64 {
    run_vm(v, 1)
}

#[aoc(day9, part2)]
pub fn part2(v: &[i64]) -> i64 {
    run_vm(v, 2)
}
