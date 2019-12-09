use super::intcode::{IntCode, State};

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(",").map(|it| it.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(v: &Vec<i64>) -> i64 {
    let mut input = v.clone();
    input[1] = 12;
    input[2] = 2;

    let mut vm = IntCode::new(input);
    match vm.run() {
        State::Halted(num) => num,
        _ => panic!("Error!"),
    }
}

#[aoc(day2, part2)]
pub fn part2(v: &Vec<i64>) -> i64 {
    for first in 0..=99 {
        for second in 0..=99 {
            let mut inp = v.clone();
            inp[1] = first;
            inp[2] = second;

            let mut vm = IntCode::new(inp);

            match vm.run() {
                State::Halted(19690720) => return first * 100 + second,
                _ => {}
            }
        }
    }

    unreachable!("Result not found!");
}
