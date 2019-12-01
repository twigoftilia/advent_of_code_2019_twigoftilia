use crate::util;

pub fn solve() {
    let v = util::integer_file_to_vector("input/day-1.txt");

    println!("Day 1 answers");
    print!(" first puzzle: ");
    println!("{}", solve_first(&v));
    print!(" second puzzle: ");
    println!("{}", solve_second(&v));
}

fn solve_first(ints: &[i32]) -> i32 {
    let mut fuel = 0;
    for weight in ints {
        fuel += weight / 3 - 2;
    }
    fuel
}

fn solve_second(ints: &[i32]) -> i32 {
    let mut fuel = 0;
    for weight in ints {
        let fuel_for_weight = weight / 3 - 2;
        let fuel_for_fuel = fuel_for_fuel(fuel_for_weight);
        fuel += fuel_for_weight + fuel_for_fuel;
    }
    fuel
}

fn fuel_for_fuel(weight: i32) -> i32 {
    let mut f4f = weight / 3 - 2;
    if f4f > 0 {
        f4f += fuel_for_fuel(f4f);
        return f4f;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_first_string(input: &str) -> i32 {
        let v = util::parse_string_of_ints_to_vec(input);
        solve_first(&v)
    }

    fn solve_second_string(input: &str) -> i32 {
        let v = util::parse_string_of_ints_to_vec(input);
        solve_second(&v)
    }

    #[test]
    fn test_cases_first() {
        // provided examples
        assert_eq!(solve_first_string("+12"), 2);
        assert_eq!(solve_first_string("14"), 2);
        assert_eq!(solve_first_string("1969"), 654);
        assert_eq!(solve_first_string("100756"), 33583);
        // my tests
        assert_eq!(solve_first_string("+12, 14"), 4);
    }

    #[test]
    fn test_cases_second() {
        // provided examples
        assert_eq!(solve_second_string("14"), 2);
        assert_eq!(solve_second_string("1969"), 966);
        assert_eq!(solve_second_string("100756"), 50346);
        // my tests
        assert_eq!(solve_second_string("14, 1969"), 968);
    }
}
