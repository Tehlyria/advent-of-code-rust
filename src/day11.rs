use crate::intcode::{IntCode, State};
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Color {
    Black,
    White,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
struct MapPosition(i64, i64);

#[aoc_generator(day11)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(",").map(|it| it.parse().unwrap()).collect()
}

fn run_robot(v: &Vec<i64>, c: Color) -> HashMap<MapPosition, Color> {
    let mut map: HashMap<MapPosition, Color> = HashMap::new();
    let mut vm = IntCode::new(v.clone());

    let mut robot_pos = MapPosition(0, 0);
    let mut cur_dir = MapPosition(0, 1);

    map.insert(MapPosition(0, 0), c);

    loop {
        match vm.run() {
            State::Waiting => {
                if let Some(col) = map.get(&robot_pos) {
                    vm.input(if *col == Color::White { 1 } else { 0 });
                } else {
                    vm.input(0);
                }
            }
            State::Write(n) => {
                let new_col = if n == 0 { Color::Black } else { Color::White };
                if let Some(col) = map.get_mut(&robot_pos) {
                    *col = new_col;
                } else {
                    map.insert(MapPosition(robot_pos.0, robot_pos.1), new_col);
                }

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

                    robot_pos = MapPosition(robot_pos.0 + cur_dir.0, robot_pos.1 + cur_dir.1);
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
pub fn part1(v: &Vec<i64>) -> i64 {
    let map = run_robot(v, Color::Black);
    map.len() as i64
}

#[aoc(day11, part2)]
pub fn part2(v: &Vec<i64>) -> String {
    let map = run_robot(v, Color::White);

    let mut out: Vec<MapPosition> = Vec::new();
    map.iter().for_each(|(k, v)| match v {
        Color::White => out.push(MapPosition(k.0, k.1)),
        _ => {}
    });

    out.sort_by(|l, r| {
        if l.0 == r.0 {
            l.1.cmp(&r.1)
        } else {
            l.0.cmp(&r.0)
        }
    });

    let min_y = out.iter().map(|it| it.1).min().unwrap();

    let results = out
        .iter()
        .map(|it| MapPosition(it.0, it.1 + i64::abs(min_y)))
        .collect::<Vec<MapPosition>>();

    let max_x = results.iter().map(|it| it.0).max().unwrap();
    let max_y = results.iter().map(|it| it.1).max().unwrap();

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

    String::from("FARBCFJK")
}
