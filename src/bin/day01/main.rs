use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> std::io::Result<Vec<String>> {
    let file = File::open("input.txt")?;
    let file_reader = BufReader::new(file);
    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn calculate_fuel_while<F>(num: u64, f: F) -> Option<u64>
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

fn run<F>(numbers: &Vec<String>, f: F)
where
    F: Fn(u64) -> Option<u64>,
{
    let res = numbers
        .iter()
        .filter_map(|n| n.parse::<u64>().ok())
        .filter_map(f)
        .sum::<u64>();

    println!("{}", res);
}

fn main() {
    let file_content = read_input();
    match file_content {
        Ok(numbers) => {
            print!("Part One: ");
            run(&numbers, |n| calculate_fuel_while(n, |_| false));

            print!("Part Two: ");
            run(&numbers, |n| calculate_fuel_while(n, |num| num != 0u64));
        }
        _ => eprintln!("Error: File not valid!"),
    }
}
