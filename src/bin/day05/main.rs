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

fn run(ops: &mut Vec<i64>, inp: i64) {
    let mut vpc = 0i64;
    loop {
        let cur_opcode = ops[vpc as usize];
        if cur_opcode == 99 {
            println!("Halt.");
            break;
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
                ops[dst_idx as usize] = inp;
                vpc += 2;
            }
            4 => {
                let out_val = load_opcode(cur_opcode, vpc, 1, &ops);
                println!("Write: {}", out_val);
                vpc += 2;
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

fn setup_memory(content: String) -> Vec<i64> {
    content
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|it| it.parse::<i64>().unwrap())
        .collect()
}

fn part_one(mem: &Vec<i64>) {
    let mut cur = mem.clone();
    run(&mut cur, 1);
}

fn part_two(mem: &Vec<i64>) {
    let mut cur = mem.clone();
    run(&mut cur, 5);
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
    fn test_mode() {
        assert_eq!(check_mode(1002, 0), ParameterMode::Position);
        assert_eq!(check_mode(1002, 1), ParameterMode::Immediate);
        assert_eq!(check_mode(1002, 2), ParameterMode::Position);
    }
}
