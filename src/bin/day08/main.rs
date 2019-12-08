use std::fs::File;
use std::io::{BufReader, Read};

fn read_input() -> std::io::Result<String> {
    let file = File::open("input.txt")?;
    let mut file_reader = BufReader::new(file);
    let mut result = String::new();

    match file_reader.read_to_string(&mut result) {
        Ok(_) => Ok(result),
        Err(e) => Err(e),
    }
}

fn part_one(inp: &String) {
    let width = 25;
    let height = 6;

    let pixel_per_layer = width * height;

    let as_vec = inp.chars().collect::<Vec<_>>();
    let layers = as_vec.chunks(pixel_per_layer);

    let res = layers.min_by(|lhs, rhs| {
        lhs.to_vec()
            .iter()
            .filter(|l| **l == '0')
            .count()
            .cmp(&rhs.to_vec().iter().filter(|l| **l == '0').count())
    });

    match res {
        Some(r) => {
            let num_one = r.to_vec().iter().filter(|it| **it == '1').count();
            let num_two = r.to_vec().iter().filter(|it| **it == '2').count();

            println!("Part One {}", num_one * num_two);
        }
        None => eprintln!("No result found!"),
    }
}

fn part_two(inp: &String) {
    let width = 25;
    let height = 6;

    let pixel_per_layer = width * height;

    let as_vec = inp.chars().collect::<Vec<_>>();
    let layers = as_vec.chunks(pixel_per_layer).collect::<Vec<_>>();

    let mut pixels: Vec<Vec<_>> = Vec::new();

    for idx in 0..pixel_per_layer {
        let mut v = Vec::new();
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

    println!("Part Two");
    for y in 0..height {
        for x in 0..width {
            let chr = result[x + y * width];
            print!("{}", if chr == '0' { ' ' } else { '#' });
        }

        println!();
    }
}

fn main() {
    let file_content = read_input();

    match file_content {
        Ok(content) => {
            part_one(&content);
            part_two(&content);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
