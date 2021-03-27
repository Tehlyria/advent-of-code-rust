use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;
use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr, Clone, Debug)]
#[display("{0} {1}")]
struct Ingredient(usize, String);

#[derive(Debug)]
pub struct Recipe {
    inputs: Vec<Ingredient>,
    output: Ingredient,
}

fn parse_single_ingredient(ingredient: &str) -> Option<Ingredient> {
    ingredient.trim().parse::<Ingredient>().ok()
}

fn parse_ingredients(ingredients: &str) -> Vec<Ingredient> {
    ingredients
        .trim()
        .split(',')
        .filter_map(parse_single_ingredient)
        .collect()
}

#[aoc_generator(day14)]
pub fn generate(inp: &str) -> Vec<Recipe> {
    inp.lines()
        .filter_map(|it| {
            let mut spl = it.split("=>");
            let inputs = spl.next().map(parse_ingredients)?;
            let output = spl.next().map(parse_single_ingredient).unwrap_or(None)?;

            Some(Recipe { inputs, output })
        })
        .collect()
}

fn find_recipe<'a>(name: &str, all: &'a [Recipe]) -> Option<&'a Recipe> {
    all.iter().find(|it| it.output.1.eq(&name))
}

fn count_ore_impl(
    inp: &str,
    amount: usize,
    recipes: &[Recipe],
    mats: &mut HashMap<String, usize>,
) -> usize {
    if inp.eq("ORE") {
        return amount;
    }

    let quant = mats.entry(inp.to_string()).or_insert(0);

    let need = if *quant > 0 {
        let available_quant = *quant;
        *quant = if available_quant >= amount {
            available_quant - amount
        } else {
            0
        };
        if amount >= available_quant {
            amount - available_quant
        } else {
            0
        }
    } else {
        amount
    };

    if need == 0 {
        return 0;
    }

    let recipe = find_recipe(inp, recipes).unwrap();
    let need_steps = need.div_ceil(&recipe.output.0);
    let produces = recipe.output.0 * need_steps;
    if need < produces {
        let leftover = produces - need;
        *mats.entry(inp.to_string()).or_insert(0) += leftover;
    }

    recipe
        .inputs
        .iter()
        .map(|it| count_ore_impl(&it.1, it.0 * need_steps, recipes, mats))
        .sum()
}

fn count_ore(inp: &str, amount: usize, recipes: &[Recipe]) -> usize {
    count_ore_impl(inp, amount, recipes, &mut HashMap::new())
}

#[aoc(day14, part1)]
pub fn part1(inp: &[Recipe]) -> usize {
    count_ore("FUEL", 1, inp)
}

trait BinSearchable {
    fn binary_search_by<F>(&mut self, f: F) -> usize
    where
        F: FnMut(usize) -> Ordering;
}

impl BinSearchable for RangeInclusive<usize> {
    fn binary_search_by<F>(&mut self, mut f: F) -> usize
    where
        F: FnMut(usize) -> Ordering,
    {
        let mut low = *self.start();
        let mut high = *self.end();

        while low <= high {
            let mid = (low + high) / 2;
            match f(mid) {
                Ordering::Greater => high = mid - 1,
                Ordering::Equal => return mid,
                Ordering::Less => low = mid + 1,
            }
        }

        low - 1
    }
}

