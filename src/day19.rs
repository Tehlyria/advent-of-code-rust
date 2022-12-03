use crate::intcode::{IntCode, State};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

#[aoc_generator(day19)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').map(|it| it.parse().unwrap()).collect()
}

#[aoc(day19, part1)]
pub fn part1(inp: &[i64]) -> i64 {
    let mut result = 0;

    for (x, y) in iproduct!(0..50, 0..50) {
        let mut vm = IntCode::new(inp);

        // X
        if let State::Waiting = vm.run() {
            vm.input(x);
        }

        // Y
        if let State::Waiting = vm.run() {
            vm.input(y);
        }

        if let State::Write(n) = vm.run() {
            result += n;
        }
    }

    result
}

fn can_fit_n_in_row(x: usize, y: usize, map: &[Vec<char>], num: usize) -> bool {
    for idx in y..y + num {
        if idx >= map[x].len() {
            return false;
        }

        if map[x][idx] == '.' {
            return false;
        }
    }

    true
}

fn can_fit_n_in_col(x: usize, y: usize, map: &[Vec<char>], num: usize) -> bool {
    for idx in x..x + num {
        if idx >= map.len() {
            return false;
        }

        if map[idx][y] == '.' {
            return false;
        }
    }

    true
}

#[aoc(day19, part2)]
pub fn part2(inp: &[i64]) -> usize {
    let mut map = vec![vec!['.'; 1800]; 1800];

    // Skip first 1000x1000 block
    for (x, y) in iproduct!(1000..map.len(), 1000..map[0].len()) {
        let mut vm = IntCode::new(inp);

        // X
        if let State::Waiting = vm.run() {
            vm.input(x as i64);
        }

        // Y
        if let State::Waiting = vm.run() {
            vm.input(y as i64);
        }

        if let State::Write(n) = vm.run() {
            if n == 1 {
                map[x][y] = '#';
            }
        }
    }

    for (x, y) in iproduct!(0..map.len(), 0..map[0].len()) {
        if can_fit_n_in_row(x, y, &map, 100) && can_fit_n_in_col(x, y, &map, 100) {
            return x * 10_000 + y;
        }
    }

    unreachable!("No solution found!")
}
