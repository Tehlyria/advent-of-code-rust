use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr)]
pub enum CableSection {
    #[display("R{0}")]
    Right(i64),

    #[display("D{0}")]
    Down(i64),

    #[display("L{0}")]
    Left(i64),

    #[display("U{0}")]
    Up(i64),
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct Position(i64, i64);

fn get_coords(wire: &str) -> Vec<Position> {
    wire.split(',')
        .filter_map(|it| it.parse::<CableSection>().ok())
        .fold(Vec::new(), |mut acc, it| {
            let last_pos = *acc.last().unwrap_or(&Position(0, 0));

            let positions = match it {
                CableSection::Right(n) => (1..=n)
                    .map(|it| Position(last_pos.0 + it, last_pos.1))
                    .collect_vec(),
                CableSection::Down(n) => (1..=n)
                    .map(|it| Position(last_pos.0, last_pos.1 - it))
                    .collect_vec(),
                CableSection::Left(n) => (1..=n)
                    .map(|it| Position(last_pos.0 - it, last_pos.1))
                    .collect_vec(),
                CableSection::Up(n) => (1..=n)
                    .map(|it| Position(last_pos.0, last_pos.1 + it))
                    .collect_vec(),
            };

            acc.extend(positions);
            acc
        })
}

#[aoc_generator(day3)]
pub fn generate(inp: &str) -> [Vec<Position>; 2] {
    let lines = inp.split('\n').collect_vec();

    let first_wire = lines[0];
    let second_wire = lines[1];

    [get_coords(first_wire), get_coords(second_wire)]
}

fn get_intersections(lhs: &[Position], rhs: &[Position]) -> Vec<Position> {
    let lhs_set = lhs.iter().collect::<HashSet<_>>();
    let rhs_set = rhs.iter().collect::<HashSet<_>>();
    lhs_set.intersection(&rhs_set).map(|it| **it).collect()
}

#[aoc(day3, part1)]
pub fn part1(wires: &[Vec<Position>; 2]) -> Option<i64> {
    let [first_wire, second_wire] = wires;
    let intersections = get_intersections(first_wire, second_wire);
    intersections.iter().map(|it| it.0.abs() + it.1.abs()).min()
}

#[aoc(day3, part2)]
pub fn part2(wires: &[Vec<Position>; 2]) -> Option<i64> {
    fn pos(v: &[Position], e: &Position) -> Option<i64> {
        v.iter().position(|it| *it == *e).map(|it| it as i64 + 1)
    }

    let [first_wire, second_wire] = wires;
    let intersections = get_intersections(first_wire, second_wire);

    intersections
        .iter()
        .filter_map(|it| {
            let lpos = pos(first_wire, it);
            let rpos = pos(second_wire, it);
            lpos.and_then(|l| rpos.map(|r| l + r))
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_p1() {
        {
            let inp = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                              U62,R66,U55,R34,D71,R55,D58,R83";
            let data = generate(inp);

            let res = part1(&data);
            assert_eq!(res, Some(159));
        }

        {
            let inp = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                              U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
            let data = generate(inp);

            let res = part1(&data);
            assert_eq!(res, Some(135));
        }
    }

    #[test]
    fn test_samples_p2() {
        {
            let inp = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                              U62,R66,U55,R34,D71,R55,D58,R83";
            let data = generate(inp);

            let res = part2(&data);
            assert_eq!(res, Some(610));
        }

        {
            let inp = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                              U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
            let data = generate(inp);

            let res = part2(&data);
            assert_eq!(res, Some(410));
        }
    }
}
