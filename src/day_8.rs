// use crate::util;

static DAY_8_INPUT: &str = include_str!(r"../input/day-8.txt");

pub fn solve() {
    println!("Day 8 answers");
    print!(" first puzzle: ");
    println!("{}", solve_first(DAY_8_INPUT, 25, 6));
    print!(" second puzzle: ");
    solve_second(DAY_8_INPUT, 25, 6);
}

fn solve_first(input: &str, width: i32, height: i32) -> usize {
    let layer_size = width * height;

    // dbg!(input);
    let data: Vec<u8> = input
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as u8))
        .collect();
    // dbg!(&data);

    let no_of_layers = data.len() as i32 / layer_size;
    let rest = data.len() as i32 % no_of_layers;
    // dbg!(layer_size, no_of_layers, rest);
    if rest != 0 {
        panic!("Bad data, last layer not complete");
    }

    let mut min_no_of_zeros = 0;
    let mut min_layer = Option::None;

    for current_layer_no in 0..no_of_layers {
        let layer_offset = (current_layer_no * layer_size) as usize;
        let layer_slice = &data[layer_offset..(layer_offset + layer_size as usize)];
        let zeros_in_layer = bytecount::count(layer_slice, 0);

        if min_layer.is_none() || zeros_in_layer < min_no_of_zeros {
            min_layer = Some(layer_slice);
            min_no_of_zeros = zeros_in_layer;
        }
    }
    let layer_slice = min_layer.unwrap();
    let ones_in_layer = bytecount::count(layer_slice, 1);
    let twos_in_layer = bytecount::count(layer_slice, 2);
    // dbg!(layer_slice, ones_in_layer, twos_in_layer);
    ones_in_layer * twos_in_layer
}

fn solve_second(input: &str, width: usize, height: usize) -> usize {
    let layer_size = width * height;

    let data: Vec<u8> = input
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as u8))
        .collect();
    // dbg!(&data);

    let no_of_layers = data.len() / layer_size;
    let mut min_no_of_zeros = 0;
    let mut min_layer = Option::None;

    for current_row in 0..height {
        println!();
        'next_pixel: for current_column in 0..width {
            let layer_offset = current_row * width + current_column;

            for current_layer in 0..no_of_layers {
                match data[current_layer * layer_size + layer_offset] {
                    0 => {
                        print!(" ");
                        continue 'next_pixel;
                    }
                    1 => {
                        print!("*");
                        continue 'next_pixel;
                    }
                    _ => {}
                }
            }
            print!("X");
            panic!("Corrupt data detected");
        }
    }

    for current_layer_no in 0..no_of_layers {
        let layer_offset = (current_layer_no * layer_size) as usize;
        let layer_slice = &data[layer_offset..(layer_offset + layer_size as usize)];
        let zeros_in_layer = bytecount::count(layer_slice, 0);

        if min_layer.is_none() || zeros_in_layer < min_no_of_zeros {
            min_layer = Some(layer_slice);
            min_no_of_zeros = zeros_in_layer;
        }
    }
    let layer_slice = min_layer.unwrap();
    let ones_in_layer = bytecount::count(layer_slice, 1);
    let twos_in_layer = bytecount::count(layer_slice, 2);
    // dbg!(layer_slice, ones_in_layer, twos_in_layer);
    ones_in_layer * twos_in_layer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_first() {
        let s = solve_first("123456789012", 3, 2);
        assert_eq!(s, 1);

        let s = solve_first("122456789012", 3, 2);
        assert_eq!(s, 2);
        let s = solve_first("122156789012", 3, 2);
        assert_eq!(s, 4);

        let s = solve_first("789012123456", 3, 2);
        assert_eq!(s, 1);

        let s = solve_first("789012122456", 3, 2);
        assert_eq!(s, 2);

        let s = solve_first("789012122156", 3, 2);
        assert_eq!(s, 4);
    }

    #[test]
    fn test_cases_second() {
        solve_second("0222112222120000", 2, 2);
    }
}
