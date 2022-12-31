use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;
use parse_display::FromStr as PFromStr;
use std::ops::AddAssign;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, PFromStr)]
#[display("<x={0}, y={1}, z={2}>")]
pub struct Vec3d(i64, i64, i64);

impl AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Planet {
    pos: Vec3d,
    vel: Vec3d,
}

impl Planet {
    pub const fn new(pos: Vec3d, vel: Vec3d) -> Self {
        Self { pos, vel }
    }

    pub const fn potential_energy(&self) -> i64 {
        i64::abs(self.pos.0) + i64::abs(self.pos.1) + i64::abs(self.pos.2)
    }

    pub const fn kinetic_energy(&self) -> i64 {
        i64::abs(self.vel.0) + i64::abs(self.vel.1) + i64::abs(self.vel.2)
    }
}

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Vec<Planet> {
    inp.lines()
        .filter_map(|it| it.parse::<Vec3d>().ok())
        .map(|it| Planet::new(it, Vec3d(0, 0, 0)))
        .collect()
}

fn apply_gravity(v: &mut Vec<Planet>) {
    let tmp = v.clone();

    for it in v {
        for ot in &tmp {
            if it == ot {
                continue;
            }

            it.vel.0 += match ot.pos.0.cmp(&it.pos.0) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };

            it.vel.1 += match ot.pos.1.cmp(&it.pos.1) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };

            it.vel.2 += match ot.pos.2.cmp(&it.pos.2) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            }
        }
    }
}

fn apply_velocity(v: &mut [Planet]) {
    for it in v {
        it.pos += it.vel;
    }
}

fn total_energy(planets: &[Planet]) -> i64 {
    planets.iter().fold(0, |acc, it| {
        acc + it.kinetic_energy() * it.potential_energy()
    })
}

#[aoc(day12, part1)]
pub fn part1(v: &[Planet]) -> i64 {
    let mut planets = v.to_vec();

    for _ in 0..1000 {
        apply_gravity(&mut planets);
        apply_velocity(&mut planets);
    }

    total_energy(&planets)
}

#[aoc(day12, part2)]
pub fn part2(v: &[Planet]) -> i64 {
    let mut planets = v.to_vec();

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
