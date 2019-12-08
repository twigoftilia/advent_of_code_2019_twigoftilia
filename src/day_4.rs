static DAY_4_INPUT_1_RANGE_START: usize = 240_298;
static DAY_4_INPUT_1_RANGE_END: usize = 784_956;

pub fn solve() {
    let (a1, a2) = solve_both(DAY_4_INPUT_1_RANGE_START, DAY_4_INPUT_1_RANGE_END);
    println!("Day 4 answers");
    print!(" first puzzle: ");
    println!("{}", a1);

    print!(" second puzzle: ");
    println!("{}", a2);
}

fn solve_both(range_start: usize, range_end: usize) -> (usize, usize) {
    let mut counter_1 = 0;
    let mut counter_2 = 0;

    let mut x = range_start;

    let mut num_vec: [usize; 6] = [0; 6];
    while x <= range_end {
        let mut n = x;
        // let mut unaltered_x = x;
        let mut mutated = false;
        for i in 0..6 {
            num_vec[5 - i] = n % 10;
            n /= 10;
        }

        //      dbg!(num_vec);
        // for i in 0..6 {
        //     println!("1 Pos val: {} {}", i, num_vec[i]);
        //     n /= 10;
        // }

        let mut fill = false;
        let mut cur_adjecents = 0;
        let mut max_adjecents = 0;
        let mut two_adjecents = false;

        for i in 0..6 {
            if fill {
                num_vec[i] = num_vec[i - 1];
                cur_adjecents += 1;
            } else if i > 0 && num_vec[i] < num_vec[i - 1] {
                num_vec[i] = num_vec[i - 1];
                mutated = true;
                cur_adjecents += 1;
                fill = true;
            } else if i > 0 && num_vec[i] == num_vec[i - 1] {
                cur_adjecents += 1;
            } else {
                max_adjecents = max_adjecents.max(cur_adjecents);
                if cur_adjecents == 1 {
                    two_adjecents = true;
                }

                cur_adjecents = 0;
            }

            if i == 5 {
                max_adjecents = max_adjecents.max(cur_adjecents);
                if cur_adjecents == 1 {
                    two_adjecents = true;
                }
            }
        }

        // for i in 0..6 {
        //     println!("2 Pos val: {} {}", i, num_vec[i]);
        //     n /= 10;
        // }
        //      dbg!(num_vec);

        if mutated {
            x = num_vec[0] * 100_000
                + num_vec[1] * 10_000
                + num_vec[2] * 1_000
                + num_vec[3] * 100
                + num_vec[4] * 10
                + num_vec[5]
        }

        // println!(
        //     "  {} {} {} {}",
        //     unaltered_x, x, max_adjecents, two_adjecents
        // );

        if max_adjecents > 0 && x <= range_end {
            counter_1 += 1;
        }
        if two_adjecents && x <= range_end {
            counter_2 += 1;
        }
        x += 1;
    }

    (counter_1, counter_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_first() {
        // provided examples

        // let (a1, a2) = solve_both(111_110, 111_112);
        // assert_eq!(a1, 2);
        // let (a1, a2) = solve_both(123_456, 123_467);
        // assert_eq!(a1, 1);

        let (a1, _a2) = solve_both(367_899, 367_899);
        assert_eq!(a1, 1);
    }
}
