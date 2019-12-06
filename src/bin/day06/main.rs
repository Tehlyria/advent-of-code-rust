use std::collections::HashMap;
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

fn get_planets(inp: &String) -> (String, String) {
    let res = inp
        .split(")")
        .map(|it| String::from(it))
        .collect::<Vec<String>>();

    (res[0].clone(), res[1].clone())
}

fn count_all(dag: &HashMap<String, Vec<String>>) -> u64 {
    fn count_all_impl(cur_node: &Vec<String>, dag: &HashMap<String, Vec<String>>) -> u64 {
        let mut current_count = cur_node.len() as u64;

        for elem in cur_node {
            if dag.contains_key(elem) {
                current_count += count_all_impl(dag.get(elem).unwrap(), &dag);
            }
        }

        return current_count;
    }

    dag.iter()
        .fold(0, |acc, (_, v)| acc + count_all_impl(v, dag))
}

fn parse_map(inp: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut dag: HashMap<String, Vec<String>> = HashMap::new();

    for elem in inp {
        let (l, r) = get_planets(&elem);
        if dag.contains_key(&l) {
            dag.get_mut(&l).unwrap().push(r);
        } else {
            dag.insert(l, vec![r]);
        }
    }

    dag
}

fn part_one(dag: &HashMap<String, Vec<String>>) {
    let result = count_all(&dag);
    println!("Part One {}", result);
}

fn find_node(
    current: String,
    target: &String,
    path: &mut Vec<String>,
    dag: &HashMap<String, Vec<String>>,
) -> bool {
    if current == *target {
        return true;
    } else {
        if dag.contains_key(&current) {
            for chld in dag.get(&current).unwrap() {
                if find_node(chld.clone(), &target, path, dag) {
                    path.push(chld.clone());
                    return true;
                }
            }
        }
    }

    return false;
}

fn part_two(dag: &HashMap<String, Vec<String>>) {
    let start = String::from("COM");

    // find YOU
    let myself = String::from("YOU");
    let mut to_me: Vec<String> = Vec::new();
    find_node(start.clone(), &myself, &mut to_me, &dag);
    to_me.push(start.clone());
    to_me.reverse();

    // find SAN
    let santa = String::from("SAN");
    let mut to_santa: Vec<String> = Vec::new();
    find_node(start.clone(), &santa, &mut to_santa, &dag);
    to_santa.push(start.clone());
    to_santa.reverse();

    let mine = to_me.iter().filter(|it| !to_santa.contains(it)).count();
    let san = to_santa.iter().filter(|it| !to_me.contains(it)).count();

    println!("Part Two {}", mine + san - 2);
}

fn main() {
    let file_content = read_input();

    match file_content {
        Ok(content) => {
            let parsed = parse_map(content);
            part_one(&parsed);
            part_two(&parsed);
        }
        _ => eprintln!("Error: File not valid!"),
    }
}
