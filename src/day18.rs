use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::matrix::Matrix;
use std::collections::HashMap;

type PositionAndKeys = (usize, usize, Vec<char>);
type KeyAndCost = Vec<(usize, usize, usize)>;

const fn is_wall(c: char) -> bool {
    c == '#'
}

const fn is_start(c: char) -> bool {
    c == '@'
}

const fn is_open_tile(c: char) -> bool {
    c == '.' || is_start(c)
}

const fn is_key(c: char) -> bool {
    c.is_ascii_lowercase()
}

fn is_door_and_have_key(c: char, keys: &[char]) -> bool {
    c.is_ascii_uppercase() && keys.contains(&c.to_ascii_lowercase())
}

#[aoc_generator(day18)]
pub fn generate(inp: &str) -> Option<Matrix<char>> {
    let rows = inp.lines().map(|it| it.chars().collect_vec()).collect_vec();
    Matrix::from_rows(rows).ok()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    idx: usize,
    keys: Vec<char>,
    row: usize,
    col: usize,
    cost: usize,
}

fn find_start_position(grid: &Matrix<char>) -> (usize, usize) {
    grid.items()
        .find_map(|((r, c), it)| if is_start(*it) { Some((r, c)) } else { None })
        .expect("Starting position has to exist")
}

fn count_available_keys(grid: &Matrix<char>) -> usize {
    grid.items()
        .filter(|(_, it)| it.is_ascii_lowercase() || it.is_ascii_uppercase())
        .map(|(_, it)| it.to_ascii_lowercase())
        .unique()
        .count()
}

#[aoc(day18, part1)]
pub fn part1(grid: &Matrix<char>) -> Option<usize> {
    let (s_row, s_col) = find_start_position(grid);

    let num_keys = count_available_keys(grid);

    let start_state = State {
        idx: 0,
        keys: Vec::new(),
        row: s_row,
        col: s_col,
        cost: 0,
    };

    let mut keys_cache: HashMap<PositionAndKeys, KeyAndCost> = HashMap::new();
    let mut cache: HashMap<PositionAndKeys, State> = HashMap::new();

    let (_, cost) = pathfinding::prelude::dijkstra(
        &start_state,
        |it| {
            let mut result = vec![];

            for (r, c, cost) in
                find_reachable_keys((it.row, it.col), &it.keys, grid, &mut keys_cache)
            {
                let mut new_keys = it.keys.clone();
                new_keys.push(grid[(r, c)]);

                let cache_key = (r, c, new_keys.iter().sorted().copied().collect_vec());

                let new_state = State {
                    idx: it.idx,
                    keys: new_keys,
                    row: r,
                    col: c,
                    cost: it.cost + cost,
                };

                if let Some(cached) = cache.get(&cache_key) {
                    if cached.cost <= new_state.cost {
                        return vec![];
                    }
                }

                cache.insert(cache_key, new_state.clone());

                result.push((new_state, cost));
            }

            result
        },
        |it| it.keys.len() == num_keys,
    )?;

    Some(cost)
}

fn update_map(grid: &mut Matrix<char>) -> Vec<(usize, usize)> {
    let (row, col) = find_start_position(grid);

    // ...     @#@
    // .@.  -> ###
    // ...     @#@
    grid[(row, col)] = '#';
    grid[(row - 1, col)] = '#';
    grid[(row + 1, col)] = '#';
    grid[(row, col - 1)] = '#';
    grid[(row, col + 1)] = '#';
    grid[(row - 1, col - 1)] = '@';
    grid[(row - 1, col + 1)] = '@';
    grid[(row + 1, col - 1)] = '@';
    grid[(row + 1, col + 1)] = '@';

    vec![
        (row - 1, col - 1),
        (row - 1, col + 1),
        (row + 1, col - 1),
        (row + 1, col + 1),
    ]
}

#[aoc(day18, part2)]
pub fn part2(grid: &Matrix<char>) -> Option<usize> {
    let mut grid = grid.clone();
    let start_positions = update_map(&mut grid);
    assert_eq!(start_positions.len(), 4);

    let num_keys = count_available_keys(&grid);

    let start_state = State {
        idx: 0,
        keys: Vec::new(),
        row: start_positions[0].0,
        col: start_positions[0].1,
        cost: 0,
    };

    // Use single pathfinding, but if a state has no reachable keys anymore,
    // skip to the next saved state, pretending there exists an edge between the
    // different quadrants of the map with no cost
    let starting_states = vec![
        start_state.clone(),
        State {
            idx: 1,
            keys: Vec::new(),
            row: start_positions[1].0,
            col: start_positions[1].1,
            cost: 0,
        },
        State {
            idx: 2,
            keys: Vec::new(),
            row: start_positions[2].0,
            col: start_positions[2].1,
            cost: 0,
        },
        State {
            idx: 3,
            keys: Vec::new(),
            row: start_positions[3].0,
            col: start_positions[3].1,
            cost: 0,
        },
    ];

    let mut keys_cache: HashMap<PositionAndKeys, KeyAndCost> = HashMap::new();

    let (_, cost) = pathfinding::prelude::dijkstra(
        &(start_state, starting_states),
        |(it, saved_states)| {
            let mut result = vec![];

            let mut saved_states = saved_states.clone();

            let reachable_keys =
                find_reachable_keys((it.row, it.col), &it.keys, &grid, &mut keys_cache);
            if reachable_keys.is_empty() {
                let next_idx = (it.idx + 1) % 4;
                saved_states[it.idx] = it.clone();

                let mut next_state = saved_states[next_idx].clone();
                next_state.keys.clone_from(&it.keys);
                next_state.cost = it.cost;

                result.push(((next_state, saved_states.clone()), 0));

                return result;
            }

            for (r, c, cost) in reachable_keys {
                let mut new_keys = it.keys.clone();
                new_keys.push(grid[(r, c)]);

                let new_state = State {
                    idx: it.idx,
                    keys: new_keys,
                    row: r,
                    col: c,
                    cost: it.cost + cost,
                };

                saved_states[it.idx] = new_state.clone();

                result.push(((new_state, saved_states.clone()), cost));
            }

            result
        },
        |(it, _)| it.keys.len() == num_keys,
    )?;

    Some(cost)
}

