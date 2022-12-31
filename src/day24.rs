use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use num::Integer;
use pathfinding::matrix::Matrix;
use std::collections::HashSet;

const fn is_bug(c: char) -> bool {
    c == '#'
}

const fn is_space(c: char) -> bool {
    c == '.'
}

#[aoc_generator(day24)]
pub fn generate(inp: &str) -> Option<Matrix<char>> {
    let rows = inp.lines().map(|it| it.chars().collect_vec()).collect_vec();
    Matrix::from_rows(rows).ok()
}

fn count_adjacent_bugs(row: usize, col: usize, grid: &Matrix<char>) -> usize {
    grid.neighbours((row, col), false)
        .filter(|&(r, c)| is_bug(grid[(r, c)]))
        .count()
}

fn calculate_biodiversity(grid: &Matrix<char>) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(idx, row)| {
            if idx.is_even() {
                row.to_vec()
            } else {
                row.iter().rev().copied().collect_vec()
            }
        })
        .zip(std::iter::successors(Some(1), |it| Some(it * 2)))
        .fold(0, |acc, (l, r)| acc + if is_bug(l) { r } else { 0 })
}

#[aoc(day24, part1)]
pub fn part1(grid: &Matrix<char>) -> usize {
    let mut current = grid.clone();
    let mut next = Matrix::new(current.rows, current.columns, '.');

    let mut seen = HashSet::new();
    seen.insert(current.clone());

    loop {
        for (row, col) in iproduct!(0..current.rows, 0..current.columns) {
            let num_bugs = count_adjacent_bugs(row, col, &current);
            if (num_bugs == 1 || num_bugs == 2) && is_space(current[(row, col)]) {
                next[(row, col)] = '#';
            } else if num_bugs != 1 && is_bug(current[(row, col)]) {
                next[(row, col)] = '.';
            } else {
                next[(row, col)] = current[(row, col)];
            }
        }

        current = next.clone();

        if !seen.insert(current.clone()) {
            break;
        }
    }

    calculate_biodiversity(&current)
}
