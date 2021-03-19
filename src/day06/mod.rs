use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr)]
#[display("{0}){1}")]
struct PlanetPair(String, String);

#[aoc_generator(day6)]
pub fn generate(inp: &str) -> HashMap<String, Vec<String>> {
    inp.lines()
        .map(|it| it.parse::<PlanetPair>().unwrap())
        .fold(HashMap::new(), |mut acc, it| {
            acc.entry(it.0).or_insert_with(Vec::new).push(it.1);
            acc
        })
}

fn count_all(dag: &HashMap<String, Vec<String>>) -> usize {
    fn count_all_impl(cur: &[String], dag: &HashMap<String, Vec<String>>) -> usize {
        let mut count = cur.len();

        for elem in cur {
            if let Some(node) = dag.get(elem) {
                count += count_all_impl(node, &dag);
            }
        }

        count
    }

    dag.iter()
        .fold(0, |acc, (_, v)| acc + count_all_impl(v, dag))
}

fn find_node(
    current: &str,
    target: &str,
    path: &mut Vec<String>,
    dag: &HashMap<String, Vec<String>>,
) -> bool {
    if *current == *target {
        return true;
    } else if let Some(node) = dag.get(current) {
        for chld in node {
            if find_node(chld, &target, path, dag) {
                path.push(chld.clone());
                return true;
            }
        }
    }

    false
}

fn find(target: String, dag: &HashMap<String, Vec<String>>) -> Vec<String> {
    let start = String::from("COM");

    let mut path: Vec<String> = Vec::new();
    find_node(&start, &target, &mut path, &dag);
    path.push(start);
    path.reverse();

    path
}

#[aoc(day6, part1)]
pub fn part1(dag: &HashMap<String, Vec<String>>) -> usize {
    count_all(&dag)
}

#[aoc(day6, part2)]
pub fn part2(dag: &HashMap<String, Vec<String>>) -> usize {
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

    mine + san - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = "COM)B\n\
                          B)C\n\
                          C)D\n\
                          D)E\n\
                          E)F\n\
                          B)G\n\
                          G)H\n\
                          D)I\n\
                          E)J\n\
                          J)K\n\
                          K)L";

        let data = generate(inp);
        assert_eq!(42, part1(&data));
    }

    #[test]
    fn test_sample_p2() {
        let inp = "COM)B\n\
                          B)C\n\
                          C)D\n\
                          D)E\n\
                          E)F\n\
                          B)G\n\
                          G)H\n\
                          D)I\n\
                          E)J\n\
                          J)K\n\
                          K)YOU\n\
                          I)SAN";

        let data = generate(inp);
        assert_eq!(4, part2(&data));
    }
}
