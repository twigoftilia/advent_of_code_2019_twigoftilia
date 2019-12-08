use crate::util;

static DAY_5_PROGRAM_INPUT: &str = include_str!(r"../input/day-5.txt");

pub fn solve() {
    let mut v = util::parse_string_of_ints_to_vec(DAY_5_PROGRAM_INPUT);

    println!("Day 5 answers");
    print!(" first puzzle: ");
    println!("{}", solve_first(&mut v));

    let mut v = util::parse_string_of_ints_to_vec(DAY_5_PROGRAM_INPUT);
    print!(" second puzzle: ");
    println!("{}", solve_second(&mut v));
}

fn solve_first(program: &mut [i32]) -> i32 {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    inputs.push(1);
    compute(program, &mut inputs, &mut outputs);
    outputs[outputs.len() - 1]
}

fn solve_second(program: &mut [i32]) -> i32 {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    inputs.push(5);
    compute(program, &mut inputs, &mut outputs);
    outputs[outputs.len() - 1]
}

fn compute(program: &mut [i32], inputs: &mut Vec<i32>, outputs: &mut Vec<i32>) {
    let mut ip = 0;
    let mut length;

    loop {
        //        dbg!(ip);
        let instruction_raw = program[ip];
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
                let (p1, p2, p3) = (program[ip + 1], program[ip + 2], program[ip + 3]);
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
                let (p1, p2, p3) = (program[ip + 1], program[ip + 2], program[ip + 3]);
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
                let input = inputs.pop();
                //                dbg!(input);

                let p1 = program[ip + 1];
                program[p1 as usize] = input.unwrap();
            }
            // Output
            4 => {
                length = 2;
                let p1 = program[ip + 1];
                let v1 = if mode_p1 == 0 {
                    program[p1 as usize]
                } else {
                    p1
                };

                outputs.push(v1);
            }
            // jump-if-true
            // Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer
            // to the value from the second parameter. Otherwise, it does nothing.
            5 => {
                length = 3;
                let p1 = program[ip + 1];
                let v1 = if mode_p1 == 0 {
                    program[p1 as usize]
                } else {
                    p1
                };

                if v1 != 0 {
                    let p2 = program[ip + 2];
                    let v2 = if mode_p2 == 0 {
                        program[p2 as usize]
                    } else {
                        p2
                    };

                    ip = v2 as usize;
                    ip_auto = false;
                }
            }
            // jump-if-false
            // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer
            // to the value from the second parameter. Otherwise, it does nothing.
            6 => {
                length = 3;
                let p1 = program[ip + 1];
                let v1 = if mode_p1 == 0 {
                    program[p1 as usize]
                } else {
                    p1
                };

                if v1 == 0 {
                    let p2 = program[ip + 2];
                    let v2 = if mode_p2 == 0 {
                        program[p2 as usize]
                    } else {
                        p2
                    };

                    //            dbg!(p1, p2, v1, v2);

                    ip = v2 as usize;
                    ip_auto = false;
                }
            }
            // less than
            // Opcode 7 is less than: if the first parameter is less than the second parameter,
            // it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            7 => {
                length = 4;
                let (p1, p2, p3) = (program[ip + 1], program[ip + 2], program[ip + 3]);
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
                let (p1, p2, p3) = (program[ip + 1], program[ip + 2], program[ip + 3]);
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
                break;
            }
            _ => {
                panic!(format!("Invalid instruction: {}", instruction));
            }
        }

        if ip_auto {
            ip += length;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_first() {
        // provided examples

        let mut inputs = Vec::new();
        inputs.push(1);
        let mut outputs = Vec::new();

        let mut program = vec![1, 0, 0, 0, 99];
        compute(&mut program, &mut inputs, &mut outputs);
        assert_eq!(program, vec![2, 0, 0, 0, 99]);

        let mut program = vec![2, 4, 4, 5, 99, 0];
        compute(&mut program, &mut inputs, &mut outputs);
        assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);

        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        compute(&mut program, &mut inputs, &mut outputs);
        assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);

        let mut inputs = Vec::new();
        inputs.push(42);
        let mut outputs = Vec::new();

        let mut program = vec![3, 0, 4, 0, 99];
        compute(&mut program, &mut inputs, &mut outputs);
        assert_eq!(outputs, vec![42]);

        // dbg!(outputs);

        // 8 => 1 else 0
        let mut inputs = Vec::new();
        inputs.push(8);
        let mut outputs = Vec::new();

        let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![1]);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        inputs.push(7);

        let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![0]);

        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).

        let mut inputs = Vec::new();
        inputs.push(7);
        let mut outputs = Vec::new();

        let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![1]);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        inputs.push(8);

        let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![0]);

        // 3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).

        let mut inputs = Vec::new();
        inputs.push(8);
        let mut outputs = Vec::new();

        let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![1]);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        inputs.push(7);

        let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![0]);

        // 3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).

        let mut inputs = Vec::new();
        inputs.push(7);
        let mut outputs = Vec::new();

        let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![1]);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        inputs.push(9);

        let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![0]);

        //Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        //3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
        //3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)

        let mut inputs = Vec::new();
        inputs.push(0);
        let mut outputs = Vec::new();

        let mut program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![0]);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        inputs.push(9);

        let mut program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        compute(&mut program, &mut inputs, &mut outputs);

        //        dbg!(&outputs);
        assert_eq!(outputs, vec![1]);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        inputs.push(9);

        let mut program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![1]);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        inputs.push(9);

        let mut program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![1]);

        // Longer example
        let mut inputs = Vec::new();
        inputs.push(9);
        let mut outputs = Vec::new();

        let mut program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        compute(&mut program, &mut inputs, &mut outputs);

        // dbg!(&outputs);
        assert_eq!(outputs, vec![1001]);
    }
}
