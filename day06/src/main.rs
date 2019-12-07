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

    // Print results
    println!("{:?}", running_total);
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
