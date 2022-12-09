use crate::intcode::{IntCode, State};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day21)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

fn run_program(vm: &mut IntCode, program: &[i64]) -> i64 {
    let mut last_write = 0;

    let mut cur_idx = 0;

    loop {
        match vm.run() {
            State::Waiting => {
                vm.input(program[cur_idx]);
                cur_idx += 1;
            }
            State::Write(n) => {
                last_write = n;
            }
            State::Halted(_) => return last_write,
        };
    }
}

#[aoc(day21, part1)]
pub fn part1(inp: &[i64]) -> i64 {
    #[rustfmt::skip]
    const PROGRAM: [char; 44] = [
        'O', 'R', ' ', 'A', ' ', 'T', '\n',
        'A', 'N', 'D', ' ', 'B', ' ', 'T', '\n',
        'A', 'N', 'D', ' ', 'C', ' ', 'T', '\n',
        'N', 'O', 'T', ' ', 'T', ' ', 'J', '\n',
        'A', 'N', 'D', ' ', 'D', ' ', 'J', '\n',
        'W', 'A', 'L', 'K', '\n'
    ];

    let input = PROGRAM.iter().map(|it| i64::from(*it as u8)).collect_vec();

    let mut vm = IntCode::new(inp);

    run_program(&mut vm, &input)
}

#[aoc(day21, part2)]
pub fn part2(inp: &[i64]) -> i64 {
    #[rustfmt::skip]
    const PROGRAM: [char; 65] = [
        'O', 'R', ' ', 'A', ' ', 'T', '\n',
        'A', 'N', 'D', ' ', 'B', ' ', 'T', '\n',
        'A', 'N', 'D', ' ', 'C', ' ', 'T', '\n',
        'N', 'O', 'T', ' ', 'T', ' ', 'J', '\n',
        'O', 'R', ' ', 'E', ' ', 'T', '\n',
        'O', 'R', ' ', 'H', ' ', 'T', '\n',
        'A', 'N', 'D', ' ', 'T', ' ', 'J', '\n',
        'A', 'N', 'D', ' ', 'D', ' ', 'J', '\n',
        'R', 'U', 'N', '\n'
    ];

    let input = PROGRAM.iter().map(|it| i64::from(*it as u8)).collect_vec();

    let mut vm = IntCode::new(inp);

    run_program(&mut vm, &input)
}
