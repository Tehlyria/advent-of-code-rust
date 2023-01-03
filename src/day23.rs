use crate::intcode::{IntCode, State};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day23)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day23, part1)]
pub fn part1(code: &[i64]) -> i64 {
    let mut vms = vec![IntCode::new(code); 50];
    let mut packets: HashMap<usize, VecDeque<(i64, i64)>> = HashMap::new();

    let mut first = true;
    let mut idx = 0;

    loop {
        for (vm_idx, vm) in vms.iter_mut().enumerate() {
            match vm.run() {
                State::Waiting => {
                    if first {
                        vm.input(idx);
                        idx += 1;
                    } else if let Some((x_value, y_value)) =
                        packets.entry(vm_idx).or_default().pop_front()
                    {
                        vm.input(x_value);
                        if State::Waiting == vm.run() {
                            vm.input(y_value);
                        } else {
                            panic!("Not waiting after receiving x-value");
                        }
                    } else {
                        vm.input(-1);
                    }
                }
                State::Write(target_addr) => {
                    if let State::Write(x_value) = vm.run() {
                        if let State::Write(y_value) = vm.run() {
                            if target_addr == 255 {
                                return y_value;
                            }
                            packets
                                .entry(target_addr as usize)
                                .or_default()
                                .push_back((x_value, y_value));
                        } else {
                            panic!("No y-value after x-value!");
                        }
                    } else {
                        panic!("No x-value after target-addr!");
                    }
                }
                State::Halted(_) => {}
            }
        }

        first = false;
    }
}

#[aoc(day23, part2)]
pub fn part2(code: &[i64]) -> i64 {
    let mut vms = vec![IntCode::new(code); 50];
    let mut packets: HashMap<usize, VecDeque<(i64, i64)>> = HashMap::new();

    let mut first = true;
    let mut network_addr = 0;

    let mut nat_package: Option<(i64, i64)> = None;

    let mut seen = HashSet::new();

    loop {
        let mut is_idle = 0;
        for (vm_idx, vm) in vms.iter_mut().enumerate() {
            match vm.run() {
                State::Waiting => {
                    if first {
                        vm.input(network_addr);
                        network_addr += 1;
                    } else if let Some((x_value, y_value)) =
                        packets.entry(vm_idx).or_default().pop_front()
                    {
                        vm.input(x_value);
                        if State::Waiting == vm.run() {
                            vm.input(y_value);
                        } else {
                            panic!("Not waiting after receiving x-value");
                        }
                    } else {
                        vm.input(-1);
                        is_idle += 1;
                    }
                }
                State::Write(target_addr) => {
                    if let State::Write(x_value) = vm.run() {
                        if let State::Write(y_value) = vm.run() {
                            if target_addr == 255 {
                                nat_package = Some((x_value, y_value));
                            } else {
                                packets
                                    .entry(target_addr as usize)
                                    .or_default()
                                    .push_back((x_value, y_value));
                            }
                        } else {
                            panic!("No y-value after x-value!");
                        }
                    } else {
                        panic!("No x-value after target-addr!");
                    }
                }
                State::Halted(_) => {}
            }
        }

        if is_idle == network_addr && packets.values().all(VecDeque::is_empty) {
            if let Some((x_value, y_value)) = nat_package {
                if seen.contains(&y_value) {
                    return y_value;
                }

                seen.insert(y_value);

                if let Some(it) = vms.first_mut() {
                    if State::Waiting == it.run() {
                        it.input(x_value);

                        if State::Waiting == it.run() {
                            it.input(y_value);
                        } else {
                            panic!("Not waiting on y-value after NAT send x-value!");
                        }
                    } else {
                        panic!("Network seems idle, but VM0 not waiting on NAT packet!");
                    }
                }
            }
        }

        first = false;
    }
}
