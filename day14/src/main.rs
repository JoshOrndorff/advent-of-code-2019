use std::collections::{BTreeMap, HashSet};
mod parser;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Reagent {
    //TODO should I be using &str everywhere?
    name: String,
    quantity: u64,
}

//todo better name
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct State {
    reagents: BTreeMap<String, u64>,
    ore_consumed: u64,
}

impl State {
    fn new() -> Self {
        Self {
            reagents: BTreeMap::new(),
            ore_consumed: 0,
        }
    }

    fn total_ore(&self) -> u64 {
        match self.reagents.get("ORE".into()) {
            Some(ore) => *ore,
            None => 0,
        }
    }

    // fn get_prev_states(&self, reactions: &Vec<Reaction>) -> Vec<Self> {
    // 	let mut neighbors = Vec::new();
    // 	'reaction: for reaction in reactions {
    // 		// Make a clone that we will mutate into the neighbor state
    // 		let mut neighbor = self.clone();
    //
    // 		// Loop through the outputs subtracting them
    // 		for output in &reaction.outputs {
    // 			// If we don't have enough, then go on to the next reaction
    // 			match neighbor.reagents.get(&output.name) {
    // 				None => { continue 'reaction; },
    // 				Some(&amount_we_have) => {
    // 					if amount_we_have < output.quantity {
    // 						continue 'reaction;
    // 					}
    // 					neighbor.reagents.insert(output.name.clone(), amount_we_have - output.quantity);
    // 				}
    // 			}
    // 		}
    //
    // 		// If we made it throug hthe outputs without early returning, then this reaction is possible
    // 		// so update the input quantities too
    // 		for input in &reaction.inputs {
    // 			let maybe_amount = neighbor.reagents.get(&input.name);
    // 			match maybe_amount {
    // 				None => { neighbor.reagents.insert(input.name.clone(), input.quantity); },
    // 				Some(&amount_we_have) => {
    // 					neighbor.reagents.insert(input.name.clone(), input.quantity + amount_we_have);
    // 				}
    // 			}
    // 		}
    //
    // 		neighbors.push(neighbor);
    // 	}
    // 	neighbors
    // }

    fn get_next_state(&self, reaction: &Reaction) -> Option<Self> {
        // Make a clone that we will mutate into the neighbor state
        let mut neighbor = self.clone();

        // Loop through the inputs subtracting them
        for input in &reaction.inputs {
            // If we don't have enough, then exit early
            match neighbor.reagents.get(&input.name) {
                None => {
                    return None;
                }
                Some(&amount_we_have) => {
                    if amount_we_have < input.quantity {
                        return None;
                    }
                    neighbor
                        .reagents
                        .insert(input.name.clone(), amount_we_have - input.quantity);
                }
            }
        }

        // If we made it through the inputs without early returning, then this reaction is possible
        // so update the output quantities too
        for output in &reaction.outputs {
            match neighbor.reagents.get(&output.name) {
                None => {
                    neighbor
                        .reagents
                        .insert(output.name.clone(), output.quantity);
                }
                Some(&amount_we_have) => {
                    neighbor
                        .reagents
                        .insert(output.name.clone(), output.quantity + amount_we_have);
                }
            }
        }

        Some(neighbor)
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
    //TODO only one output, so no need to loop
    outputs: Vec<Reagent>,
}

fn dijkstra_solution(reactions: &Vec<Reaction>) -> u64 {
    // We'll solve the problem using Dijkstra's algorithm to find a path from no resources
    // to one fules
    let mut unexplored = HashSet::<State>::new();
    let mut explored = HashSet::<State>::new();

    // We'll find a path from the
    // starting state (empty) to any valid target state (at least one fuel).
    unexplored.insert(State::new());

    let mut current_state = State::new();

    while !current_state.has_fuel() {
        println!(
            "\n\nIn main dijkstra loop with {} unexplored states",
            unexplored.len()
        );

        // Get an owned instance of the next state to explore
        current_state = unexplored
            .iter()
            .min_by(|x, y| x.ore_consumed.cmp(&y.ore_consumed))
            .expect("Unexplored set should not be empty; min_by will return Some(_); qed;")
            .clone();

        println!("Current state: {:?}", current_state);

        // Calculate any states we can transition to by applying a reaction
        let neighbors = reactions
            .iter()
            .filter_map(|r| current_state.get_next_state(r))
            .collect::<Vec<_>>();
        println!("Found {} neighbors", neighbors.len());

        // Mark the current state explored
        explored.insert(unexplored.take(&current_state).unwrap());

        // We also want to explore gathering more ore because some reactions
        // need more than 1 ore.
        unexplored.insert(current_state.gather_ore());

        // Mark each neighbor as unexplored
        for neighbor in neighbors {
            println!("{:?}", neighbor);
            unexplored.insert(neighbor);
        }
    }

    std::thread::sleep(std::time::Duration::from_millis(500));

    current_state.ore_consumed
}

fn main() {
    // Parse the puzzle input, which represents a set of reactions
    let reactions = parser::parse_reactions(
        "
		9 ORE => 2 A
		8 ORE => 3 B
		7 ORE => 5 C
		3 A, 4 B => 1 AB
		5 B, 7 C => 1 BC
		4 C, 1 A => 1 CA
		2 AB, 3 BC, 4 CA => 1 FUEL
	",
    );

    let part_1 = dijkstra_solution(&reactions);

    println!("Minimal ORE needed: {}", part_1);
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

    assert_eq!(dijkstra_solution(&reactions), 31);
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

    assert_eq!(dijkstra_solution(&reactions), 165);
}
