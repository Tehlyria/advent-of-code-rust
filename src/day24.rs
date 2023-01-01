use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
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

fn count_adjacent_bugs(idx: (usize, usize), grid: &Matrix<char>) -> usize {
    grid.neighbours(idx, false)
        .filter(|&pos| is_bug(grid[pos]))
        .count()
}

fn count_adjacent_bugs_plutonian(
    idx: (usize, usize),
    depth: usize,
    grids: &[Matrix<char>],
) -> usize {
    let current = &grids[depth];

    let mut result = current
        .neighbours(idx, false)
        .filter(|&it| it != (2, 2))
        .map(|it| current[it])
        .collect_vec();

    let tile_number = current.idx(idx);

    if depth < grids.len() - 1 {
        let inner = &grids[depth + 1];

        if tile_number == 7 {
            result.push(inner[(0, 0)]);
            result.push(inner[(0, 1)]);
            result.push(inner[(0, 2)]);
            result.push(inner[(0, 3)]);
            result.push(inner[(0, 4)]);
        } else if tile_number == 11 {
            result.push(inner[(0, 0)]);
            result.push(inner[(1, 0)]);
            result.push(inner[(2, 0)]);
            result.push(inner[(3, 0)]);
            result.push(inner[(4, 0)]);
        } else if tile_number == 13 {
            result.push(inner[(0, 4)]);
            result.push(inner[(1, 4)]);
            result.push(inner[(2, 4)]);
            result.push(inner[(3, 4)]);
            result.push(inner[(4, 4)]);
        } else if tile_number == 17 {
            result.push(inner[(4, 0)]);
            result.push(inner[(4, 1)]);
            result.push(inner[(4, 2)]);
            result.push(inner[(4, 3)]);
            result.push(inner[(4, 4)]);
        }
    }

    if depth > 0 {
        let outer = &grids[depth - 1];

        if tile_number == 0
            || tile_number == 1
            || tile_number == 2
            || tile_number == 3
            || tile_number == 4
        {
            result.push(outer[(1, 2)]);
        }

        if tile_number == 0
            || tile_number == 5
            || tile_number == 10
            || tile_number == 15
            || tile_number == 20
        {
            result.push(outer[(2, 1)]);
        }

        if tile_number == 4
            || tile_number == 9
            || tile_number == 14
            || tile_number == 19
            || tile_number == 24
        {
            result.push(outer[(2, 3)]);
        }

        if tile_number == 20
            || tile_number == 21
            || tile_number == 22
            || tile_number == 23
            || tile_number == 24
        {
            result.push(outer[(3, 2)]);
        }
    }

    result.iter().filter(|&&it| is_bug(it)).count()
}

fn calculate_biodiversity(grid: &Matrix<char>) -> usize {
    grid.items()
        .map(|(_, it)| *it)
        .zip(std::iter::successors(Some(1), |it| Some(it * 2)))
        .filter_map(|(c, score)| if is_bug(c) { Some(score) } else { None })
        .sum()
}

fn next_state(num_bugs: usize, idx: (usize, usize), current: &Matrix<char>) -> char {
    if (num_bugs == 1 || num_bugs == 2) && is_space(current[idx]) {
        // An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
        '#'
    } else if num_bugs != 1 && is_bug(current[idx]) {
        // A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
        '.'
    } else {
        current[idx]
    }
}

#[aoc(day24, part1)]
pub fn part1(grid: &Matrix<char>) -> usize {
    let mut current = grid.clone();
    let mut next = Matrix::new(current.rows, current.columns, '.');

    let mut seen = HashSet::new();
    seen.insert(current.clone());

    loop {
        for idx in iproduct!(0..current.rows, 0..current.columns) {
            let num_bugs = count_adjacent_bugs(idx, &current);
            next[idx] = next_state(num_bugs, idx, &current);
        }

        current = next.clone();

        if !seen.insert(current.clone()) {
            break;
        }
    }

    calculate_biodiversity(&current)
}

fn simulate_plutonian(duration: usize, grid: &Matrix<char>) -> usize {
    let mut grids = vec![Matrix::new(5, 5, '.'); 256];
    grids[128] = grid.clone();

    for _ in 0..duration {
        let mut next_grids = vec![Matrix::new(5, 5, '.'); grids.len()];

        for depth in 0..grids.len() {
            let mut next = Matrix::new(5, 5, '.');

            for idx in iproduct!(0..5, 0..5) {
                if idx == (2, 2) {
                    continue;
                }

                let num_bugs = count_adjacent_bugs_plutonian(idx, depth, &grids);
                next[idx] = next_state(num_bugs, idx, &grids[depth]);
            }

            next_grids[depth] = next;
        }

        grids = next_grids;
    }

    grids
        .iter()
        .flatten()
        .map(|it| it.iter().filter(|&&c| is_bug(c)).count())
        .sum()
}

#[aoc(day24, part2)]
pub fn part2(grid: &Matrix<char>) -> usize {
    simulate_plutonian(200, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "....#\n\
                             #..#.\n\
                             #..##\n\
                             ..#..\n\
                             #....";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_DATA).expect("Failed to parse test data");
        let res = part1(&data);
        assert_eq!(res, 2_129_920);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_DATA).expect("Failed to parse test data");
        let res = simulate_plutonian(10, &data);
        assert_eq!(res, 99);
    }
}
