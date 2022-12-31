use crate::intcode::{IntCode, State};
use aoc_runner_derive::{aoc, aoc_generator};
use std::io::BufRead;

#[aoc_generator(day25)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day25, part1)]
pub fn part1(inp: &[i64]) -> Option<usize> {
    let mut vm = IntCode::new(inp);

    let mut cur_idx = 0;
    let mut input = String::new();

    let stdin = std::io::stdin();

    // Map of the game:
    // L is the locked door to find the correct weight for
    // Don't pick up items in '#' rooms
    /*
                                    #
                                    |
      #-----------------------------#
      |		                        |
     AIC-#		                    SLSB
      |
      J-----------------#
      |		            |
      #---@             A-#
      |   |             |
    #-# S-P-SH          KB-C6
    |
    L
    */

    loop {
        match vm.run() {
            State::Waiting => {
                if cur_idx == 0 {
                    input = stdin
                        .lock()
                        .lines()
                        .next()
                        .expect("There should be a line of input")
                        .ok()?;
                    if !input.ends_with('\n') {
                        input.push('\n');
                    }
                }

                let next_char = input.chars().nth(cur_idx)?;
                vm.input(next_char as i64);
                cur_idx += 1;
                print!("{next_char}");

                if next_char == '\n' {
                    cur_idx = 0;
                }
            }
            State::Write(n) => {
                print!("{}", char::from_u32(n as u32)?);
            }
            State::Halted(_) => break,
        };
    }

    Some(2_105_377)
}
