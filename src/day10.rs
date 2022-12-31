use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::f64::consts::PI;

#[derive(PartialEq, Eq, Hash)]
pub struct Position(i64, i64);

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> Vec<Position> {
    inp.lines()
        .enumerate()
        .fold(Vec::new(), |acc, (cur_line, l)| {
            l.chars().enumerate().fold(acc, |mut acc, (cur_char, c)| {
                match c {
                    '#' => acc.push(Position(cur_char as i64, cur_line as i64)),
                    '.' => {}
                    _ => unreachable!("Invalid input {}", c),
                }

                acc
            })
        })
}

#[allow(clippy::cast_precision_loss)]
fn calc_atan2(lhs: &Position, rhs: &Position) -> f64 {
    (lhs.1 as f64 - rhs.1 as f64).atan2(lhs.0 as f64 - rhs.0 as f64)
}

fn detection_count(p: &Position, v: &[Position]) -> i64 {
    let mut ms = v
        .iter()
        .filter(|it| (**it).ne(p))
        .map(|it| calc_atan2(it, p))
        .collect::<Vec<f64>>();

    ms.sort_by(|l, r| l.partial_cmp(r).unwrap_or(Ordering::Less));
    ms.dedup();

    ms.len() as i64
}

fn get_best_position(v: &[Position]) -> Option<(&Position, i64)> {
    v.iter()
        .map(|it| (it, detection_count(it, v)))
        .max_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs))
}

#[aoc(day10, part1)]
pub fn part1(v: &[Position]) -> Option<i64> {
    let (_, detection_count) = get_best_position(v)?;

    Some(detection_count)
}

#[aoc(day10, part2)]
pub fn part2(v: &[Position]) -> Option<i64> {
    let (p, _) = get_best_position(v)?;

    #[allow(clippy::cast_precision_loss)]
    let dist =
        |lhs: &Position| -> f64 { (lhs.0 as f64 - p.0 as f64).hypot(lhs.1 as f64 - p.1 as f64) };

    let sort_pos =
        |lhs: &Position, rhs: &Position| -> Option<Ordering> { dist(lhs).partial_cmp(&dist(rhs)) };

    let mut angles: Vec<(f64, Vec<&Position>)> =
        v.iter()
            .filter(|it| (**it).ne(p))
            .fold(vec![], |mut acc, it| {
                let angle = calc_atan2(it, p) + PI / 2.0;
                let result = if angle < 0.0 {
                    2.0f64.mul_add(PI, angle)
                } else {
                    angle
                };

                if let Some(pos) = acc.iter().position(|(a, _)| (*a - result).abs() < 0.001) {
                    if let Some((_, vec)) = &mut acc.get_mut(pos) {
                        vec.push(it);
                        vec.sort_by(|l, r| sort_pos(l, r).unwrap_or(Ordering::Less));
                    }
                } else {
                    acc.push((result, vec![it]));
                    acc.sort_by(|(l, _), (r, _)| l.partial_cmp(r).unwrap_or(Ordering::Less));
                }

                acc
            });

    let mut idx = 0;
    loop {
        for (_, vec) in &mut angles {
            if let Some(elem) = vec.pop() {
                idx += 1;
                if idx == 200 {
                    return Some(elem.0 * 100 + elem.1);
                }
            }
        }
    }
}
