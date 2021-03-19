pub struct IntCode {
    vpc: usize,
    rel_base: i64,
    mem: Vec<i64>,
}

#[derive(PartialEq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
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
const LT: i64 = 7;
const EQ: i64 = 8;
const RB: i64 = 9;
const HALT: i64 = 99;

impl IntCode {
    pub fn new(init_mem: &[i64]) -> Self {
        let mut vec = vec![0; 0x500];
        vec[..init_mem.len()].clone_from_slice(init_mem);

        Self {
            vpc: 0,
            rel_base: 0,
            mem: vec,
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
            let opc = self.mem[self.vpc];
            let cur_opcode = opc % 100;

            match cur_opcode {
                ADD => self.do_arith(|lhs, rhs| lhs + rhs),
                MUL => self.do_arith(|lhs, rhs| lhs * rhs),
                READ => return State::Waiting,
                WRITE => return State::Write(self.write()),
                JT => self.jump(|it| it != 0),
                JF => self.jump(|it| it == 0),
                LT => self.cmp(|lhs, rhs| lhs < rhs),
                EQ => self.cmp(|lhs, rhs| lhs == rhs),
                RB => self.set_rel_base(),
                HALT => return State::Halted(self.mem[0]),
                _ => panic!("Unknown opcode {}!", cur_opcode),
            }
        }
    }

    fn do_arith<F>(&mut self, f: F)
    where
        F: FnOnce(i64, i64) -> i64,
    {
        let new_val = f(self.get_param(1), self.get_param(2));
        self.set_param(3, new_val);
        self.vpc += 4;
    }

    pub fn input(&mut self, inp: i64) {
        self.set_param(1, inp);
        self.vpc += 2;
    }

    fn write(&mut self) -> i64 {
        let val = self.get_param(1);
        self.vpc += 2;

        val
    }

    fn jump<F>(&mut self, f: F)
    where
        F: FnOnce(i64) -> bool,
    {
        self.vpc = if f(self.get_param(1)) {
            self.get_param(2) as usize
        } else {
            self.vpc + 3
        }
    }

    fn cmp<F>(&mut self, f: F)
    where
        F: FnOnce(i64, i64) -> bool,
    {
        let cond = f(self.get_param(1), self.get_param(2));
        self.set_param(3, if cond { 1 } else { 0 });
        self.vpc += 4;
    }

    fn set_rel_base(&mut self) {
        self.rel_base += self.get_param(1);
        self.vpc += 2;
    }

    fn get_param_mode(&self, param_idx: i64) -> ParameterMode {
        let denom = 10 * 10i64.pow(param_idx as u32);
        let val = self.mem[self.vpc] / denom;

        match val % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown parameter mode!"),
        }
    }

    fn get_param(&self, param: i64) -> i64 {
        let memory = &self.mem;
        let val = memory[self.vpc + (param as usize)];

        match self.get_param_mode(param) {
            ParameterMode::Position => memory[val as usize],
            ParameterMode::Immediate => val,
            ParameterMode::Relative => memory[(self.rel_base + val) as usize],
        }
    }

    fn set_param(&mut self, param: i64, new_val: i64) {
        let val = self.mem[self.vpc + (param as usize)] as usize;

        match self.get_param_mode(param) {
            ParameterMode::Position => self.mem[val] = new_val,
            ParameterMode::Relative => self.mem[(self.rel_base as usize) + val] = new_val,
            _ => panic!("Invalid parameter mode for write!"),
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

        let mut vm = IntCode::new(&inp);
        vm.run();

        assert!(vm.mem.starts_with(&expected));
    }

    #[test]
    fn test_two() {
        let inp: Vec<i64> = vec![2, 3, 0, 3, 99];
        let expected: Vec<i64> = vec![2, 3, 0, 6, 99];

        let mut vm = IntCode::new(&inp);
        vm.run();

        assert!(vm.mem.starts_with(&expected));
    }

    #[test]
    fn test_three() {
        let inp: Vec<i64> = vec![2, 4, 4, 5, 99, 0];
        let expected: Vec<i64> = vec![2, 4, 4, 5, 99, 9801];

        let mut vm = IntCode::new(&inp);
        vm.run();

        assert!(vm.mem.starts_with(&expected));
    }

    #[test]
    fn test_four() {
        let inp: Vec<i64> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected: Vec<i64> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let mut vm = IntCode::new(&inp);
        vm.run();

        assert!(vm.mem.starts_with(&expected));
    }

    #[test]
    fn test_five() {
        let inp = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut vm = IntCode::new(&inp);

        for it in inp {
            if let State::Write(n) = vm.run() {
                assert_eq!(it, n)
            }
        }
    }

    #[test]
    fn test_six() {
        let inp = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let mut vm = IntCode::new(&inp);

        if let State::Write(n) = vm.run() {
            assert_eq!(1219070632396864, n)
        }
    }

    #[test]
    fn test_seven() {
        let inp = vec![104, 1125899906842624, 99];

        let mut vm = IntCode::new(&inp);

        if let State::Write(n) = vm.run() {
            assert_eq!(1125899906842624, n)
        }
    }

    #[test]
    fn test_mode() {
        let vm = IntCode::new(&vec![1002]);
        assert_eq!(vm.get_param_mode(1), ParameterMode::Position);
        assert_eq!(vm.get_param_mode(2), ParameterMode::Immediate);
        assert_eq!(vm.get_param_mode(3), ParameterMode::Position);

        let vm = IntCode::new(&vec![2002]);
        assert_eq!(vm.get_param_mode(1), ParameterMode::Position);
        assert_eq!(vm.get_param_mode(2), ParameterMode::Relative);
        assert_eq!(vm.get_param_mode(3), ParameterMode::Position);
    }
}
