use crate::util;

use permutohedron::heap_recursive;

static DAY_7_PROGRAM_INPUT: &str = include_str!(r"../input/day-7.txt");

pub fn solve() {
    let v = util::parse_string_of_ints_to_vec(DAY_7_PROGRAM_INPUT);
    println!("Day 7 answers");
    print!(" first puzzle: ");
    println!("{}", solve_first(&v));
    print!(" second puzzle: ");
    println!("{}", solve_second(&v));
}

fn solve_first(program: &[i32]) -> i32 {
    let mut permutations = Vec::new();
    heap_recursive(&mut [0, 1, 2, 3, 4], |permutation| {
        permutations.push(permutation.to_vec())
    });
    let mut max_signal = 0;
    for p in permutations {
        let mut signal = 0;
        for i in p {
            let mut computer = Computer::new(program);
            computer.inputs.push(signal);
            computer.inputs.push(i);
            computer.compute(false);
            signal = computer.outputs.pop().unwrap();
        }
        max_signal = max_signal.max(signal);
    }
    max_signal
}

fn solve_second(program: &[i32]) -> i32 {
    let mut permutations = Vec::new();
    heap_recursive(&mut [5, 6, 7, 8, 9], |permutation| {
        permutations.push(permutation.to_vec())
    });
    let mut max_signal = 0;
    'perm: for p in permutations {
        let mut signal = 0;
        let mut signal_from_last = 0;
        let mut initialized = false;
        let mut computers = Vec::new();
        let last_computer_in_perm = p.last().unwrap();

        for _i in &p {
            let c = Computer::new(program);
            computers.push(c);
        }

        loop {
            'next_computer: for (idx, i) in p.iter().enumerate() {
                let computer = &mut computers[idx];

                if computer.halted {
                    // k, we are done here
                    max_signal = max_signal.max(signal_from_last);
                    continue 'perm;
                }

                computer.inputs.push(signal);
                if !initialized {
                    // dbg!(&p, tmp, idx, i, last_computer_in_perm);
                    computer.inputs.push(*i);
                    if i == last_computer_in_perm {
                        initialized = true;
                    }
                }

                //let mut prog = &programs[idx];
                let reason = computer.compute(false);

                match reason {
                    ComputeStopReason::WaitForInput | ComputeStopReason::Halt => {
                        if let Some(out) = computer.outputs.pop() {
                            signal = out;
                        }

                        if i == last_computer_in_perm {
                            signal_from_last = signal;
                        }
                        max_signal = max_signal.max(signal_from_last);
                        continue 'next_computer;
                    }
                    _ => panic!("Unexpected halt reason {:?}", reason),
                }
            }
        }
    }
    max_signal
}

struct Computer {
    program: Vec<i32>,
    inputs: Vec<i32>,
    outputs: Vec<i32>,
    ip: usize,
    halted: bool,
}

