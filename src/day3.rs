use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct Position(i64, i64);

fn get_coords(wire: &str) -> Vec<Position> {
    let steps: Vec<&str> = wire.split(",").collect();
    let set = Vec::new();

    let mut last_coord = Position(0, 0);

    steps.iter().fold(set, |mut acc, it| {
        let dir = it.chars().next().unwrap();
        let count = it[1..].parse::<i64>().unwrap();

        for _ in 0..count {
            match dir {
                'U' => {
                    let next = Position(last_coord.0, last_coord.1 + 1);
                    last_coord = next.clone();
                    acc.push(next);
                }
                'R' => {
                    let next = Position(last_coord.0 + 1, last_coord.1);
                    last_coord = next.clone();
                    acc.push(next);
                }
                'D' => {
                    let next = Position(last_coord.0, last_coord.1 - 1);
                    last_coord = next.clone();
                    acc.push(next);
                }
                'L' => {
                    let next = Position(last_coord.0 - 1, last_coord.1);
                    last_coord = next.clone();
                    acc.push(next);
                }
                _ => panic!(),
            }
        }

        acc
    })
}

#[aoc_generator(day3)]
pub fn generate(inp: &str) -> [Vec<Position>; 2] {
    let lines: Vec<&str> = inp.split("\n").collect();

    let first_wire = lines[0];
    let second_wire = lines[1];

    return [get_coords(first_wire), get_coords(second_wire)];
}

fn get_intersections(lhs: &Vec<Position>, rhs: &Vec<Position>) -> Vec<Position> {
    let lhs_set: HashSet<&Position> = HashSet::from_iter(lhs.iter());
    let rhs_set: HashSet<&Position> = HashSet::from_iter(rhs.iter());
    lhs_set.intersection(&rhs_set).map(|it| **it).collect()
}

#[aoc(day3, part1)]
pub fn part1(wires: &[Vec<Position>; 2]) -> i64 {
    let first_wire = wires.get(0).unwrap();
    let second_wire = wires.get(1).unwrap();

    let intersections = get_intersections(&first_wire, &second_wire);

    intersections
        .iter()
        .map(|it| i64::abs(it.0) + i64::abs(it.1))
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn part2(wires: &[Vec<Position>; 2]) -> i64 {
    let first_wire = wires.get(0).unwrap();
    let second_wire = wires.get(1).unwrap();

    let intersections = get_intersections(&first_wire, &second_wire);

    fn pos(v: &Vec<Position>, e: &Position) -> usize {
        v.iter().position(|it| *it == *e).unwrap()
    }

    intersections
        .iter()
        .map(|it| {
            let lpos = pos(first_wire, it) as i64 + 1;
            let rpos = pos(second_wire, it) as i64 + 1;
            lpos + rpos
        })
        .min()
        .unwrap()
}