fn find_reachable_keys(
    (row, col): (usize, usize),
    keys: &[char],
    grid: &Matrix<char>,
    keys_cache: &mut HashMap<PositionAndKeys, KeyAndCost>,
) -> Vec<(usize, usize, usize)> {
    let cache_key = (row, col, keys.iter().sorted().copied().collect_vec());
    if let Some(cached) = keys_cache.get(&cache_key) {
        return cached.clone();
    }

    let nodes = pathfinding::prelude::dijkstra_all(&(row, col), |&(r, c)| {
        grid.neighbours((r, c), false)
            .filter_map(|pos| {
                let chr = grid[pos];
                if !is_wall(chr)
                    && (is_open_tile(chr) || is_key(chr) || is_door_and_have_key(chr, keys))
                {
                    Some((pos, 1))
                } else {
                    None
                }
            })
            .collect_vec()
    });

    let res = nodes
        .iter()
        .filter_map(|((r, c), (_, cost))| {
            let chr = grid[(*r, *c)];
            if chr.is_ascii_lowercase() && !keys.contains(&chr) {
                Some((*r, *c, *cost))
            } else {
                None
            }
        })
        .collect_vec();

    keys_cache.insert(cache_key, res.clone());

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1_1() {
        let inp = "#########\n\
                          #b.A.@.a#\n\
                          #########";

        let data = generate(inp).expect("");
        let res = part1(&data);
        assert_eq!(res, Some(8));
    }

    #[test]
    fn test_sample_p1_2() {
        let inp = "########################\n\
                          #f.D.E.e.C.b.A.@.a.B.c.#\n\
                          ######################.#\n\
                          #d.....................#\n\
                          ########################";

        let data = generate(inp).expect("");
        let res = part1(&data);
        assert_eq!(res, Some(86));
    }

    #[test]
    fn test_sample_p1_3() {
        let inp = "########################\n\
                          #...............b.C.D.f#\n\
                          #.######################\n\
                          #.....@.a.B.c.d.A.e.F.g#\n\
                          ########################";

        let data = generate(inp).expect("");
        let res = part1(&data);
        assert_eq!(res, Some(132));
    }

    #[test]
    fn test_sample_p1_4() {
        let inp = "#################\n\
                          #i.G..c...e..H.p#\n\
                          ########.########\n\
                          #j.A..b...f..D.o#\n\
                          ########@########\n\
                          #k.E..a...g..B.n#\n\
                          ########.########\n\
                          #l.F..d...h..C.m#\n\
                          #################";

        let data = generate(inp).expect("");
        let res = part1(&data);
        assert_eq!(res, Some(136));
    }

    #[test]
    fn test_sample_p1_5() {
        let inp = "########################\n\
                          #@..............ac.GI.b#\n\
                          ###d#e#f################\n\
                          ###A#B#C################\n\
                          ###g#h#i################\n\
                          ########################";

        let data = generate(inp).expect("");
        let res = part1(&data);
        assert_eq!(res, Some(81));
    }

    #[test]
    fn test_sample_p2_1() {
        let inp = "#######\n\
                          #a.#Cd#\n\
                          ##...##\n\
                          ##.@.##\n\
                          ##...##\n\
                          #cB#Ab#\n\
                          #######";

        let data = generate(inp).expect("");
        let res = part2(&data);
        assert_eq!(res, Some(8));
    }

    #[test]
    fn test_sample_p2_2() {
        let inp = "###############\n\
                          #d.ABC.#.....a#\n\
                          ######...######\n\
                          ######.@.######\n\
                          ######...######\n\
                          #b.....#.....c#\n\
                          ###############";

        let data = generate(inp).expect("");
        let res = part2(&data);
        assert_eq!(res, Some(24));
    }

    #[test]
    fn test_sample_p2_3() {
        let inp = "#############\n\
                          #DcBa.#.GhKl#\n\
                          #.###...#I###\n\
                          #e#d#.@.#j#k#\n\
                          ###C#...###J#\n\
                          #fEbA.#.FgHi#\n\
                          #############";

        let data = generate(inp).expect("");
        let res = part2(&data);
        assert_eq!(res, Some(32));
    }

    #[test]
    fn test_sample_p2_4() {
        let inp = "#############\n\
                          #g#f.D#..h#l#\n\
                          #F###e#E###.#\n\
                          #dCba...BcIJ#\n\
                          #####.@.#####\n\
                          #nK.L...G...#\n\
                          #M###N#H###.#\n\
                          #o#m..#i#jk.#\n\
                          #############";

        let data = generate(inp).expect("");
        let res = part2(&data);
        assert_eq!(res, Some(72));
    }
}
