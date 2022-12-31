use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generate(inp: &str) -> Vec<u64> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

const fn calc_fuel(num: u64) -> u64 {
    let div = num / 3;
    if div <= 2 {
        0
    } else {
        div - 2
    }
}

const fn calc_fuel_p2(num: u64) -> u64 {
    let mut result = 0u64;
    let mut cur = num;

    loop {
        cur = calc_fuel(cur);
        result += cur;

        if cur == 0 {
            break;
        }
    }

    result
}

#[aoc(day1, part1)]
pub fn part1(v: &[u64]) -> u64 {
    v.iter().fold(0, |acc, it| acc + calc_fuel(*it))
}

#[aoc(day1, part2)]
pub fn part2(v: &[u64]) -> u64 {
    v.iter().fold(0, |acc, it| acc + calc_fuel_p2(*it))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_p1() {
        let inp = vec![(12u64, 2u64), (14, 2), (1969, 654), (100_756, 33583)];

        for (val, expected) in inp {
            let inp = vec![val];
            let res = part1(&inp);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_samples_p2() {
        let inp = vec![(14, 2), (1969, 966), (100_756, 50346)];

        for (val, expected) in inp {
            let inp = vec![val];
            let res = part2(&inp);
            assert_eq!(res, expected);
        }
    }
}
