use crate::util;
use std::str::FromStr;

use std::collections::HashSet;

static DAY_3_INPUT: &str = include_str!(r"../input/day-3.txt");

pub fn solve() {
    println!("Day 3 answers");
    print!(" first puzzle: ");
    let (answer1, answer2) = solve_both(DAY_3_INPUT);
    println!("{}", answer1);

    print!(" second puzzle: ");
    // let answer = solve_second();
    println!("{}", answer2);
}

fn solve_both(wiring: &str) -> (usize, usize) {
    let wires = parse_wires(wiring);

    let wire_a = &wires[0];
    let wire_b = &wires[1];

    let points_wire_a = get_points_from_origo(wire_a);
    let points_wire_b = get_points_from_origo(wire_b);

    // dbg!(&points_wire_a);

    let intersections_intersections = points_wire_a.intersection(&points_wire_b);
    let mut intersections = HashSet::new();
    for intersection in intersections_intersections {
        intersections.insert(intersection);
    }

    let mut lowest = Option::None;
    for intersection in &intersections {
        let md = manhattan_distance(intersection, 0, 0);

        if let Some(current) = lowest {
            if md < current {
                lowest.replace(md);
            }
        } else {
            lowest = Some(md);
        }
    }
    let answer1 = lowest.unwrap() as usize;

    let mut lowest = Option::None;
    for intersection in &intersections {
        let w_a_dist = wire_dist_to_point(wire_a, intersection);
        let w_b_dist = wire_dist_to_point(wire_b, intersection);

        let len = w_a_dist + w_b_dist;

        if let Some(current) = lowest {
            if len < current {
                lowest.replace(len);
            }
        } else {
            lowest = Some(len);
        }
    }

    let answer2 = lowest.unwrap() as usize;
    (answer1, answer2)
}

fn wire_dist_to_point(wire: &[WireInfoEntry], point_in_line: &Point) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut previous_seg_length: i32 = 0;

    for entry in wire {
        let mut x_step: i32 = 0;
        let mut y_step: i32 = 0;
        match entry.direction {
            Direction::Left => x_step = -1,
            Direction::Right => x_step = 1,
            Direction::Down => y_step = -1,
            Direction::Up => y_step = 1,
        }

        let x_end = x + x_step * entry.length as i32;
        let y_end = y + y_step * entry.length as i32;

        match entry.direction {
            Direction::Left | Direction::Right => {
                if point_in_line.y == y_end
                    && point_in_line.x < x.max(x_end)
                    && point_in_line.x >= x.min(x_end)
                {
                    if entry.direction == Direction::Left {
                        return previous_seg_length + x - point_in_line.x;
                    }
                    return previous_seg_length + point_in_line.x - x;
                }
            }

            Direction::Down | Direction::Up => {
                if point_in_line.x == x_end
                    && point_in_line.y < y.max(y_end)
                    && point_in_line.y >= y.min(y_end)
                {
                    if entry.direction == Direction::Down {
                        return previous_seg_length + y - point_in_line.y;
                    }
                    return previous_seg_length + point_in_line.y - y;
                }
            }
        }

        previous_seg_length += entry.length as i32;
        x = x_end;
        y = y_end;
    }

    panic!();
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

#[derive(Debug, PartialEq)]
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

        let wiring = "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83";
        let (a1, a2) = solve_both(wiring);
        assert_eq!(a1, 159);
        assert_eq!(a2, 610);

        let wiring = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let (a1, _) = solve_both(wiring);

        println!("XXXXXXXX a1: {}", a1);
        assert_eq!(a1, 6);

        let wiring = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let (a1, a2) = solve_both(wiring);
        assert_eq!(a1, 135);
        assert_eq!(a2, 410);
    }

    // #[test]
    // fn test_cases_second() {
    //     // solve_second
    // }
}
