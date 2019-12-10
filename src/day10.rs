use std::cmp::Ordering;
use std::f64::consts::PI;

#[derive(PartialEq, Eq, Hash)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> Vec<Position> {
    let mut cur_line = 0;

    let mut res: Vec<Position> = Vec::new();

    for l in inp.lines() {
        let mut cur_char = 0;

        for c in l.chars() {
            match c {
                '#' => res.push(Position::new(cur_char, cur_line)),
                '.' => {}
                _ => unreachable!("Invalid input {}", c),
            };

            cur_char += 1;
        }

        cur_line += 1;
    }

    res
}

fn calc_atan2(lhs: &Position, rhs: &Position) -> f64 {
    (lhs.y as f64 - rhs.y as f64).atan2(lhs.x as f64 - rhs.x as f64)
}

fn detection_count(p: &Position, v: &Vec<Position>) -> i64 {
    let mut ms = v
        .iter()
        .filter(|it| it.x != p.x || it.y != p.y)
        .map(|it| calc_atan2(it, p))
        .collect::<Vec<f64>>();

    ms.sort_by(|l, r| l.partial_cmp(r).unwrap());
    ms.dedup();

    ms.len() as i64
}

fn get_best_position(v: &Vec<Position>) -> (&Position, i64) {
    v.iter()
        .map(|it| (it, detection_count(it, v)))
        .max_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs))
        .unwrap()
}

#[aoc(day10, part1)]
pub fn part1(v: &Vec<Position>) -> i64 {
    let (_, detection_count) = get_best_position(v);

    detection_count
}

#[aoc(day10, part2)]
pub fn part2(v: &Vec<Position>) -> i64 {
    let (p, _) = get_best_position(v);

    let dist = |lhs: &Position| -> f64 {
        ((lhs.x as f64 - p.x as f64).powf(2.0) + (lhs.y as f64 - p.y as f64).powf(2.0)).sqrt()
    };

    let sort_pos =
        |lhs: &Position, rhs: &Position| -> Option<Ordering> { dist(lhs).partial_cmp(&dist(rhs)) };

    let mut angles: Vec<(f64, Vec<&Position>)> = v
        .iter()
        .filter(|it| p.x != it.x || p.y != it.y)
        .fold(vec![], |mut acc, it| {
            let angle = calc_atan2(it, p) + PI / 2.0;
            let result = if angle < 0.0 { 2.0 * PI + angle } else { angle };

            if let Some(pos) = acc.iter().position(|(a, _)| *a == result) {
                let vec = &mut acc.get_mut(pos).unwrap().1;
                vec.push(it);
                vec.sort_by(|l, r| sort_pos(*l, *r).unwrap());
            } else {
                acc.push((result, vec![it]));
                acc.sort_by(|(l, _), (r, _)| l.partial_cmp(&r).unwrap());
            }

            acc
        });

    let mut idx = 0;
    loop {
        for (_, vec) in angles.iter_mut() {
            match vec.pop() {
                Some(elem) => {
                    idx += 1;
                    if idx == 200 {
                        return elem.x * 100 + elem.y;
                    }
                }
                _ => {}
            }
        }
    }
}
