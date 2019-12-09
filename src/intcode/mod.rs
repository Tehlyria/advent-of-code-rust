pub struct IntCode {
    vpc: i64,
    mem: Vec<i64>,
}

#[derive(PartialEq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

pub enum State {
    Waiting,
    Write(i64),
    Halted(i64),
}

const ADD: i64 = 1;
const MUL: i64 = 2;
const READ: i64 = 3;
const WRITE: i64 = 4;
const JT: i64 = 5;
const JF: i64 = 6;
const JLT: i64 = 7;
const JE: i64 = 8;
const HALT: i64 = 99;

impl IntCode {
    pub fn new(init_mem: Vec<i64>) -> Self {
        Self {
            vpc: 0,
            mem: init_mem,
        }
    }

    pub fn run_with_input(&mut self, mut inp_idx: usize, inp: &[i64]) -> State {
        loop {
            match self.run() {
                State::Waiting => {
                    self.input(inp[inp_idx]);
                    inp_idx += 1;
                }
                s => return s,
            }
        }
    }

    pub fn run(&mut self) -> State {
        loop {
            let cur_opcode = self.mem[self.vpc as usize];
            match cur_opcode % 100 {
                ADD => self.do_add(cur_opcode),
                MUL => self.do_mul(cur_opcode),
                READ => return State::Waiting,
                WRITE => {
                    let val = self.write(cur_opcode);
                    return State::Write(val);
                }
                JT => self.do_jump(cur_opcode, |it: i64| it != 0),
                JF => self.do_jump(cur_opcode, |it: i64| it == 0),
                JLT => self.do_jump_cmp(cur_opcode, |lhs: i64, rhs: i64| lhs < rhs),
                JE => self.do_jump_cmp(cur_opcode, |lhs: i64, rhs: i64| lhs == rhs),
                HALT => {
                    return State::Halted(self.mem[0]);
                }
                _ => eprintln!("Unknown opcode {}!", cur_opcode % 100),
            }
        }
    }

    pub fn input(&mut self, inp: i64) {
        let dst_idx = self.mem[self.vpc as usize + 1];
        self.mem[dst_idx as usize] = inp;
        self.vpc += 2;
    }

    fn do_add(&mut self, opcode: i64) {
        let lhs = self.load_opcode(opcode, 1);
        let rhs = self.load_opcode(opcode, 2);

        let dest = self.mem[(self.vpc + 3) as usize];
        self.mem[dest as usize] = lhs + rhs;

        self.vpc += 4;
    }

    fn do_mul(&mut self, opcode: i64) {
        let lhs = self.load_opcode(opcode, 1);
        let rhs = self.load_opcode(opcode, 2);

        let dest = self.mem[(self.vpc + 3) as usize];
        self.mem[dest as usize] = lhs * rhs;

        self.vpc += 4;
    }

    fn write(&mut self, opcode: i64) -> i64 {
        let val = self.load_opcode(opcode, 1);
        self.vpc += 2;

        val
    }

    fn do_jump<F>(&mut self, opcode: i64, f: F)
    where
        F: Fn(i64) -> bool,
    {
        let val = self.load_opcode(opcode, 1);

        if f(val) {
            self.vpc = self.load_opcode(opcode, 2);
        } else {
            self.vpc += 3;
        }
    }

    fn do_jump_cmp<F>(&mut self, opcode: i64, f: F)
    where
        F: Fn(i64, i64) -> bool,
    {
        let lhs = self.load_opcode(opcode, 1);
        let rhs = self.load_opcode(opcode, 2);

        let dst = self.mem[(self.vpc + 3) as usize];

        self.mem[dst as usize] = if f(lhs, rhs) { 1 } else { 0 };
        self.vpc += 4;
    }

    fn load_opcode(&self, opcode: i64, param_num: i64) -> i64 {
        let val = self.mem[(self.vpc + param_num) as usize];
        match self.check_mode(opcode, param_num - 1) {
            ParameterMode::Position => self.mem[val as usize],
            ParameterMode::Immediate => val,
        }
    }

    fn check_mode(&self, opcode: i64, val: i64) -> ParameterMode {
        let s = opcode.to_string().chars().rev().collect::<String>();

        if s.len() <= 2 {
            return ParameterMode::Position;
        }

        let tail = s.chars().skip(2).collect::<String>();
        if val >= tail.len() as i64 {
            return ParameterMode::Position;
        }

        match tail.chars().nth(val as usize) {
            Some(c) => {
                return if c == '0' {
                    ParameterMode::Position
                } else {
                    ParameterMode::Immediate
                };
            }
            _ => panic!("Something went wrong!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let inp = vec![1, 0, 0, 0, 99];
        let expected: Vec<i64> = vec![2, 0, 0, 0, 99];

        let mut vm = IntCode::new(inp);
        vm.run();

        assert_eq!(vm.mem, expected);
    }

    #[test]
    fn test_two() {
        let mut inp: Vec<i64> = vec![2, 3, 0, 3, 99];
        let expected: Vec<i64> = vec![2, 3, 0, 6, 99];

        let mut vm = IntCode::new(inp);
        vm.run();

        assert_eq!(vm.mem, expected);
    }

    #[test]
    fn test_three() {
        let mut inp: Vec<i64> = vec![2, 4, 4, 5, 99, 0];
        let expected: Vec<i64> = vec![2, 4, 4, 5, 99, 9801];

        let mut vm = IntCode::new(inp);
        vm.run();

        assert_eq!(vm.mem, expected);
    }

    #[test]
    fn test_four() {
        let mut inp: Vec<i64> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected: Vec<i64> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let mut vm = IntCode::new(inp);
        vm.run();

        assert_eq!(vm.mem, expected);
    }

    #[test]
    fn test_mode() {
        let vm = IntCode::new(vec![]);
        assert_eq!(vm.check_mode(1002, 0), ParameterMode::Position);
        assert_eq!(vm.check_mode(1002, 1), ParameterMode::Immediate);
        assert_eq!(vm.check_mode(1002, 2), ParameterMode::Position);
    }
}
