#[derive(Debug)]
pub struct Computer {
    pub program: Vec<i64>,
    pub inputs: Vec<i64>,
    pub outputs: Vec<i64>,
    ip: usize,
    relative_base: i32,
    pub halted: bool,
}

#[derive(Debug)]
pub enum Interrupt {
    Halt,
    WaitForInput,
    HasOutput,
    // GuruMeditation,
}
impl Computer {
    pub fn new(program: &[i64]) -> Computer {
        Computer {
            program: {
                let mut p = program.to_owned();
                p.resize(p.len() + 1000, 0);
                p
            },
            inputs: Vec::new(),
            outputs: Vec::new(),
            ip: 0,
            relative_base: 0,
            halted: false,
        }
    }

    pub fn compute(&mut self, interrupt_for_output: bool) -> Interrupt {
        let ip = &mut self.ip;
        let mut length;
        let program = &mut self.program;
        let relative_base = &mut self.relative_base;

        loop {
            let instruction_raw = program[*ip];
            let instruction = instruction_raw % 100;
            let mode_p1 = instruction_raw / 100 % 10;
            let mode_p2 = instruction_raw / 1000 % 10;
            let mode_p3 = instruction_raw / 10000 % 10;

            let read_value = |reg_val, mode| -> i64 {
                if mode == 0 {
                    program[reg_val as usize]
                } else if mode == 1 {
                    reg_val
                } else {
                    program[(reg_val + *relative_base as i64) as usize]
                }
            };

            let write_pos = |reg_val, mode| -> usize {
                if mode == 0 {
                    reg_val as usize
                } else if mode == 2 {
                    (reg_val + *relative_base as i64) as usize
                } else {
                    panic!("Illegal write mode");
                }
            };

            let mut ip_auto = true;
            // dbg!(instruction_raw, instruction, mode_p1, mode_p2, mode_p3);
            match instruction {
                // Add
                1 => {
                    length = 4;
                    let (p1, p2, p3) = (program[*ip + 1], program[*ip + 2], program[*ip + 3]);
                    let v1 = read_value(p1, mode_p1);
                    let v2 = read_value(p2, mode_p2);
                    program[write_pos(p3, mode_p3)] = v1 + v2;
                }
                // Multiply
                2 => {
                    length = 4;
                    let (p1, p2, p3) = (program[*ip + 1], program[*ip + 2], program[*ip + 3]);
                    let v1 = read_value(p1, mode_p1);
                    let v2 = read_value(p2, mode_p2);
                    program[write_pos(p3, mode_p3)] = v1 * v2;
                }
                // Input
                3 => {
                    length = 2;
                    if self.inputs.is_empty() {
                        return Interrupt::WaitForInput;
                    }
                    let input = self.inputs.pop();
                    //                dbg!(input);
                    let p1 = program[*ip + 1];
                    program[write_pos(p1, mode_p1)] = input.unwrap();
                }
                // Output
                4 => {
                    length = 2;
                    let p1 = program[*ip + 1];
                    let v1 = read_value(p1, mode_p1);
                    self.outputs.push(v1);
                }
                // jump-if-true
                // Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer
                // to the value from the second parameter. Otherwise, it does nothing.
                5 => {
                    length = 3;
                    let p1 = program[*ip + 1];
                    let v1 = read_value(p1, mode_p1);
                    if v1 != 0 {
                        let p2 = program[*ip + 2];
                        let v2 = read_value(p2, mode_p2);
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
                    let v1 = read_value(p1, mode_p1);
                    if v1 == 0 {
                        let p2 = program[*ip + 2];
                        let v2 = read_value(p2, mode_p2);
                        //            dbg!(p1, p2, v1, v2);
                        *ip = v2 as usize;
                        ip_auto = false;
                    }
                }
                7 => {
                    length = 4;
                    let (p1, p2, p3) = (program[*ip + 1], program[*ip + 2], program[*ip + 3]);
                    let v1 = read_value(p1, mode_p1);
                    let v2 = read_value(p2, mode_p2);
                    program[write_pos(p3, mode_p3)] = if v1 < v2 { 1 } else { 0 };
                }
                // equals
                // Opcode 8 is equals: if the first parameter is equal to the second parameter,
                // it stores 1 in the position given by the third parameter. Otherwise, it stores 0
                8 => {
                    length = 4;
                    let (p1, p2, p3) = (program[*ip + 1], program[*ip + 2], program[*ip + 3]);
                    let v1 = read_value(p1, mode_p1);
                    let v2 = read_value(p2, mode_p2);
                    program[write_pos(p3, mode_p3)] = if v1 == v2 { 1 } else { 0 };
                }
                // Adjust the relative base
                9 => {
                    length = 2;
                    let p1 = program[*ip + 1];
                    let v1 = read_value(p1, mode_p1);
                    *relative_base += v1 as i32;
                }
                99 => {
                    self.halted = true;
                    return Interrupt::Halt;
                }
                _ => {
                    panic!(format!("Invalid instruction: {}", instruction));
                }
            }
            if ip_auto {
                *ip += length;
            }
            if interrupt_for_output && instruction == 4 {
                return Interrupt::HasOutput;
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

        let program: Vec<i64> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut computer: Computer = Computer::new(&program);
        computer.compute(false);
        assert_eq!(program, computer.outputs);

        let program: Vec<i64> = vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        let mut computer = Computer::new(&program);
        computer.compute(false);
        dbg!(computer.outputs);

        let x: i64 = 1_125_899_906_842_624;
        let program: Vec<i64> = vec![104, x, 99];
        let mut computer = Computer::new(&program);
        computer.compute(false);
        assert_eq!(computer.outputs.pop().unwrap(), x);
    }

    #[test]
    fn test_cases_second() {}
}
