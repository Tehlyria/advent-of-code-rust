use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("cut {0}")]
    Cut(i64),

    #[display("deal with increment {0}")]
    WithIncrement(usize),

    #[display("deal into new stack")]
    NewStack,
}

impl Instruction {
    fn execute(&self, cards: &mut Vec<i64>) {
        match *self {
            Instruction::Cut(n) => {
                if n.is_positive() {
                    cards.rotate_left(n as usize);
                } else {
                    cards.rotate_right(n.unsigned_abs() as usize);
                }
            }
            Instruction::WithIncrement(n) => {
                let mut new_vec = vec![-1; cards.len()];
                for card_num in 0..cards.len() {
                    let next_idx = (card_num * n) % cards.len();
                    assert_eq!(new_vec[next_idx], -1);
                    new_vec[next_idx] = cards[card_num];
                }

                *cards = new_vec;
            }
            Instruction::NewStack => {
                cards.reverse();
            }
        };
    }
}

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day22, part1)]
pub fn part1(insts: &[Instruction]) -> Option<usize> {
    let cards = insts
        .iter()
        .fold((0i64..=10_006).collect_vec(), |mut cards, it| {
            it.execute(&mut cards);
            cards
        });

    cards.iter().position(|it| *it == 2019)
}

#[aoc(day22, part2)]
pub fn part2(_: &[Instruction]) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let mut inp = (0..10).collect_vec();
        Instruction::NewStack.execute(&mut inp);
        assert_eq!(inp, (0..10).rev().collect_vec());
    }

    #[test]
    fn test_cut_positive() {
        let mut inp = (0..10).collect_vec();
        Instruction::Cut(3).execute(&mut inp);
        assert_eq!(inp, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        let mut inp = (0..10).collect_vec();
        Instruction::Cut(-4).execute(&mut inp);
        assert_eq!(inp, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let mut inp = (0..10).collect_vec();
        Instruction::WithIncrement(3).execute(&mut inp);
        assert_eq!(inp, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
