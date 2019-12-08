use permutohedron::LexicalPermutation;
use std::fs::File;
use std::io::{BufReader, Read};

fn read_input() -> std::io::Result<String> {
    let file = File::open("input.txt")?;
    let mut file_reader = BufReader::new(file);
    let mut result = String::new();

    match file_reader.read_to_string(&mut result) {
        Ok(_) => Ok(result),
        Err(e) => Err(e),
    }
}

fn setup_memory(content: String) -> Vec<i64> {
    content
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|it| it.parse::<i64>().unwrap())
        .collect()
}

#[derive(PartialEq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

fn check_mode(opcode: i64, val: i64) -> ParameterMode {
    let s = opcode.to_string().chars().rev().collect::<String>();

    if s.len() <= 2 {
        return ParameterMode::Position;
    }

    let tail = s.chars().skip(2).collect::<String>();
    if val >= tail.len() as i64 {
        return ParameterMode::Position;
    }

    match tail.chars().nth(val as usize) {
        Some(c) => {
            return if c == '0' {
                ParameterMode::Position
            } else {
                ParameterMode::Immediate
            };
        }
        _ => panic!("Something went wrong!"),
    }
}

fn load_opcode(opcode: i64, vpc: i64, param_num: i64, ops: &Vec<i64>) -> i64 {
    let val = ops[(vpc + param_num) as usize];
    match check_mode(opcode, param_num - 1) {
        ParameterMode::Position => ops[val as usize],
        ParameterMode::Immediate => val,
    }
}

fn run(ops: &mut Vec<i64>, mut vpc: i64, inp: &[i64; 2]) -> Option<(i64, i64)> {
    let mut inp_num = if vpc == 0 { 0 } else { 1 };

    loop {
        let cur_opcode = ops[vpc as usize];
        if cur_opcode == 99 {
            println!("Halt.");
            return None;
        }

        match cur_opcode % 100 {
            1 => {
                let lhs_val = load_opcode(cur_opcode, vpc, 1, &ops);
                let rhs_val = load_opcode(cur_opcode, vpc, 2, &ops);
                let dst_idx = ops[vpc as usize + 3];
                ops[dst_idx as usize] = lhs_val + rhs_val;

                vpc += 4;
            }
            2 => {
                let lhs_val = load_opcode(cur_opcode, vpc, 1, &ops);
                let rhs_val = load_opcode(cur_opcode, vpc, 2, &ops);
                let dst_idx = ops[vpc as usize + 3];
                ops[dst_idx as usize] = lhs_val * rhs_val;

                vpc += 4;
            }
            3 => {
                let dst_idx = ops[vpc as usize + 1];
                ops[dst_idx as usize] = inp[inp_num as usize];
                inp_num += 1;
                vpc += 2;
            }
            4 => {
                let out_val = load_opcode(cur_opcode, vpc, 1, &ops);
                return Some((vpc + 2, out_val));
                //println!("Write: {}", out_val);
                //vpc += 2;
            }
            5 => {
                let first_val = load_opcode(cur_opcode, vpc, 1, &ops);
                if first_val != 0 {
                    vpc = load_opcode(cur_opcode, vpc, 2, &ops);
                } else {
                    vpc += 3;
                }
            }
            6 => {
                let first_val = load_opcode(cur_opcode, vpc, 1, &ops);
                if first_val == 0 {
                    vpc = load_opcode(cur_opcode, vpc, 2, &ops);
                } else {
                    vpc += 3;
                }
            }
            7 => {
                let lhs_val = load_opcode(cur_opcode, vpc, 1, &ops);
                let rhs_val = load_opcode(cur_opcode, vpc, 2, &ops);
                let dst_idx = ops[vpc as usize + 3];
                ops[dst_idx as usize] = if lhs_val < rhs_val { 1 } else { 0 };
                vpc += 4;
            }
            8 => {
                let lhs_val = load_opcode(cur_opcode, vpc, 1, &ops);
                let rhs_val = load_opcode(cur_opcode, vpc, 2, &ops);
                let dst_idx = ops[vpc as usize + 3];
                ops[dst_idx as usize] = if lhs_val == rhs_val { 1 } else { 0 };
                vpc += 4;
            }
            _ => {
                panic!("Unknown opcode: {}", cur_opcode);
            }
        }
    }
}

