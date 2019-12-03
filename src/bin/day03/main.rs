use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
struct Position(i64, i64, i64);

fn read_input() -> std::io::Result<Vec<String>> {
    let file = File::open("input.txt")?;
    let file_reader = BufReader::new(file);
    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn get_coords(wire: &String) -> HashSet<Position> {
    let steps: Vec<&str> = wire.split(",").collect();

    let mut set = HashSet::new();
    set.insert(Position(0, 0, 0));

    let mut last_coord = Position(0, 0, 0);

    steps.iter().fold(set, |mut acc, it| {
        let dir = it.chars().next().unwrap();
        let count = it[1..].parse::<i64>().unwrap();

        for _ in 0..count {
            match dir {
                'U' => {
                    let next = Position(last_coord.0, last_coord.1 + 1, last_coord.2 + 1);
                    last_coord = next.clone();
                    acc.insert(next);
                }
                'R' => {
                    let next = Position(last_coord.0 + 1, last_coord.1, last_coord.2 + 1);
                    last_coord = next.clone();
                    acc.insert(next);
                }
                'D' => {
                    let next = Position(last_coord.0, last_coord.1 - 1, last_coord.2 + 1);
                    last_coord = next.clone();
                    acc.insert(next);
                }
                'L' => {
                    let next = Position(last_coord.0 - 1, last_coord.1, last_coord.2 + 1);
                    last_coord = next.clone();
                    acc.insert(next);
                }
                _ => panic!(),
            }
        }

        acc
    })
}

fn run(inp: &Vec<String>) {
    if inp.len() != 2 {
        eprintln!("Error: Did not read two lines!");
        return;
    }

    let first_wire = &inp[0];
    let second_wire = &inp[1];

    let first_coords = get_coords(&first_wire);
    let second_coords = get_coords(&second_wire);

    let first_clean = first_coords
        .iter()
        .map(|it| Position(it.0, it.1, 0))
        .collect::<HashSet<Position>>();

    let second_clean = second_coords
        .iter()
        .map(|it| Position(it.0, it.1, 0))
        .collect::<HashSet<Position>>();

    let intersections = first_clean
        .intersection(&second_clean)
        .collect::<HashSet<&Position>>();

    let min_dist = intersections
        .iter()
        .filter(|it| it.0 != 0 || it.1 != 0)
        .map(|it| i64::abs(it.0) + i64::abs(it.1))
        .min()
        .unwrap();

    println!("Part One: {}", min_dist);

    // intersections of first wire - match by first two members to keep step count
    let mut first_inters_only: Vec<&Position> = first_coords
        .iter()
        .filter(|it| intersections.contains(&Position(it.0, it.1, 0)))
        .collect();

    first_inters_only.sort_by(|lhs, rhs| {
        let l = Position(lhs.0, lhs.1, 0);
        let r = Position(rhs.0, rhs.1, 0);
        return l.cmp(&r);
    });

    // intersections of second wire - match by first two members to keep step count
    let mut second_inters_only: Vec<&Position> = second_coords
        .iter()
        .filter(|it| intersections.contains(&Position(it.0, it.1, 0)))
        .collect();

    second_inters_only.sort_by(|lhs, rhs| {
        let l = Position(lhs.0, lhs.1, 0);
        let r = Position(rhs.0, rhs.1, 0);
        return l.cmp(&r);
    });

    let both = first_inters_only.iter().zip(second_inters_only);

    let shortest_walk = both
        .filter(|(l, r)| (l.0 != 0 || l.1 != 0) && (r.0 != 0 || r.1 != 0))
        .map(|(l, r)| l.2 + r.2)
        .min()
        .unwrap();

    println!("Part Two: {}", shortest_walk);
}

fn main() {
    let file_content = read_input();

    match file_content {
        Ok(content) => run(&content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
