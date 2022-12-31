use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use num::Integer;
use std::collections::HashSet;

const fn is_bug(c: char) -> bool {
    c == '#'
}

const fn is_space(c: char) -> bool {
    c == '.'
}

#[aoc_generator(day24)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().fold(vec![], |mut acc, it| {
        acc.push(it.chars().collect_vec());
        acc
    })
}

fn count_adjacent_bugs(row: usize, col: usize, grid: &[Vec<char>]) -> usize {
    let mut result = 0;

    if row > 0 {
        result += usize::from(is_bug(grid[row - 1][col]));
    }
    if row < grid.len() - 1 {
        result += usize::from(is_bug(grid[row + 1][col]));
    }

    if col > 0 {
        result += usize::from(is_bug(grid[row][col - 1]));
    }

    if col < grid[row].len() - 1 {
        result += usize::from(is_bug(grid[row][col + 1]));
    }

    result
}

fn calculate_biodiversity(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(idx, row)| {
            if idx.is_even() {
                row.clone()
            } else {
                row.iter().rev().copied().collect()
            }
        })
        .zip(std::iter::successors(Some(1), |it| Some(it * 2)))
        .fold(0, |acc, (l, r)| acc + if is_bug(l) { r } else { 0 })
}

#[aoc(day24, part1)]
pub fn part1(grid: &[Vec<char>]) -> usize {
    let mut current = grid.to_vec();
    let mut next = vec![vec!['.'; current[0].len()]; current.len()];

    let mut seen = HashSet::new();
    seen.insert(current.clone());

    loop {
        for (row, col) in iproduct!(0..current.len(), 0..current[0].len()) {
            let num_bugs = count_adjacent_bugs(row, col, &current);
            if (num_bugs == 1 || num_bugs == 2) && is_space(current[row][col]) {
                next[row][col] = '#';
            } else if num_bugs != 1 && is_bug(current[row][col]) {
                next[row][col] = '.';
            } else {
                next[row][col] = current[row][col];
            }
        }

        current = next.clone();

        if !seen.insert(current.clone()) {
            break;
        }
    }

    calculate_biodiversity(&current)
}
