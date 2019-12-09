use crate::intcode::{IntCode, State};
use permutohedron::LexicalPermutation;

#[aoc_generator(day7)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(",").map(|it| it.parse().unwrap()).collect()
}

fn permutate(mut data: [i64; 5]) -> Vec<Vec<i64>> {
    let mut permutations = Vec::new();

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    permutations
}

#[aoc(day7, part1)]
pub fn part1(mem: &Vec<i64>) -> i64 {
    permutate([0, 1, 2, 3, 4])
        .iter()
        .map(|it| {
            it.iter().fold(0, |acc, elem| {
                let mut vm = IntCode::new(mem.clone());

                match vm.run_with_input(0, &[*elem, acc]) {
                    State::Halted(n) => n,
                    State::Write(n) => n,
                    _ => panic!("Too many inputs?"),
                }
            })
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part2(mem: &Vec<i64>) -> i64 {
    permutate([5, 6, 7, 8, 9])
        .iter()
        .map(|it| {
            let mut last = 0;
            let mut last_output = 0;
            let mut needs_phases = true;

            let mut vms = vec![
                IntCode::new(mem.clone()),
                IntCode::new(mem.clone()),
                IntCode::new(mem.clone()),
                IntCode::new(mem.clone()),
                IntCode::new(mem.clone()),
            ];

            loop {
                let inp_idx = if needs_phases { 0 } else { 1 };

                for i in 0..it.len() {
                    let it_val = it[i];
                    let cur_vm = &mut vms[i];

                    match cur_vm.run_with_input(inp_idx, &[it_val, last]) {
                        State::Halted(_) => return last_output,
                        State::Write(n) => {
                            last_output = n;
                            last = n;
                        }
                        _ => panic!("Too many inputs?"),
                    }
                }

                needs_phases = false;
            }
        })
        .max()
        .unwrap()
}
