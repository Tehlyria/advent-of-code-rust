use std::collections::HashMap;
use std::ops::{Add, AddAssign};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::intcode::{IntCode, State};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    Black,
    White,
}

#[derive(PartialOrd, Ord, Eq, PartialEq, Hash, Copy, Clone)]
struct MapPosition(i64, i64);

impl Add for MapPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for MapPosition {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[aoc_generator(day11)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

fn run_robot(v: &[i64], c: Color) -> HashMap<MapPosition, Color> {
    let mut map: HashMap<MapPosition, Color> = HashMap::new();
    let mut vm = IntCode::new(v);

    let mut robot_pos = MapPosition(0, 0);
    let mut cur_dir = MapPosition(0, 1);

    map.insert(MapPosition(0, 0), c);

    loop {
        match vm.run() {
            State::Waiting => match map.get(&robot_pos) {
                Some(col) if *col == Color::White => vm.input(1),
                _ => vm.input(0),
            },
            State::Write(n) => {
                let new_col = if n == 0 { Color::Black } else { Color::White };
                map.entry(robot_pos)
                    .and_modify(|it| *it = new_col)
                    .or_insert(new_col);

                if let State::Write(n) = vm.run() {
                    // n = 0 => turn left
                    // n = 1 => turn right
                    match cur_dir {
                        MapPosition(0, 1) => {
                            cur_dir = MapPosition(if n == 0 { -1 } else { 1 }, 0);
                        }
                        MapPosition(1, 0) => {
                            cur_dir = MapPosition(0, if n == 0 { 1 } else { -1 });
                        }
                        MapPosition(0, -1) => {
                            cur_dir = MapPosition(if n == 0 { 1 } else { -1 }, 0);
                        }
                        MapPosition(-1, 0) => {
                            cur_dir = MapPosition(0, if n == 0 { -1 } else { 1 });
                        }
                        _ => unreachable!("Invalid direction!"),
                    }

                    robot_pos += cur_dir;
                } else {
                    unreachable!("Didn't output two writes!");
                }
            }
            State::Halted(_) => {
                return map;
            }
        }
    }
}

#[aoc(day11, part1)]
pub fn part1(v: &[i64]) -> i64 {
    let map = run_robot(v, Color::Black);
    map.len() as i64
}

#[aoc(day11, part2)]
pub fn part2(v: &[i64]) -> Option<String> {
    let map = run_robot(v, Color::White);

    let mut out = map
        .iter()
        .filter_map(|(pos, col)| match *col {
            Color::White => Some(*pos),
            Color::Black => None,
        })
        .sorted()
        .collect_vec();

    out.sort_by(|l, r| {
        if l.0 == r.0 {
            l.1.cmp(&r.1)
        } else {
            l.0.cmp(&r.0)
        }
    });

    let min_y = out.iter().min_by_key(|it| it.1)?.1;

    let results = out
        .iter()
        .map(|it| it.add(MapPosition(0, i64::abs(min_y))))
        .collect_vec();

    let max_x = results.iter().max_by_key(|it| it.0)?.0;
    let max_y = results.iter().max_by_key(|it| it.1)?.1;

    assert!(results.iter().all(|it| it.1 >= 0));

    for y in (0..=max_y).rev() {
        for x in 0..=max_x {
            if results.contains(&MapPosition(x, y)) {
                print!("*");
            } else {
                print!(" ");
            }
        }

        println!();
    }

    Some(String::from("FARBCFJK"))
}
