use crate::util;
use regex::Regex;
use std::collections::HashMap;

static DAY_6_INPUT: &str = include_str!(r"../input/day-6.txt");

pub fn solve() {
    let v = util::rows_to_vector(DAY_6_INPUT);
    let o = orbit_nodes(&v);
    println!("Day 6  answers");
    print!(" first puzzle: ");
    println!("{}", solve_first(&o));

    print!(" second puzzle: ");
    println!("{}", solve_second(&o));
}

#[derive(Debug)]
struct OrbitNode {
    orbits: Option<String>,
    kiddos: Vec<String>,
}

fn orbit_nodes(input: &[&str]) -> HashMap<String, OrbitNode> {
    let mut orbit_nodes: HashMap<String, OrbitNode> = HashMap::new();
    let re = Regex::new(r"^(\w+)\)(\w+)$").unwrap();
    for s in input {
        let caps = re.captures(s);
        if let Some(caps) = caps {
            let (base, satelite) = (&caps[1], &caps[2]);

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
    orbit_nodes
}

fn solve_first(orbit_nodes: &HashMap<String, OrbitNode>) -> i32 {
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
    get_orbits(&orbit_nodes, String::from("COM"), 0)
}

fn solve_second(orbit_nodes: &HashMap<String, OrbitNode>) -> i32 {
    fn get_path_to_from_com(
        orbit_nodes: &HashMap<String, OrbitNode>,
        target_node_name: String,
    ) -> Vec<String> {
        let mut path = vec![];

        let mut current_node_name = String::from(&target_node_name);

        // dbg!(&current_node_name);

        loop {
            let current_node = orbit_nodes.get(&current_node_name).unwrap();
            path.push(String::from(&current_node_name));

            // dbg!(&current_node);

            if let Some(parent_name) = &current_node.orbits {
                // dbg!(&current_node);
                current_node_name = String::from(parent_name);
            } else {
                break;
            }
            //path.push(String::new())
        }
        path.reverse();
        path
    }

    let you_path = get_path_to_from_com(&orbit_nodes, String::from("YOU"));
    let santa_path = get_path_to_from_com(&orbit_nodes, String::from("SAN"));

    let mut idx = 0;
    //   dbg!(&you_path, &santa_path);

    let last_shared_idx = loop {
        //       dbg!(you_path.get(idx), santa_path.get(idx));
        if you_path.get(idx) == santa_path.get(idx) {
            idx += 1;
        } else {
            break idx as i32;
        }
    };
    //   dbg!(last_shared_idx);

    you_path.len() as i32 + santa_path.len() as i32 - 2 * last_shared_idx - 2 // don't count santa or you so remove 2
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
        let o = orbit_nodes(&v);
        assert_eq!(solve_first(&o), 42);
    }

    #[test]
    fn test_cases_second() {
        // provided examples
        let i = "    COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L
    K)YOU
    I)SAN";
        let v = util::rows_to_vector(i);
        let o = orbit_nodes(&v);
        assert_eq!(solve_second(&o), 4);
    }
}
