use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    fs,
};
mod parser;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Reagent {
    //TODO should I be using &str everywhere?
    name: String,
    quantity: u64,
}

fn needs_with_fuel(amt: u64) -> HashMap<String, u64> {
    let mut s = HashMap::new();
    s.insert("FUEL".into(), amt);
    s
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct Reaction {
    inputs: Vec<Reagent>,
    output: Reagent,
}

fn depends_on(target: &String, dependency: &String, reactions: &Vec<Reaction>) -> bool {
    // println!("checking whether {} depends on {}", target, dependency);
    match reactions.iter().filter(|r| &r.output.name == target).nth(0) {
        None => {
            // There is no recipe for this element. So it must be ORE. So we never encountered
            // the dependency in question
            false
        }
        Some(recipe) => {
            if recipe
                .inputs
                .iter()
                .find(|i| &i.name == dependency)
                .is_some()
            {
                // It is a direct dependency
                true
            } else {
                // Not a direct dependency, so recurse
                recipe.inputs.iter().fold(false, |so_far, input| {
                    // Something to investigate: I initially forgot the `so_far ||`
                    // but could never make a test fail.
                    so_far || depends_on(&input.name, dependency, reactions)
                })
            }
        }
    }
}

/// Calculates the total ore that must be collected to create one fuel.
/// This solution is much more efficient than Djkstra's but is also less general. This relies on a
/// few assumptions, that I didn't notice when I first read the problem. Specifically,
/// 1. Each reaction has exactly one output
/// 2. Each element can be created by exactly one reaction (except ore)
fn dependency_traversal_solution(
    targets: &mut HashMap<String, u64>,
    reactions: &Vec<Reaction>,
) -> u64 {
    while targets
        .iter()
        .filter(|(name, _)| name != &&String::from("ORE"))
        .count()
        > 0
    {
        // println!("\n\nAnalyzing targets {:?}", targets);

        // First find a target that can be analyzed right now. O(n2)
        // A target can be analyzed now iff none of the other targets depend on it
        let mut build_now = String::from("nothing");
        'outer: for target in targets.iter() {
            for other_target in targets.iter() {
                if depends_on(&other_target.0, &target.0, reactions) {
                    // println!("{} depends on {}", target.0, other_target.0);
                    continue 'outer;
                }
            }
            build_now = target.0.clone();
        }

        // println!("{} can be built now ", build_now);

        // Look up the recipe to build the target
        let reaction = reactions
            .iter()
            .filter(|r| &r.output.name == &build_now)
            .nth(0)
            .expect("There is a reaction for ever chemical");

        // Figure out how many times this reaction can be applied
        let quantity_needed = targets
            .get(&reaction.output.name)
            .expect("we got this target from the targets map in the first place");
        let times_to_apply = quantity_needed / reaction.output.quantity
            + if quantity_needed % reaction.output.quantity == 0 {
                0
            } else {
                1
            };

        // println!("{:?} can be applied {} times", reaction, times_to_apply);

        // We do't need to "substract" the output. We're creating all that we need, so just remove it.
        targets.remove(&reaction.output.name);

        // Increase the inputs
        for input in &reaction.inputs {
            *targets.entry(input.name.clone()).or_insert(0) += times_to_apply * input.quantity;
        }

        // std::thread::sleep(std::time::Duration::from_millis(500));
    }

    *targets
        .get("ORE")
        .expect("The problem states we should only need ore")
}

