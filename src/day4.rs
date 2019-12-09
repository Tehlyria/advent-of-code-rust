use std::iter::Filter;
use std::ops::Range;

pub struct InputRange(i64, i64);

impl InputRange {
    pub fn filter_valid(&self) -> Filter<Range<i64>, fn(&i64) -> bool> {
        (self.0..self.1).filter(|it| is_valid(it.to_string()))
    }
}

#[aoc_generator(day4)]
pub fn generate(inp: &str) -> InputRange {
    let spl: Vec<&str> = inp.split("-").collect();

    InputRange(spl[0].parse().unwrap(), spl[1].parse().unwrap())
}

fn increasing(num: &Vec<char>) -> bool {
    num.windows(2).all(|it| it[0] <= it[1])
}

fn two_adjacent_eq(num: &Vec<char>) -> bool {
    num.windows(2).any(|it| it[0] == it[1])
}

fn one_double_pair(num: String) -> bool {
    num.chars()
        .map(|it| num.chars().filter(|e| *e == it).count())
        .any(|it| it == 2)
}

fn is_valid(num: String) -> bool {
    let chrs = num.chars().collect::<Vec<char>>();

    if !increasing(&chrs) {
        return false;
    }

    if !two_adjacent_eq(&chrs) {
        return false;
    }

    true
}

#[aoc(day4, part1)]
pub fn part1(r: &InputRange) -> i64 {
    r.filter_valid().count() as i64
}

#[aoc(day4, part2)]
pub fn part2(r: &InputRange) -> i64 {
    r.filter_valid()
        .filter(|it| one_double_pair(it.to_string()))
        .count() as i64
}
