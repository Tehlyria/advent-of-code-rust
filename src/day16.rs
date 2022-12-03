use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[allow(clippy::cast_possible_wrap)]
#[aoc_generator(day16)]
pub fn generate(inp: &str) -> Vec<i32> {
    inp.chars()
        .filter_map(|it| it.to_digit(10).map(|it| it as i32))
        .collect()
}

fn create_pattern(idx: usize) -> Vec<i32> {
    let num_reps = idx + 1;

    let a = std::iter::repeat(0).take(num_reps);
    let b = std::iter::repeat(1).take(num_reps);
    let c = std::iter::repeat(0).take(num_reps);
    let d = std::iter::repeat(-1).take(num_reps);

    a.chain(b).chain(c).chain(d).collect()
}

fn create_new_list(current: &[i32], next: &mut [i32]) {
    let signal_length = current.len();
    assert_eq!(signal_length, next.len());

    for idx in 0..signal_length {
        let pattern = create_pattern(idx);

        let new_digit = current
            .iter()
            .zip(pattern.iter().cycle().dropping(1))
            .map(|(lhs, rhs)| lhs * rhs)
            .sum::<i32>();
        next[idx] = new_digit.abs() % 10;
    }
}

#[aoc(day16, part1)]
pub fn part1(inp: &[i32]) -> String {
    let mut inp = inp.to_vec();
    let mut next = vec![0; inp.len()];

    for _ in 0..100 {
        create_new_list(&inp, &mut next);
        std::mem::swap(&mut inp, &mut next);
    }

    inp.iter().take(8).join("")
}

#[aoc(day16, part2)]
pub fn part2(inp: &[i32]) -> Option<String> {
    let mut inp = inp.repeat(10_000);
    let mut next = vec![0; inp.len()];

    let offset = inp[..7].iter().join("").parse::<usize>().ok()?;

    for idx in 0..100 {
        let mut partial_sum = inp[idx..].iter().sum::<i32>();
        for i in idx..inp.len() {
            next[i] = partial_sum % 10;
            partial_sum -= inp[i];
        }

        std::mem::swap(&mut inp, &mut next);
    }

    Some(inp.iter().skip(offset).take(8).join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_generation() {
        assert_eq!(create_pattern(0), [0, 1, 0, -1]);
        assert_eq!(create_pattern(1), [0, 0, 1, 1, 0, 0, -1, -1]);
        assert_eq!(create_pattern(2), [0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]);
    }
}
