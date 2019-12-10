use crate::int_computer::*;
use crate::util;

static DAY_9_PROGRAM_INPUT: &str = include_str!(r"../input/day-9.txt");

pub fn solve() {
    let v = util::parse_string_of_ints_to_vec(DAY_9_PROGRAM_INPUT);
    println!("Day 9 answers");
    print!(" first puzzle: ");
    println!("{}", solve_first(&v));
    print!(" second puzzle: ");
    println!("{}", solve_second(&v));
}

fn solve_first(program: &[i64]) -> i64 {
    let mut computer: Computer = Computer::new(program);
    computer.inputs.push(1);
    computer.compute(false);
    computer.outputs.pop().unwrap()
}

fn solve_second(program: &[i64]) -> i64 {
    let mut computer: Computer = Computer::new(program);
    computer.inputs.push(2);
    computer.compute(false);
    computer.outputs.pop().unwrap()
}