fn part_one(mem: &Vec<i64>) {
    let mut inp = mem.clone();

    // generate input
    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    let res = permutations
        .iter()
        .map(|it| {
            let (_, a) = run(&mut inp, 0, &[it[0], 0]).unwrap();
            let (_, b) = run(&mut inp, 0, &[it[1], a]).unwrap();
            let (_, c) = run(&mut inp, 0, &[it[2], b]).unwrap();
            let (_, d) = run(&mut inp, 0, &[it[3], c]).unwrap();
            let (_, e) = run(&mut inp, 0, &[it[4], d]).unwrap();

            e
        })
        .max()
        .unwrap();

    println!("Part One {}", res);
}

fn part_two(mem: &Vec<i64>) {
    // generate input
    let mut data = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    let res = permutations
        .iter()
        .map(|it| {
            let mut cur_inp = 0;

            let mut state_a = mem.clone();
            let mut state_b = mem.clone();
            let mut state_c = mem.clone();
            let mut state_d = mem.clone();
            let mut state_e = mem.clone();

            let mut vpc_a = 0;
            let mut vpc_b = 0;
            let mut vpc_c = 0;
            let mut vpc_d = 0;
            let mut vpc_e = 0;

            let mut last_e = 0;

            loop {
                if let Some((va, a)) = run(&mut state_a, vpc_a, &[it[0], cur_inp]) {
                    vpc_a = va;
                    if let Some((vb, b)) = run(&mut state_b, vpc_b, &[it[1], a]) {
                        vpc_b = vb;
                        if let Some((vc, c)) = run(&mut state_c, vpc_c, &[it[2], b]) {
                            vpc_c = vc;
                            if let Some((vd, d)) = run(&mut state_d, vpc_d, &[it[3], c]) {
                                vpc_d = vd;
                                if let Some((ve, e)) = run(&mut state_e, vpc_e, &[it[4], d]) {
                                    vpc_e = ve;
                                    last_e = e;
                                    cur_inp = last_e;
                                    continue;
                                }
                            }
                        }
                    }
                }

                break;
            }

            last_e
        })
        .max()
        .unwrap();

    println!("Part Two {}", res);
}

fn main() {
    let file_content = read_input();

    match file_content {
        Ok(content) => {
            let memory = setup_memory(content);
            part_one(&memory);
            part_two(&memory);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let op = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let mut state_a = op.clone();
        let mut state_b = op.clone();
        let mut state_c = op.clone();
        let mut state_d = op.clone();
        let mut state_e = op.clone();

        let mut vpc_a = 0;
        let mut vpc_b = 0;
        let mut vpc_c = 0;
        let mut vpc_d = 0;
        let mut vpc_e = 0;

        let mut last_e = 0;
        let mut cur_inp = 0;
        loop {
            if let Some((va, a)) = run(&mut state_a, vpc_a, &[9, cur_inp]) {
                vpc_a = va;
                if let Some((vb, b)) = run(&mut state_b, vpc_b, &[8, a]) {
                    vpc_b = vb;
                    if let Some((vc, c)) = run(&mut state_c, vpc_c, &[7, b]) {
                        vpc_c = vc;
                        if let Some((vd, d)) = run(&mut state_d, vpc_d, &[6, c]) {
                            vpc_d = vd;
                            if let Some((ve, e)) = run(&mut state_e, vpc_e, &[5, d]) {
                                vpc_e = ve;
                                last_e = e;
                                cur_inp = last_e;
                                continue;
                            }
                        }
                    }
                }
            }

            break;
        }

        assert_eq!(last_e, 139629729);
    }

    #[test]
    fn test_two() {
        let op = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let mut state_a = op.clone();
        let mut state_b = op.clone();
        let mut state_c = op.clone();
        let mut state_d = op.clone();
        let mut state_e = op.clone();

        let mut vpc_a = 0;
        let mut vpc_b = 0;
        let mut vpc_c = 0;
        let mut vpc_d = 0;
        let mut vpc_e = 0;

        let mut last_e = 0;
        let mut cur_inp = 0;
        loop {
            if let Some((va, a)) = run(&mut state_a, vpc_a, &[9, cur_inp]) {
                vpc_a = va;
                if let Some((vb, b)) = run(&mut state_b, vpc_b, &[7, a]) {
                    vpc_b = vb;
                    if let Some((vc, c)) = run(&mut state_c, vpc_c, &[8, b]) {
                        vpc_c = vc;
                        if let Some((vd, d)) = run(&mut state_d, vpc_d, &[5, c]) {
                            vpc_d = vd;
                            if let Some((ve, e)) = run(&mut state_e, vpc_e, &[6, d]) {
                                vpc_e = ve;
                                last_e = e;
                                cur_inp = last_e;
                                continue;
                            }
                        }
                    }
                }
            }

            break;
        }

        assert_eq!(last_e, 18216);
    }
}
