use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::intcode::{IntCode, State};

#[aoc_generator(day13)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day13, part1)]
pub fn part1(inp: &[i64]) -> usize {
    let mut vm = IntCode::new(inp);

    let mut outputs = Vec::new();

    while let State::Write(x) = vm.run() {
        outputs.push(x);
    }

    let chunks = outputs.iter().chunks(3);

    let mut num_block_tiles = 0;
    for chunk in &chunks {
        if let Some(tile_id) = chunk.into_iter().nth(2) {
            if *tile_id == 2 {
                num_block_tiles += 1;
            }
        }
    }

    num_block_tiles
}

fn get_distance_ball_paddle(map: &[Vec<i64>]) -> i64 {
    let mut ball_x = 0;
    let mut paddle_x = 0;

    for row in map {
        for (col_idx, tile) in row.iter().enumerate() {
            if *tile == 3 {
                paddle_x = col_idx;
            } else if *tile == 4 {
                ball_x = col_idx;
            }
        }
    }

    match ball_x.cmp(&paddle_x) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[aoc(day13, part2)]
pub fn part2(inp: &[i64]) -> i64 {
    let mut vm = IntCode::new(inp);
    vm.init_ram(0, 2);

    let mut map = vec![vec![0i64; 40]; 24];

    let tmp_output: &mut [i64; 3] = &mut [0, 0, 0];
    let mut cur_out = 0;
    let mut score = 0;
    loop {
        match vm.run() {
            State::Write(n) => {
                tmp_output[cur_out] = n;
                cur_out = (cur_out + 1) % 3;

                if cur_out == 0 {
                    let [x, y, id] = tmp_output;
                    if *x == -1 && *y == 0 {
                        score = *id;
                    } else {
                        map[*y as usize][*x as usize] = *id;
                    }
                }
            }
            State::Waiting => {
                let paddle_inp = get_distance_ball_paddle(&map);
                vm.input(paddle_inp);
            }
            State::Halted(_) => {
                break;
            }
        };
    }

    score
}