fn main() {
    // Parse the puzzle input, which represents a set of reactions
    let reactions = parser::parse_reactions(&fs::read_to_string("input.txt").expect("file error"));

    // I initially solved part1 using Djkstra's algorithm which is correct, but is too slow.
    // println!(
    //     "Minimal ORE needed (Dijkstra method)  : {}",
    //     djkstra::dijkstra_solution(&reactions)
    // );

    // Part 1
    // let rough_ore_per_fuel = 907302u64;
    let rough_ore_per_fuel = dependency_traversal_solution(&mut needs_with_fuel(1), &reactions);
    println!(
        "Minimal ORE needed (Dependency method): {}",
        rough_ore_per_fuel
    );

    // Part 2

    // For part 2 we'll use the binary search to find the maximum number of fuel we can
    // create with the 1 trillion ore we collected.
    const COLLECTED_ORE: u64 = 1_000_000_000_000;

    // Begin by bracketing the potential solution
    // The worst possible case is that it takes as many ore to make each fuel as it took to make
    // the first one.
    let mut min_fuel = COLLECTED_ORE / rough_ore_per_fuel;

    // Assume we'll yield at most double that worst case
    let mut max_fuel = min_fuel * 2;

    // The guess at how much ore we'll need. We don't actually guess 0, this value is mutated
    // right after entering the loop.
    let mut candidate = 0u64;

    // Now implement a basic binary search
    while min_fuel < max_fuel {
        candidate = (max_fuel + min_fuel) / 2;

        let ore_needed = dependency_traversal_solution(&mut needs_with_fuel(candidate), &reactions);

        let percent = (ore_needed as f64) / (COLLECTED_ORE as f64) * 100.0;

        println!(
            "It takes {} ore ({:.0}%) to make {} fuel",
            ore_needed, percent, candidate
        );

        // TODO some off-by-one error here, but I still found the answer
        if ore_needed < COLLECTED_ORE {
            min_fuel = candidate;
        } else {
            max_fuel = candidate;
        }
    }

    println!("The most fuel we can make is {}", max_fuel);
}

// #[test]
// fn first_example() {
//     let input = "
//         10 ORE => 10 A
//         1 ORE => 1 B
//         7 A, 1 B => 1 C
//         7 A, 1 C => 1 D
//         7 A, 1 D => 1 E
//         7 A, 1 E => 1 FUEL
//     ";
//
//     let reactions = parser::parse_reactions(input);
//
//     assert_eq!(
//         dependency_traversal_solution(&mut needs_with_fuel(1), &reactions),
//         31
//     );
// }
//
// #[test]
// fn second_example() {
//     let input = "
//         9 ORE => 2 A
//         8 ORE => 3 B
//         7 ORE => 5 C
//         3 A, 4 B => 1 AB
//         5 B, 7 C => 1 BC
//         4 C, 1 A => 1 CA
//         2 AB, 3 BC, 4 CA => 1 FUEL
//     ";
//
//     let reactions = parser::parse_reactions(input);
//
//     assert_eq!(
//         dependency_traversal_solution(&mut needs_with_fuel(1), &reactions),
//         165
//     );
// }

#[test]
fn depends_on_fuel_ore() {
    let input = "
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
    ";

    let reactions = parser::parse_reactions(input);

    assert!(depends_on(
        &String::from("FUEL"),
        &String::from("ORE"),
        &reactions
    ));
}

#[test]
fn depends_on_a_b() {
    let input = "
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
    ";

    let reactions = parser::parse_reactions(input);

    assert!(!depends_on(
        &String::from("A"),
        &String::from("B"),
        &reactions
    ));
}

#[test]
fn depends_on_c_a() {
    let input = "
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
    ";

    let reactions = parser::parse_reactions(input);

    assert!(depends_on(
        &String::from("C"),
        &String::from("A"),
        &reactions
    ));
}

#[test]
fn doesnt_depend_on_self() {
    let input = "
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
    ";

    let reactions = parser::parse_reactions(input);

    assert!(!depends_on(
        &String::from("C"),
        &String::from("C"),
        &reactions
    ));
}

#[test]
fn fuel_depends_on_d() {
    let input = "
        10 ORE => 10 X
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
		1 X => 1 A
    ";

    let reactions = parser::parse_reactions(input);

    assert!(depends_on(
        &String::from("FUEL"),
        &String::from("D"),
        &reactions
    ));
}
