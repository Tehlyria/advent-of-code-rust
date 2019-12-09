use std::collections::HashMap;

fn get_planets(inp: &String) -> (String, String) {
    let res: Vec<String> = inp.split(")").map(String::from).collect();

    (res[0].clone(), res[1].clone())
}

#[aoc_generator(day6)]
pub fn generate(inp: &str) -> HashMap<String, Vec<String>> {
    let lines: Vec<String> = inp.lines().map(String::from).collect();

    let mut dag: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let (l, r) = get_planets(&line);
        if dag.contains_key(&l) {
            dag.get_mut(&l).unwrap().push(r);
        } else {
            dag.insert(l, vec![r]);
        }
    }

    dag
}

fn count_all(dag: &HashMap<String, Vec<String>>) -> i64 {
    fn count_all_impl(cur: &Vec<String>, dag: &HashMap<String, Vec<String>>) -> i64 {
        let mut count = cur.len() as i64;

        for elem in cur {
            if dag.contains_key(elem) {
                count += count_all_impl(dag.get(elem).unwrap(), &dag);
            }
        }

        count
    }

    dag.iter()
        .fold(0, |acc, (_, v)| acc + count_all_impl(v, dag))
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

fn find(target: String, dag: &HashMap<String, Vec<String>>) -> Vec<String> {
    let start = String::from("COM");

    let mut path: Vec<String> = Vec::new();
    find_node(start.clone(), &target, &mut path, &dag);
    path.push(start.clone());
    path.reverse();

    path
}

#[aoc(day6, part1)]
pub fn part1(dag: &HashMap<String, Vec<String>>) -> i64 {
    count_all(&dag)
}

#[aoc(day6, part2)]
pub fn part2(dag: &HashMap<String, Vec<String>>) -> i64 {
    let path_to_me = find(String::from("YOU"), &dag);
    let path_to_santa = find(String::from("SAN"), &dag);

    let mine = path_to_me
        .iter()
        .filter(|it| !path_to_santa.contains(it))
        .count();
    let san = path_to_santa
        .iter()
        .filter(|it| !path_to_me.contains(it))
        .count();

    (mine + san - 2) as i64
}
