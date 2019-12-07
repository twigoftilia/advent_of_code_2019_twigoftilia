use crate::util;
use regex::Regex;
use std::collections::HashMap;

static DAY_6_INPUT: &str = include_str!(r"../input/day-6.txt");

pub fn solve() {
    let v = util::rows_to_vector(DAY_6_INPUT);
    println!("Day 6  answers");
    print!(" first puzzle: ");
    println!("{}", solve_first(&v));
}

#[derive(Debug)]
struct OrbitNode {
    orbits: Option<String>,
    kiddos: Vec<String>,
}

fn solve_first(input: &[&str]) -> usize {
    let mut orbit_nodes: HashMap<String, OrbitNode> = HashMap::new();
    let re = Regex::new(r"^(\w+)\)(\w+)$").unwrap();
    for s in input {
        let caps = re.captures(s);
        if let Some(caps) = caps {
            let base = &caps[1];
            let satelite = &caps[2];

            if let Some(s_node) = orbit_nodes.get_mut(satelite) {
                s_node.orbits = Some(base.to_owned());
            } else {
                let on = OrbitNode {
                    orbits: Some(base.to_owned()),
                    kiddos: vec![],
                };
                orbit_nodes.insert(satelite.to_owned(), on);
            };

            if let Some(base_node) = orbit_nodes.get_mut(base) {
                base_node.kiddos.push(satelite.to_owned());
            } else {
                let on = OrbitNode {
                    orbits: None,
                    kiddos: vec![satelite.to_owned()],
                };
                orbit_nodes.insert(base.to_owned(), on);
            };
        } else {
            panic!("Can't parse line: {}", s);
        }
    }

    let mut start_nodes = vec![];

    for (name, orbit_node) in &orbit_nodes {
        if orbit_node.orbits == Option::None {
            start_nodes.push(String::from(name));
        }
    }
    let mut orbits = 0;

    for start_node in start_nodes {
        {
            orbits += get_orbits(&orbit_nodes, start_node, 0);
        }
    }

    fn get_orbits(
        orbit_nodes: &HashMap<String, OrbitNode>,
        node_key: String,
        inherited_orbits: i32,
    ) -> i32 {
        let current_node = orbit_nodes.get(&node_key).unwrap();

        let my_count;
        if let Some(_p) = &current_node.orbits {
            my_count = inherited_orbits + 1;
        } else {
            my_count = 0;
        }

        let mut underling_counts = 0;
        for underling in &current_node.kiddos {
            underling_counts += get_orbits(orbit_nodes, String::from(underling), my_count);
        }
        my_count + underling_counts
    }
    orbits as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_first() {
        // provided examples
        let i = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";

        let v = util::rows_to_vector(i);
        assert_eq!(solve_first(&v), 42);
    }
}
