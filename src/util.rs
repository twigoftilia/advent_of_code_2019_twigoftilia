use std::fmt::Debug;
// Takes a csv row of integers and returns a vector with corresponding values
// Chars may or may not be prefixed with sign (+ or -).
// Whitespaces are ignored
// Empty values are not allows exept at end. And is then not counted
// Empty string -> empty vector
//
#[allow(dead_code)]
pub fn parse_string_of_ints_to_vec<T>(s: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    let s = s.trim();

    if s.is_empty() {
        return Vec::new();
    }

    s.split(',')
        .map(|x| x.trim().parse().expect("Not an integer"))
        .collect()
}

// Returns a vector of ints, taken from an aoc provided input file (one integer per row)
pub fn integer_file_buf_to_vector(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse().expect("Not an integer"))
        .collect()
}

// Returns a vector of trimmed strings with row content
pub fn rows_to_vector(input: &str) -> Vec<&str> {
    input.lines().map(|line| line.trim()).collect()
}
