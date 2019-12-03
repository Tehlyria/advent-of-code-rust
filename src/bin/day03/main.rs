use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
struct Position(i64, i64, i64);

impl Position {
    fn copy(p: &Position) -> Self {
        Position(p.0, p.1, p.2)
    }
}

fn read_input() -> std::io::Result<Vec<String>> {
    let file = File::open("input.txt")?;
    let file_reader = BufReader::new(file);
    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn not_center(pos: &Position) -> bool {
    pos.0 != 0 || pos.1 != 0
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

fn get_intersections(lhs: &HashSet<Position>, rhs: &HashSet<Position>) -> Vec<Position> {
    fn remove_steps(pos: &Position) -> Position {
        Position(pos.0, pos.1, 0)
    }

    let first_clean = lhs.iter().map(remove_steps).collect::<HashSet<Position>>();
    let second_clean = rhs.iter().map(remove_steps).collect::<HashSet<Position>>();

    first_clean
        .intersection(&second_clean)
        .map(Position::copy)
        .collect()
}

fn inters_with_steps(
    lhs: &HashSet<Position>,
    rhs: &HashSet<Position>,
    inters: &Vec<Position>,
) -> Vec<(Position, Position)> {
    fn get_inters(v: &HashSet<Position>, inters: &Vec<Position>) -> Vec<Position> {
        v.iter()
            .filter(|it| inters.contains(&Position(it.0, it.1, 0)))
            .map(Position::copy)
            .collect()
    }

    fn sort_pos(range: &mut Vec<Position>) {
        range.sort_by(|l, r| {
            let ll = Position(l.0, l.1, 0);
            let rr = Position(r.0, r.1, 0);
            return ll.cmp(&rr);
        })
    }

    let mut first_inters_only = get_inters(&lhs, &inters);
    let mut second_inters_only = get_inters(&rhs, &inters);

    sort_pos(&mut first_inters_only);
    sort_pos(&mut second_inters_only);

    first_inters_only
        .into_iter()
        .zip(second_inters_only)
        .collect::<Vec<(Position, Position)>>()
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

    let intersections = get_intersections(&first_coords, &second_coords);

    let min_dist = intersections
        .iter()
        .filter(|it| not_center(*it))
        .map(|it| i64::abs(it.0) + i64::abs(it.1))
        .min()
        .unwrap();

    println!("Part One: {}", min_dist);

    let zipped = inters_with_steps(&first_coords, &second_coords, &intersections);

    let shortest_walk = zipped
        .iter()
        .filter(|(l, r)| not_center(l) && not_center(r))
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
