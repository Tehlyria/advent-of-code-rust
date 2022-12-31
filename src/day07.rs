use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{izip, Itertools};

use crate::intcode::{IntCode, State};

#[aoc_generator(day7)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day7, part1)]
pub fn part1(mem: &[i64]) -> Option<i64> {
    [0, 1, 2, 3, 4]
        .iter()
        .permutations(5)
        .map(|it| {
            it.iter().fold(0, |acc, &&elem| {
                let mut vm = IntCode::new(mem);

                match vm.run_with_input(0, &[elem, acc]) {
                    State::Write(n) | State::Halted(n) => n,
                    State::Waiting => panic!("Too many inputs?"),
                }
            })
        })
        .max()
}

#[aoc(day7, part2)]
pub fn part2(mem: &[i64]) -> Option<i64> {
    [5, 6, 7, 8, 9]
        .iter()
        .permutations(5)
        .map(|it| {
            let mut last = 0;
            let mut last_output = 0;
            let mut needs_phases = true;

            let mut vms = vec![
                IntCode::new(mem),
                IntCode::new(mem),
                IntCode::new(mem),
                IntCode::new(mem),
                IntCode::new(mem),
            ];

            loop {
                let inp_idx = usize::from(!needs_phases);

                for (&&val, vm) in izip!(&it, &mut vms) {
                    match vm.run_with_input(inp_idx, &[val, last]) {
                        State::Halted(_) => return last_output,
                        State::Write(n) => {
                            last_output = n;
                            last = n;
                        }
                        State::Waiting => panic!("Too many inputs?"),
                    }
                }

                needs_phases = false;
            }
        })
        .max()
}
