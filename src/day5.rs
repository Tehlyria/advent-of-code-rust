use crate::intcode::{IntCode, State};

#[aoc_generator(day5)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(",").map(|it| it.parse().unwrap()).collect()
}

fn run_vm_with_input(v: Vec<i64>, inp: i64) -> i64 {
    let input = v.clone();

    let mut vm = IntCode::new(input);

    loop {
        match vm.run() {
            State::Write(0) => {}
            State::Write(n) => return n,
            State::Halted(n) => return n,
            State::Waiting => vm.input(inp),
        }
    }
}

#[aoc(day5, part1)]
pub fn part1(v: &Vec<i64>) -> i64 {
    run_vm_with_input(v.clone(), 1)
}

#[aoc(day5, part2)]
pub fn part2(v: &Vec<i64>) -> i64 {
    run_vm_with_input(v.clone(), 5)
}
