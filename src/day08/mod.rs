use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const PIXEL_PER_LAYER: usize = WIDTH * HEIGHT;

#[aoc_generator(day8)]
pub fn generate(inp: &str) -> Vec<char> {
    inp.chars().collect()
}

fn count_char(chr: char, inp: &[char]) -> usize {
    inp.iter().filter(|it| **it == chr).count()
}

#[aoc(day8, part1)]
pub fn part1(v: &[char]) -> Option<usize> {
    let layers = v.chunks(PIXEL_PER_LAYER);

    let min = layers.min_by(|lhs, rhs| {
        let zl = count_char('0', lhs);
        let zr = count_char('0', rhs);
        zl.cmp(&zr)
    });

    match min {
        Some(res) => {
            let ones = count_char('1', res);
            let twos = count_char('2', res);

            Some(ones * twos)
        }
        None => None,
    }
}

#[aoc(day8, part2)]
pub fn part2(v: &[char]) -> String {
    let layers = v.chunks(PIXEL_PER_LAYER).collect_vec();

    let pixels = (0..PIXEL_PER_LAYER)
        .map(|idx| layers.iter().map(|it| it[idx]).collect_vec())
        .collect_vec();

    let mut result = Vec::new();
    for elem in pixels {
        if let Some(res_pix) = elem.iter().find(|it| **it != '2') {
            result.push(*res_pix);
        }
    }

    let mut res = String::from("\n");

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let chr = result[x + y * WIDTH];
            res.push(if chr == '0' { ' ' } else { '#' });
        }
        res.push('\n');
    }

    res
}
