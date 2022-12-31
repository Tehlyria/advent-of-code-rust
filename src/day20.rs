use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[display("{row},{col}")]
pub struct Position {
    row: usize,
    col: usize,
}

#[derive(Display, FromStr, Clone, Debug)]
#[display("{name}: {a}|{b}")]
pub struct Portal {
    name: String,
    a: Position,
    b: Position,
}

#[derive(Clone, Debug)]
pub struct MazeData {
    start_pos: Position,
    goal: Position,
    portals: Vec<Portal>,
    maze: Vec<Vec<char>>,
}

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> Option<MazeData> {
    let mut lines = inp.lines();
    let start_pos = lines.next()?.parse::<Position>().ok()?;
    let goal = lines.next()?.parse::<Position>().ok()?;

    // Skip empty
    lines.next()?;

    let mut portals = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let portal = line.parse::<Portal>().ok()?;
        portals.push(portal);
    }

    let maze = lines.fold(vec![], |mut acc, line| {
        let row = line.chars().collect_vec();
        acc.push(row);
        acc
    });

    // sanity check
    for p in &portals {
        assert_eq!(maze[p.a.row][p.a.col], '.');
        assert_eq!(maze[p.b.row][p.b.col], '.');
    }

    Some(MazeData {
        start_pos,
        goal,
        portals,
        maze,
    })
}

impl RecursionState {
    pub const fn new(row: usize, col: usize, depth: usize) -> Self {
        Self {
            position: Position { row, col },
            depth,
        }
    }
}

const fn is_outside_portal(pos: Position, height: usize, width: usize) -> bool {
    pos.col == 0 || pos.col == width - 1 || pos.row == 0 || pos.row == height - 1
}

fn successors(
    state: &RecursionState,
    md: &MazeData,
    ignore_depth: bool,
) -> Vec<(RecursionState, usize)> {
    let MazeData { maze, portals, .. } = md;
    let cur = state.position;

    let mut result = vec![];

    // Up
    if cur.row > 0 && maze[cur.row - 1][cur.col] == '.' {
        result.push((RecursionState::new(cur.row - 1, cur.col, state.depth), 1));
    }

    // Down
    if cur.row < maze.len() - 1 && maze[cur.row + 1][cur.col] == '.' {
        result.push((RecursionState::new(cur.row + 1, cur.col, state.depth), 1));
    }

    // Left
    if cur.col > 0 && maze[cur.row][cur.col - 1] == '.' {
        result.push((RecursionState::new(cur.row, cur.col - 1, state.depth), 1));
    }

    // Right
    if cur.col < maze[cur.row].len() - 1 && maze[cur.row][cur.col + 1] == '.' {
        result.push((RecursionState::new(cur.row, cur.col + 1, state.depth), 1));
    }

    // Handle portals
    for (next_state, cost) in &mut result {
        if let Some(portal) = portals
            .iter()
            .find(|p| p.a == next_state.position || p.b == next_state.position)
        {
            let is_outside_portal =
                is_outside_portal(next_state.position, maze.len(), maze[cur.row].len());

            if !ignore_depth && is_outside_portal && next_state.depth == 0 {
                continue;
            }

            if !ignore_depth && is_outside_portal {
                assert_ne!(next_state.depth, 0);
                next_state.depth -= 1;
            } else {
                next_state.depth += 1;
            }

            next_state.position = if portal.a == next_state.position {
                portal.b
            } else {
                portal.a
            };

            *cost += 1;
        }
    }

    result
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct RecursionState {
    position: Position,
    depth: usize,
}

#[aoc(day20, part1)]
pub fn part1(md: &MazeData) -> Option<usize> {
    let start_state = RecursionState::new(md.start_pos.row, md.start_pos.col, 0);
    let (_, cost) = pathfinding::prelude::dijkstra(
        &start_state,
        |it| successors(it, md, true),
        |it| it.position == md.goal,
    )?;

    Some(cost)
}

#[aoc(day20, part2)]
pub fn part2(md: &MazeData) -> Option<usize> {
    let start_state = RecursionState::new(md.start_pos.row, md.start_pos.col, 0);
    let (_, cost) = pathfinding::prelude::dijkstra(
        &start_state,
        |it| successors(it, md, false),
        |it| it.depth == 0 && it.position == md.goal,
    )?;

    Some(cost)
}
