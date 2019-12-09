#[aoc_generator(day1)]
pub fn generate(inp: &str) -> Vec<u64> {
    inp.lines().map(|it| it.parse().unwrap()).collect()
}

fn calc_fuel_while<F>(num: u64, f: F) -> Option<u64>
where
    F: Fn(u64) -> bool,
{
    let mut result = 0u64;
    let mut cur = num;

    loop {
        let div = cur / 3;
        cur = if div <= 2 { 0 } else { div - 2 };
        result += cur;

        if !f(cur) {
            break;
        }
    }

    Some(result)
}

#[aoc(day1, part1)]
pub fn part1(v: &Vec<u64>) -> u64 {
    v.iter()
        .filter_map(|it| calc_fuel_while(*it, |_| false))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(v: &Vec<u64>) -> u64 {
    v.iter()
        .flat_map(|it| calc_fuel_while(*it, |n| n != 0))
        .sum()
}