#[derive(Debug)]
enum ComputeStopReason {
    Halt,
    WaitForInput,
    HasOutput,
    // GuruMeditation,
}
impl Computer {
    fn new(program: &[i32]) -> Computer {
        Computer {
            program: program.to_owned(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            ip: 0,
            halted: false,
        }
    }

    fn compute(&mut self, interrupt_for_output: bool) -> ComputeStopReason {
        let ip = &mut self.ip;
        let mut length;
        let program = &mut self.program;

        loop {
            let instruction_raw = program[*ip];
            let instruction = instruction_raw % 100;
            let mode_p1 = instruction_raw / 100 % 10;
            let mode_p2 = instruction_raw / 1000 % 10;
            let _mode_p3 = instruction_raw / 10000 % 10;
            let mut ip_auto = true;
            // dbg!(instruction_raw, instruction, mode_p1, mode_p2, mode_p3);
            match instruction {
                // Add
                1 => {
                    length = 4;
                    let (p1, p2, p3) = (program[*ip + 1], program[*ip + 2], program[*ip + 3]);
                    let v1 = if mode_p1 == 0 {
                        program[p1 as usize]
                    } else {
                        p1
                    };
                    let v2 = if mode_p2 == 0 {
                        program[p2 as usize]
                    } else {
                        p2
                    };
                    program[p3 as usize] = v1 + v2;
                }
                // Multiply
                2 => {
                    length = 4;
                    let (p1, p2, p3) = (program[*ip + 1], program[*ip + 2], program[*ip + 3]);
                    let v1 = if mode_p1 == 0 {
                        program[p1 as usize]
                    } else {
                        p1
                    };
                    let v2 = if mode_p2 == 0 {
                        program[p2 as usize]
                    } else {
                        p2
                    };
                    program[p3 as usize] = v1 * v2;
                }
                // Input
                3 => {
                    length = 2;

                    if self.inputs.is_empty() {
                        return ComputeStopReason::WaitForInput;
                    }

                    let input = self.inputs.pop();
                    //                dbg!(input);
                    let p1 = program[*ip + 1];
                    program[p1 as usize] = input.unwrap();
                }
                // Output
                4 => {
                    length = 2;
                    let p1 = program[*ip + 1];
                    let v1 = if mode_p1 == 0 {
                        program[p1 as usize]
                    } else {
                        p1
                    };
                    self.outputs.push(v1);
                }
                // jump-if-true
                // Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer
                // to the value from the second parameter. Otherwise, it does nothing.
                5 => {
                    length = 3;
                    let p1 = program[*ip + 1];
                    let v1 = if mode_p1 == 0 {
                        program[p1 as usize]
                    } else {
                        p1
                    };
                    if v1 != 0 {
                        let p2 = program[*ip + 2];
                        let v2 = if mode_p2 == 0 {
                            program[p2 as usize]
                        } else {
                            p2
                        };
                        *ip = v2 as usize;
                        ip_auto = false;
                    }
                }
                // jump-if-false
                // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer
                // to the value from the second parameter. Otherwise, it does nothing.
                6 => {
                    length = 3;
                    let p1 = program[*ip + 1];
                    let v1 = if mode_p1 == 0 {
                        program[p1 as usize]
                    } else {
                        p1
                    };
                    if v1 == 0 {
                        let p2 = program[*ip + 2];
                        let v2 = if mode_p2 == 0 {
                            program[p2 as usize]
                        } else {
                            p2
                        };
                        //            dbg!(p1, p2, v1, v2);
                        *ip = v2 as usize;
                        ip_auto = false;
                    }
                }
                7 => {
                    length = 4;
                    let (p1, p2, p3) = (program[*ip + 1], program[*ip + 2], program[*ip + 3]);
                    let v1 = if mode_p1 == 0 {
                        program[p1 as usize]
                    } else {
                        p1
                    };
                    let v2 = if mode_p2 == 0 {
                        program[p2 as usize]
                    } else {
                        p2
                    };
                    program[p3 as usize] = if v1 < v2 { 1 } else { 0 };
                }
                // equals
                // Opcode 8 is equals: if the first parameter is equal to the second parameter,
                // it stores 1 in the position given by the third parameter. Otherwise, it stores 0
                8 => {
                    length = 4;
                    let (p1, p2, p3) = (program[*ip + 1], program[*ip + 2], program[*ip + 3]);
                    let v1 = if mode_p1 == 0 {
                        program[p1 as usize]
                    } else {
                        p1
                    };
                    let v2 = if mode_p2 == 0 {
                        program[p2 as usize]
                    } else {
                        p2
                    };
                    program[p3 as usize] = if v1 == v2 { 1 } else { 0 };
                }
                99 => {
                    self.halted = true;
                    return ComputeStopReason::Halt;
                }
                _ => {
                    panic!(format!("Invalid instruction: {}", instruction));
                }
            }
            if ip_auto {
                *ip += length;
            }
            if interrupt_for_output && instruction == 4 {
                return ComputeStopReason::HasOutput;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_first() {
        // provided examples

        let program: Vec<i32> = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let max = solve_first(&program);
        assert_eq!(max, 43210);

        let program: Vec<i32> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let max = solve_first(&program);
        assert_eq!(max, 54321);
    }

    #[test]
    fn test_cases_second() {
        // provided examples

        let program: Vec<i32> = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let max = solve_second(&program);
        assert_eq!(max, 139_629_729);

        let program: Vec<i32> = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let max = solve_second(&program);
        assert_eq!(max, 18216);
    }
}
