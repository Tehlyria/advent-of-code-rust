const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const PIXEL_PER_LAYER: usize = WIDTH * HEIGHT;

#[aoc_generator(day8)]
pub fn generate(inp: &str) -> Vec<char> {
    inp.chars().collect()
}

fn count_char(chr: char, inp: &[char]) -> usize {
    inp.to_vec().iter().filter(|it| **it == chr).count()
}

#[aoc(day8, part1)]
pub fn part1(v: &Vec<char>) -> usize {
    let layers = v.chunks(PIXEL_PER_LAYER);

    let count_zeros = |inp| count_char('0', inp);
    let res = layers.min_by(|lhs, rhs| {
        let zl = count_zeros(lhs);
        let zr = count_zeros(rhs);
        zl.cmp(&zr)
    });

    let count_ones = |inp| count_char('1', inp);
    let count_twos = |inp| count_char('2', inp);

    match res {
        Some(r) => {
            let ones = count_ones(r);
            let twos = count_twos(r);

            return ones * twos;
        }
        None => panic!("No result found!"),
    }
}

#[aoc(day8, part2)]
pub fn part2(v: &Vec<char>) -> String {
    let layers = v.chunks(PIXEL_PER_LAYER).collect::<Vec<_>>();

    let mut pixels: Vec<Vec<_>> = Vec::new();

    for idx in 0..PIXEL_PER_LAYER {
        let mut v: Vec<char> = Vec::new();
        for l in &layers {
            v.push(l[idx]);
        }
        pixels.push(v);
    }

    let mut result = Vec::new();
    for elem in pixels {
        let res_pix = elem.iter().skip_while(|it| **it == '2').next().unwrap();
        result.push(*res_pix);
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
