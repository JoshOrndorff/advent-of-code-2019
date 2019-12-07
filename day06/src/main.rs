// 1799 is too low (should have been obvious, that's the number of input lines)
// 284017 is too low

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

// #[derive(PartialEq, Eq, Debug)]
// struct OrbitSystem {
//     tree: HashMap<&str, HashSet<&str>>,
// }

fn main() {
    // Read data from file
    let f = BufReader::new(File::open("input.txt").unwrap());
    let orbits: Vec<_> = f.lines().map(|l| l.unwrap()).collect();

    // Construct tree form of system
    let system = parse_system(&orbits);

    // Solve part 1
    let mut running_total = 0;
    part1(&system, &"COM", 0, &mut running_total);

    // Solve part 2
    let mut you_path = path(&system, &"COM", &"YOU").unwrap();
    you_path.reverse();
    let mut santa_path = path(&system, &"COM", &"SAN").unwrap();
    santa_path.reverse();

    // Print results
    println!("{}", running_total);
    println!("Path from com to you: {:?}", you_path);
    println!("Path from com to santa: {:?}", santa_path);
    println!("Distance from you to santa: {}", part2(you_path.as_slice(), &santa_path.as_slice()));
}

fn parse_system(orbits_strings: &Vec<String>) -> HashMap<&str, HashSet<&str>> {
    let mut system = HashMap::new();

    for orbit in orbits_strings {
        let halves = orbit.split(")").collect::<Vec<_>>();
        let center = halves[0];
        let outer = halves[1];

        // Create new key if necessary
        system.entry(center).or_insert(HashSet::new());

        // Add current orbiter
        system.get_mut(center).unwrap().insert(outer);
    }

    system
}

fn path(system: &HashMap<&str, HashSet<&str>>, start: &str, target: &str) -> Option<Vec<String>> {
    if start == target {
        Some(vec![])
    } else {
        match system.get(start) {
            None => None,
            Some(children) => {
                for child in children {
                    if let Some(mut p) = path(system, child, target) {
                        p.push(String::from(start));
                        return Some(p)
                    }
                }
                None
            }
        }
    }
}

fn part1(system: &HashMap<&str, HashSet<&str>>, start: &str, depth: usize, running_total: &mut usize) {
    *running_total += depth;
    match system.get(start) {
        None => {},
        Some(children) => {
            for child in children {
                part1(system, child, depth + 1, running_total);
            }
        }
    }
}

/// Takes two paths with a potentially common prefix. Chomps off the common prefix, and
/// returns the combined lengths of the other paths.
fn part2(path1: &[String], path2: &[String]) -> usize {
    if path1[0] == path2[0] {
        part2(&path1[1..path1.len()], &path2[1..path2.len()])
    } else {
        path1.len() + path2.len()
    }
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_full() {

        let given: Vec<String> = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect();

        let system = parse_system(&given);
        let mut answer = 0;

        part1(&system, "COM", 0, &mut answer);
        assert_eq!(answer, 42);
    }
}