#[aoc(day14, part2)]
pub fn part2(inp: &[Recipe]) -> usize {
    const TRILLION: usize = 1_000_000_000_000;

    (0..=TRILLION).binary_search_by(|it| count_ore("FUEL", it, inp).cmp(&TRILLION))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_p1() {
        let inp = "9 ORE => 2 A\n\
                          8 ORE => 3 B\n\
                          7 ORE => 5 C\n\
                          3 A, 4 B => 1 AB\n\
                          5 B, 7 C => 1 BC\n\
                          4 C, 1 A => 1 CA\n\
                          2 AB, 3 BC, 4 CA => 1 FUEL";

        let data = generate(inp);

        assert_eq!(9, count_ore("A", 1, &data));
        assert_eq!(8, count_ore("B", 1, &data));
        assert_eq!(7, count_ore("C", 1, &data));

        assert_eq!(2 * 9 + 2 * 8, count_ore("AB", 1, &data));
        assert_eq!(2 * 8 + 2 * 7, count_ore("BC", 1, &data));
        assert_eq!(7 + 9, count_ore("CA", 1, &data));
        assert_eq!(165, count_ore("FUEL", 1, &data));

        let inp = "157 ORE => 5 NZVS\n\
                          165 ORE => 6 DCFZ\n\
                          44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
                          12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
                          179 ORE => 7 PSHF\n\
                          177 ORE => 5 HKGWZ\n\
                          7 DCFZ, 7 PSHF => 2 XJWVT\n\
                          165 ORE => 2 GPVTF\n\
                          3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        let data = generate(inp);
        assert_eq!(13312, count_ore("FUEL", 1, &data));

        let inp = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
                          17 NVRVD, 3 JNWZP => 8 VPVL\n\
                          53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
                          22 VJHF, 37 MNCFX => 5 FWMGM\n\
                          139 ORE => 4 NVRVD\n\
                          144 ORE => 7 JNWZP\n\
                          5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
                          5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
                          145 ORE => 6 MNCFX\n\
                          1 NVRVD => 8 CXFTF\n\
                          1 VJHF, 6 MNCFX => 4 RFSQX\n\
                          176 ORE => 6 VJHF";

        let data = generate(inp);
        assert_eq!(180697, count_ore("FUEL", 1, &data));

        let inp = "171 ORE => 8 CNZTR\n\
                          7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
                          114 ORE => 4 BHXH\n\
                          14 VRPVC => 6 BMBT\n\
                          6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
                          6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
                          15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
                          13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
                          5 BMBT => 4 WPTQ\n\
                          189 ORE => 9 KTJDG\n\
                          1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
                          12 VRPVC, 27 CNZTR => 2 XDBXC\n\
                          15 KTJDG, 12 BHXH => 5 XCVML\n\
                          3 BHXH, 2 VRPVC => 7 MZWV\n\
                          121 ORE => 7 VRPVC\n\
                          7 XCVML => 6 RJRHP\n\
                          5 BHXH, 4 VRPVC => 5 LTCX";

        let data = generate(inp);
        assert_eq!(2210736, count_ore("FUEL", 1, &data));
    }

    #[test]
    fn test_samples_p2() {
        let inp = "157 ORE => 5 NZVS\n\
                          165 ORE => 6 DCFZ\n\
                          44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
                          12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
                          179 ORE => 7 PSHF\n\
                          177 ORE => 5 HKGWZ\n\
                          7 DCFZ, 7 PSHF => 2 XJWVT\n\
                          165 ORE => 2 GPVTF\n\
                          3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        let data = generate(inp);
        assert_eq!(82892753, part2(&data));

        let inp = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
                          17 NVRVD, 3 JNWZP => 8 VPVL\n\
                          53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
                          22 VJHF, 37 MNCFX => 5 FWMGM\n\
                          139 ORE => 4 NVRVD\n\
                          144 ORE => 7 JNWZP\n\
                          5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
                          5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
                          145 ORE => 6 MNCFX\n\
                          1 NVRVD => 8 CXFTF\n\
                          1 VJHF, 6 MNCFX => 4 RFSQX\n\
                          176 ORE => 6 VJHF";

        let data = generate(inp);
        assert_eq!(5586022, part2(&data));

        let inp = "171 ORE => 8 CNZTR\n\
                          7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
                          114 ORE => 4 BHXH\n\
                          14 VRPVC => 6 BMBT\n\
                          6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
                          6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
                          15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
                          13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
                          5 BMBT => 4 WPTQ\n\
                          189 ORE => 9 KTJDG\n\
                          1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
                          12 VRPVC, 27 CNZTR => 2 XDBXC\n\
                          15 KTJDG, 12 BHXH => 5 XCVML\n\
                          3 BHXH, 2 VRPVC => 7 MZWV\n\
                          121 ORE => 7 VRPVC\n\
                          7 XCVML => 6 RJRHP\n\
                          5 BHXH, 4 VRPVC => 5 LTCX";

        let data = generate(inp);
        assert_eq!(460664, part2(&data));
    }
}
