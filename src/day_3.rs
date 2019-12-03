use crate::util;
use std::str::FromStr;

use std::collections::HashSet;

static DAY_3_INPUT: &str = include_str!(r"../input/day-3.txt");

pub fn solve() {
    println!("Day 3 answers");
    print!(" first puzzle: ");
    println!("{}", solve_first(DAY_3_INPUT));

    // print!(" second puzzle: ");
    // let answer = solve_second();
    // println!("{}", answer);
}

fn solve_first(wiring: &str) -> usize {
    let wires = parse_wires(wiring);

    let wire_a = &wires[0];
    let wire_b = &wires[1];

    //  dbg!(wires);

    let points_wire_a = get_points_from_origo(wire_a);
    let points_wire_b = get_points_from_origo(wire_b);

    // dbg!(&points_wire_a);

    let intersections = points_wire_a.intersection(&points_wire_b);

    let mut lowest = Option::None;
    for intersection in intersections {
        let md = manhattan_distance(intersection, 0, 0);

        if let Some(current) = lowest {
            if md < current {
                lowest.replace(md);
            }
        } else {
            lowest = Some(md);
        }
    }
    lowest.unwrap() as usize
}

fn get_points_from_origo(wire: &[WireInfoEntry]) -> HashSet<Point> {
    let mut res = HashSet::new();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for entry in wire {
        //  dbg!(entry);

        let mut x_step: i32 = 0;
        let mut y_step: i32 = 0;
        match entry.direction {
            Direction::Left => x_step = -1,
            Direction::Right => x_step = 1,
            Direction::Down => y_step = -1,
            Direction::Up => y_step = 1,
        }

        for _p in 0..entry.length {
            x += x_step;
            y += y_step;
            res.insert(Point { x, y });
        }
    }

    res
}

#[derive(Debug)]
struct WireInfoEntry {
    direction: Direction,
    length: usize,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for WireInfoEntry {
    //type Err = ();
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match &s[0..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(String::from("Error parsing direction at pos 0")),
        };

        // let length: usize = *s[0].parse.expect();
        let length: usize = s[1..].parse().unwrap();
        //        dbg!(s, &direction, length);
        Ok(WireInfoEntry { direction, length })
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn manhattan_distance(point: &Point, x: i32, y: i32) -> i32 {
    point.x.max(x) - point.x.min(x) + point.y.max(y) - point.y.min(y)
}

fn parse_wires(input_lines: &str) -> Vec<Vec<WireInfoEntry>> {
    let rows = util::rows_to_vector(input_lines);
    let mut result = Vec::new();
    for row in rows {
        let entry_vec = row
            .split(',')
            .map(FromStr::from_str)
            .collect::<Result<_, _>>()
            .unwrap();
        result.push(entry_vec);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_first() {
        // provided examples

        let wiring = "R8,U5,L5,D3\nU7,R6,D4,L4";
        assert_eq!(solve_first(wiring), 6);

        let wiring = "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(solve_first(wiring), 159);

        let wiring = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(solve_first(wiring), 135);
    }

    // #[test]
    // fn test_cases_second() {
    //     // solve_second
    // }
}
