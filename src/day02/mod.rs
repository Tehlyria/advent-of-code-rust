use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

use crate::intcode::{IntCode, State};

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day2, part1)]
pub fn part1(v: &[i64]) -> i64 {
    let mut input = v.to_owned();
    input[1] = 12;
    input[2] = 2;

    let mut vm = IntCode::new(&input);
    if let State::Halted(num) = vm.run() {
        return num;
    }

    unreachable!("VM did not halt!");
}

#[aoc(day2, part2)]
pub fn part2(v: &[i64]) -> i64 {
    for (first, second) in iproduct!(0..=99, 0..=99) {
        let mut inp = v.to_owned();
        inp[1] = first;
        inp[2] = second;

        let mut vm = IntCode::new(&inp);

        if let State::Halted(19690720) = vm.run() {
            return first * 100 + second;
        }
    }

    unreachable!("Result not found!");
}
