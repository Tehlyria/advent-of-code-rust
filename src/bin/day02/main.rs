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

fn run(ops: &mut Vec<usize>) {
    let mut vpc = 0;
    loop {
        let cur_opcode = ops[vpc];
        if cur_opcode == 99 {
            break;
        }

        match cur_opcode {
            1 => {
                let lhs_val = ops[ops[vpc + 1]];
                let rhs_val = ops[ops[vpc + 2]];
                let dst_idx = ops[vpc + 3];
                ops[dst_idx] = lhs_val + rhs_val;

                vpc += 4;
            }
            2 => {
                let lhs_val = ops[ops[vpc + 1]];
                let rhs_val = ops[ops[vpc + 2]];
                let dst_idx = ops[vpc + 3];
                ops[dst_idx] = lhs_val * rhs_val;

                vpc += 4;
            }
            _ => {
                return;
            }
        }
    }
}

fn setup_memory(content: String) -> Vec<usize> {
    content
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|it| it.parse::<usize>().unwrap())
        .collect()
}

fn part_one(mem: &Vec<usize>) {
    let mut cur = mem.clone();

    cur[1] = 12;
    cur[2] = 2;
    run(&mut cur);

    println!("Part One: {}", cur[0]);
}

fn part_two(mem: &Vec<usize>) {
    for first in 0..=99 {
        for second in 0..=99 {
            let mut current = mem.clone();

            current[1] = first;
            current[2] = second;
            run(&mut current);
            if current[0] == 19690720 {
                println!(
                    "Part Two: {} * 100 + {} = {}",
                    first,
                    second,
                    first * 100 + second
                );
                return;
            }
        }
    }
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
        let mut inp: Vec<usize> = vec![1, 0, 0, 0, 99];
        run(&mut inp);
        let expected: Vec<usize> = vec![2, 0, 0, 0, 99];

        assert_eq!(inp, expected);
    }

    #[test]
    fn test_two() {
        let mut inp: Vec<usize> = vec![2, 3, 0, 3, 99];
        run(&mut inp);
        let expected: Vec<usize> = vec![2, 3, 0, 6, 99];

        assert_eq!(inp, expected);
    }

    #[test]
    fn test_three() {
        let mut inp: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
        run(&mut inp);
        let expected: Vec<usize> = vec![2, 4, 4, 5, 99, 9801];

        assert_eq!(inp, expected);
    }

    #[test]
    fn test_four() {
        let mut inp: Vec<usize> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut inp);
        let expected: Vec<usize> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(inp, expected);
    }
}
