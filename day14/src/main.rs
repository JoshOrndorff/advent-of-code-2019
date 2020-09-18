use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    fs,
};
mod djkstra;
mod parser;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Reagent {
    //TODO should I be using &str everywhere?
    name: String,
    quantity: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    reagents: HashMap<String, i64>,
    ore_consumed: u64,
}

impl State {
    fn new() -> Self {
        Self {
            reagents: HashMap::new(),
            ore_consumed: 0,
        }
    }

    fn new_one_fuel() -> Self {
        let mut s = Self {
            reagents: HashMap::new(),
            ore_consumed: 0,
        };
        s.reagents.insert("FUEL".into(), 1);
        s
    }

    fn get_prev_state(&self, reaction: &Reaction) -> Option<Self> {
        // Make a clone that we will mutate into the previous state
        let mut prev = self.clone();

        // Make sure we have the output necessary, and if so subtract it.
        let our_output_quantity = prev.reagents.get(&reaction.output.name).unwrap_or(&0);
        if our_output_quantity <= &0i64 {
            // This reaction doesn't apply, so return early
            return None;
        }

        prev.reagents.insert(
            reaction.output.name.clone(),
            our_output_quantity - reaction.output.quantity,
        );

        // Loop through the inputs adding them
        for input in &reaction.inputs {
            *prev.reagents.entry(input.name.clone()).or_insert(0) += input.quantity;
        }
        Some(prev)
    }

    fn has_fuel(&self) -> bool {
        match self.reagents.get("FUEL".into()) {
            None => false,
            Some(amount) => amount > &0,
        }
    }

    fn gather_ore(&self) -> Self {
        let mut next_state = self.clone();
        *next_state.reagents.entry("ORE".into()).or_insert(0) += 1;
        next_state.ore_consumed += 1;
        next_state
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct Reaction {
    inputs: Vec<Reagent>,
    output: Reagent,
}

/// Calculates the total ore that must be collected to create one fuel.
/// This solution is much more efficient than Djkstra's but is also less general. This relies on a
/// few assumptions, that I didn't notice when I first read the problem. Specifically,
/// 1. Each reaction has exactly one output
/// 2. Each element can be created by exactly one reaction (except ore)
fn dependency_traversal_solution(start: &State, reactions: &Vec<Reaction>) -> u64 {
    let mut current_state = start.clone();
    let mut prev_state = None;

    // Apply each reaction (in reverse) over and over until state stops changing
    while prev_state != Some(current_state.clone()) {
        prev_state = Some(current_state.clone());

        // Look through the list of reactions, applying (in reverse) all that can be
        current_state = reactions
            .iter()
            .fold(current_state, |s, r| s.get_prev_state(r).unwrap_or(s));
    }

    let signed_answer = *current_state
        .reagents
        .get("ORE".into())
        .expect("We should have some ore at the end or the problem is invalid.");

    signed_answer
        .try_into()
        .expect("The final answer should be positive, so convert it to a u64")
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
    let rough_ore_per_fuel = 907302u64;
    // let rough_ore_per_fuel = dependency_traversal_solution(&State::new_one_fuel(), &reactions);
    println!(
        "Minimal ORE needed (Dependency method): {}",
        rough_ore_per_fuel
    );

    // Part 2

    // For part 2 we'll use the binary search to find the maximum number of fuel we can
    // create with the 1 trillion ore we collected.

    // Begin by bracketing the potential solution
    // The worst possible case is that it takes as many ore to make each fuel as it took to make
    // the first one.
    let mut max = rough_ore_per_fuel * 1_000_000_000_000;

    // Assume it will take at least 90% of that worst case
    let mut min = max * 9 / 10;

    // The guess at how much ore we'll need. We don't actually guess 0, this value is mutated
    // right after entering the loop.
    let mut candidate = 0u64;

    // Now implement a basic binary search
    while min < max {
        candidate = (max + min) / 2;

        break;
    }
}

#[test]
fn first_example() {
    let input = "
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
    ";

    let reactions = parser::parse_reactions(input);

    // This one is small so test it with djkstra as well
    assert_eq!(dijkstra_solution(&reactions), 31);
    assert_eq!(
        dependency_traversal_solution(&State::new_one_fuel(), &reactions),
        31
    );
}

#[test]
fn second_example() {
    let input = "
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
    ";

    let reactions = parser::parse_reactions(input);

    // This one is larger, so only test it with reverse traversal
    // assert_eq!(dijkstra_solution(&reactions), 165);
    assert_eq!(
        dependency_traversal_solution(&State::new_one_fuel(), &reactions),
        165
    );
}
