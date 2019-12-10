use crate::util;

static DAY_2_PROGRAM_INPUT: &str = include_str!(r"../input/day-2.txt");
static DAY_2_INPUT_1_NOUN: usize = 12;
static DAY_2_INPUT_1_VERB: usize = 2;
static DAY_2_EXPECTED_2: usize = 19_690_720;

pub fn solve() {
    let mut v = util::parse_string_of_ints_to_vec(DAY_2_PROGRAM_INPUT);

    println!("Day 2 answers");
    print!(" first puzzle: ");
    println!(
        "{}",
        solve_first(&mut v, DAY_2_INPUT_1_NOUN, DAY_2_INPUT_1_VERB)
    );

    print!(" second puzzle: ");
    let answer = solve_second();
    println!("{}", answer);
}

fn solve_first(program: &mut [usize], noun: usize, verb: usize) -> usize {
    program[1] = noun;
    program[2] = verb;
    compute(program);
    program[0]
}

fn solve_second() -> usize {
    let program_template = util::parse_string_of_ints_to_vec(DAY_2_PROGRAM_INPUT);

    for n in 0..100 {
        for v in 0..100 {
            let mut program = program_template.clone();

            solve_first(&mut program, n, v);
            //dbg!(n, v);

            if program[0] == DAY_2_EXPECTED_2 {
                return 100 * n + v;
            }
        }
    }
    panic!("Could not resolve");
}

fn compute(program: &mut [usize]) {
    let mut ip = 0;
    loop {
        //        dbg!(ip);
        let instruction = program[ip];

        match instruction {
            // Add
            1 => {
                let (p1, p2, p3) = (program[ip + 1], program[ip + 2], program[ip + 3]);
                let (v1, v2) = (program[p1], program[p2]);
                program[p3] = v1 + v2;
            }
            // Multiply
            2 => {
                let (p1, p2, p3) = (program[ip + 1], program[ip + 2], program[ip + 3]);
                let (v1, v2) = (program[p1], program[p2]);
                program[p3] = v1 * v2;
            }
            99 => {
                break;
            }
            _ => {
                panic!(format!("Invalid instruction: {}", instruction));
            }
        }

        ip += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_first() {
        // provided examples

        let mut program = vec![1, 0, 0, 0, 99];
        compute(&mut program);
        assert_eq!(program, vec![2, 0, 0, 0, 99]);

        let mut program = vec![2, 4, 4, 5, 99, 0];
        compute(&mut program);
        assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);

        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        compute(&mut program);
        assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    // #[test]
    // fn test_cases_second() {
    //     // solve_second
    // }
}
