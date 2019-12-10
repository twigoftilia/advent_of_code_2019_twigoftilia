use crate::int_computer::*;
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

fn solve_first(program: &[i64]) -> i64 {
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

fn solve_second(program: &[i64]) -> i64 {
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
                    Interrupt::WaitForInput | Interrupt::Halt => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_first() {
        // provided examples

        let program: Vec<i64> = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let max = solve_first(&program);
        assert_eq!(max, 43210);

        let program: Vec<i64> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let max = solve_first(&program);
        assert_eq!(max, 54321);
    }

    #[test]
    fn test_cases_second() {
        // provided examples

        let program: Vec<i64> = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let max = solve_second(&program);
        assert_eq!(max, 139_629_729);

        let program: Vec<i64> = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let max = solve_second(&program);
        assert_eq!(max, 18216);
    }
}
