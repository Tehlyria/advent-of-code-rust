use crate::intcode::{IntCode, State};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use std::collections::HashMap;

#[aoc_generator(day17)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').map(|it| it.parse().unwrap()).collect()
}

fn generate_map(inp: &[i64]) -> Vec<Vec<char>> {
    let mut map: HashMap<(i64, i64), char> = HashMap::new();

    let mut vm = IntCode::new(inp);

    let mut cur_x = 0;
    let mut cur_y = 0;

    while let State::Write(n) = vm.run() {
        let c = char::from_u32(n as u32).expect("Not a character");

        if c == '\n' {
            cur_y += 1;
            cur_x = 0;
        } else {
            map.insert((cur_y, cur_x), c);
            cur_x += 1;
        }
    }

    assert!(vm.is_halted());

    let &(max_y, _) = map
        .keys()
        .max_by_key(|(y, _)| y)
        .expect("Map has to have a max y value");
    let &(_, max_x) = map
        .keys()
        .max_by_key(|(_, x)| x)
        .expect("Map has to have a max x value");

    let mut result = vec![];

    for y in 0..=max_y {
        let mut row = vec![];
        for x in 0..=max_x {
            let c = map.get(&(y, x)).copied().unwrap_or(' ');
            if c != '\n' {
                row.push(c);
            }
        }
        result.push(row);
    }

    result
}

#[aoc(day17, part1)]
pub fn part1(inp: &[i64]) -> usize {
    let map = generate_map(inp);
    //print_map(&map);

    let mut result = vec![];

    for (y, x) in iproduct!(1..map.len() - 1, 1..map[0].len() - 1) {
        let is_intersection = map[y][x] == '#'
            && map[y - 1][x] == '#'
            && map[y + 1][x] == '#'
            && map[y][x - 1] == '#'
            && map[y][x + 1] == '#';

        if is_intersection {
            result.push((x, y));
        }
    }

    result.iter().map(|(l, r)| l * r).sum()
}

#[aoc(day17, part2)]
pub fn part2(inp: &[i64]) -> i64 {
    // Print the map & resolve the path by hand...
    const MAIN: [char; 20] = [
        'A', ',', 'B', ',', 'A', ',', 'C', ',', 'A', ',', 'B', ',', 'C', ',', 'B', ',', 'C', ',',
        'B', '\n',
    ];
    const FUNC_A: [char; 18] = [
        'R', ',', '8', ',', 'L', ',', '1', '0', ',', 'L', ',', '1', '2', ',', 'R', ',', '4', '\n',
    ];
    const FUNC_B: [char; 17] = [
        'R', ',', '8', ',', 'L', ',', '1', '2', ',', 'R', ',', '4', ',', 'R', ',', '4', '\n',
    ];
    const FUNC_C: [char; 13] = [
        'R', ',', '8', ',', 'L', ',', '1', '0', ',', 'R', ',', '8', '\n',
    ];
    const CONT_FEED: [char; 2] = ['n', '\n'];

    let program_input = MAIN
        .iter()
        .chain(FUNC_A.iter())
        .chain(FUNC_B.iter())
        .chain(FUNC_C.iter())
        .chain(CONT_FEED.iter())
        .map(|it| i64::from(*it as u8))
        .collect_vec();

    let mut vm = IntCode::new(inp);
    vm.init_ram(0, 2);

    let mut cur_idx = 0;

    let mut last_write = 0;
    loop {
        match vm.run() {
            State::Waiting => {
                vm.input(program_input[cur_idx]);
                cur_idx += 1;
            }
            State::Write(n) => {
                last_write = n;
            }
            State::Halted(_) => {
                return last_write;
            }
        };
    }
}
