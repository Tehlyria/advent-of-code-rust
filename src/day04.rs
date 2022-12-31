use std::iter::Filter;
use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr)]
#[display("{0}-{1}")]
pub struct InputRange(i64, i64);

impl InputRange {
    pub fn filter_valid(&self) -> Filter<Range<i64>, fn(&i64) -> bool> {
        (self.0..self.1).filter(|it| is_valid(&it.to_string()))
    }

    pub fn filter_valid_p2(&self) -> Filter<Range<i64>, fn(&i64) -> bool> {
        (self.0..self.1).filter(|it| is_valid(&it.to_string()) && one_double_pair(&it.to_string()))
    }
}

#[aoc_generator(day4)]
pub fn generate(inp: &str) -> Option<InputRange> {
    inp.parse().ok()
}

fn increasing(num: &[char]) -> bool {
    num.windows(2).all(|it| it[0] <= it[1])
}

fn two_adjacent_eq(num: &[char]) -> bool {
    num.windows(2).any(|it| it[0] == it[1])
}

fn one_double_pair(num: &str) -> bool {
    num.chars()
        .map(|it| num.chars().filter(|e| *e == it).count())
        .any(|it| it == 2)
}

fn is_valid(num: &str) -> bool {
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
pub fn part1(r: &InputRange) -> usize {
    r.filter_valid().count()
}

#[aoc(day4, part2)]
pub fn part2(r: &InputRange) -> usize {
    r.filter_valid_p2().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_p1() {
        {
            let range = InputRange(111_111, 111_112);
            assert_eq!(1, part1(&range));
        }
        {
            let range = InputRange(223_450, 223_451);
            assert_eq!(0, part1(&range));
        }
        {
            let range = InputRange(123_789, 123_790);
            assert_eq!(0, part1(&range));
        }
    }

    #[test]
    fn test_samples_p2() {
        {
            let range = InputRange(112_233, 112_234);
            assert_eq!(1, part2(&range));
        }
        {
            let range = InputRange(123_444, 123_445);
            assert_eq!(0, part2(&range));
        }
        {
            let range = InputRange(111_122, 111_123);
            assert_eq!(1, part2(&range));
        }
    }
}
