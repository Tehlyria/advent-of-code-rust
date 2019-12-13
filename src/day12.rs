use num::integer::lcm;
use regex::Regex;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct Vec3d(i64, i64, i64);

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct Planet {
    pos: Vec3d,
    vel: Vec3d,
}

impl Planet {
    pub fn new(pos: Vec3d, vel: Vec3d) -> Self {
        Self { pos, vel }
    }

    pub fn potential_energy(&self) -> i64 {
        i64::abs(self.pos.0) + i64::abs(self.pos.1) + i64::abs(self.pos.2)
    }

    pub fn kinetic_energy(&self) -> i64 {
        i64::abs(self.vel.0) + i64::abs(self.vel.1) + i64::abs(self.vel.2)
    }
}

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Vec<Planet> {
    let re = Regex::new(r"<x=(-?[\d]+), y=(-?[\d]+), z=(-?[\d]+)>").unwrap();

    inp.lines()
        .map(|it| {
            assert!(re.is_match(it));

            let cap = re.captures(it).unwrap();
            let x = cap[1].parse::<i64>().unwrap();
            let y = cap[2].parse::<i64>().unwrap();
            let z = cap[3].parse::<i64>().unwrap();

            Planet::new(Vec3d(x, y, z), Vec3d(0, 0, 0))
        })
        .collect()
}

fn apply_gravity(v: &mut Vec<Planet>) {
    let tmp = v.clone();

    for it in v.iter_mut() {
        for ot in tmp.iter() {
            if it == ot {
                continue;
            }

            if ot.pos.0 > it.pos.0 {
                it.vel.0 += 1;
            } else if ot.pos.0 < it.pos.0 {
                it.vel.0 -= 1;
            }

            if ot.pos.1 > it.pos.1 {
                it.vel.1 += 1;
            } else if ot.pos.1 < it.pos.1 {
                it.vel.1 -= 1;
            }

            if ot.pos.2 > it.pos.2 {
                it.vel.2 += 1;
            } else if ot.pos.2 < it.pos.2 {
                it.vel.2 -= 1;
            }
        }
    }
}

fn apply_velocity(v: &mut Vec<Planet>) {
    v.iter_mut().for_each(|it| {
        it.pos.0 += it.vel.0;
        it.pos.1 += it.vel.1;
        it.pos.2 += it.vel.2;
    });
}

fn total_energy(planets: &Vec<Planet>) -> i64 {
    planets
        .iter()
        .map(|it| it.kinetic_energy() * it.potential_energy())
        .sum::<i64>()
}

#[aoc(day12, part1)]
pub fn part1(v: &Vec<Planet>) -> i64 {
    let mut planets = v.clone();

    for _ in 0..1000 {
        apply_gravity(&mut planets);
        apply_velocity(&mut planets);
    }

    total_energy(&planets)
}

#[aoc(day12, part2)]
pub fn part2(v: &Vec<Planet>) -> i64 {
    let mut planets = v.clone();

    let mut steps = 0;

    let mut cycle_x = -1;
    let mut cycle_y = -1;
    let mut cycle_z = -1;

    loop {
        apply_gravity(&mut planets);
        apply_velocity(&mut planets);

        steps += 1;

        if cycle_x == -1 && planets.iter().all(|it| it.vel.0 == 0) {
            cycle_x = steps;
        }
        if cycle_y == -1 && planets.iter().all(|it| it.vel.1 == 0) {
            cycle_y = steps;
        }
        if cycle_z == -1 && planets.iter().all(|it| it.vel.2 == 0) {
            cycle_z = steps;
        }

        if cycle_x != -1 && cycle_y != -1 && cycle_z != -1 {
            return 2 * lcm(cycle_x, lcm(cycle_y, cycle_z));
        }
    }
}
