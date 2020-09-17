use std::collections::{HashSet, BTreeMap};
mod parser;

#[cfg(test)]
mod tests;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Reagent{
	name: String,
	quantity: u64,
}

//todo better name
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct State {
	reagents: BTreeMap<String, u64>,
}

impl State {
	fn new() -> Self {
		Self { reagents: BTreeMap::new() }
	}

	fn is_all_ore(&self) -> bool {
		self.reagents.keys().filter(|name| name != &"ORE").count() == 0
	}

	fn total_ore(&self) -> u64 {
		match self.reagents.get("ORE".into()) {
			Some(ore) => *ore,
			None => 0,
		}
	}

	fn get_prev_states(&self, reactions: &Vec<Reaction>) -> Vec<Self> {
		let mut neighbors = Vec::new();
		'reaction: for reaction in reactions {
			// Make a clone that we will mutate into the neighbor state
			let mut neighbor = self.clone();

			// Loop through the outputs subtracting them
			for output in &reaction.outputs {
				// If we don't have enough, then go on to the next reaction
				match neighbor.reagents.get(&output.name) {
					None => { continue 'reaction; },
					Some(&amount_we_have) => {
						if amount_we_have < output.quantity {
							continue 'reaction;
						}
						neighbor.reagents.insert(output.name.clone(), amount_we_have - output.quantity);
					}
				}
			}

			// If we made it throug hthe outputs without early returning, then this reaction is possible
			// so update the input quantities too
			for input in &reaction.inputs {
				let maybe_amount = neighbor.reagents.get(&input.name);
				match maybe_amount {
					None => { neighbor.reagents.insert(input.name.clone(), input.quantity); },
					Some(&amount_we_have) => {
						neighbor.reagents.insert(input.name.clone(), input.quantity + amount_we_have);
					}
				}
			}

			neighbors.push(neighbor);
		}
		neighbors
	}

	fn get_next_states(&self, reactions: &Vec<Reaction>) -> Vec<Self> {
		let mut neighbors = Vec::new();
		'reaction: for reaction in reactions {
			// Make a clone that we will mutate into the neighbor state
			let mut neighbor = self.clone();

			// Loop through the inputs subtracting them
			for input in &reaction.inputs {
				// If we don't have enough, then go on to the next reaction
				match neighbor.reagents.get(&input.name) {
					None => { continue 'reaction; },
					Some(&amount_we_have) => {
						if amount_we_have < input.quantity {
							continue 'reaction;
						}
						neighbor.reagents.insert(input.name.clone(), amount_we_have - input.quantity);
					}
				}
			}

			// If we made it through the inputs without early returning, then this reaction is possible
			// so update the output quantities too
			for output in &reaction.outputs {
				match neighbor.reagents.get(&output.name) {
					None => { neighbor.reagents.insert(output.name.clone(), output.quantity); },
					Some(&amount_we_have) => {
						neighbor.reagents.insert(output.name.clone(), output.quantity + amount_we_have);
					}
				}
			}

			neighbors.push(neighbor);
		}
		neighbors
	}

	fn has_fuel(&self) -> bool {
		match self.reagents.get("FUEL".into()) {
			None => false,
			Some(amount) => amount > &0,
		}
	}

	fn add(&mut self, name: String, quantity: u64) {
		self.reagents.insert(name, quantity);
	}
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct Reaction {
	inputs: Vec<Reagent>,
	outputs: Vec<Reagent>,
}

fn main() {
	// Parse the puzzle input, which represents a set of reactions
	let reactions = parser::parse_reactions("dummy");

	// We'll solve the problem using Dijkstra's algorithm to find a path from no resources
	// to one fules
	let mut unexplored = HashSet::<State>::new();
	let mut explored = HashSet::<State>::new();

	// We'll find a path from the
	// starting state (empty) to any valid target state (at least one fuel).
	let mut start_state = State::new();
	unexplored.insert(start_state);

	let mut current_state = State::new();

	while !current_state.has_fuel() {
		println!("\n\nIn main dijkstra loop");

		// Get a reference to the next state to explore
		current_state = unexplored.iter().min_by(|x, y| x.total_ore().cmp(&y.total_ore())).expect("Fuck, the unexplored set must have been empty. That means no path.").clone();
		println!("Current state: {:?}", current_state);


		let neighbors = current_state.get_next_states(&reactions);
		println!("Found {} neighbors", neighbors.len());

		for neighbor in neighbors {
			println!("{:?}", neighbor);
			unexplored.insert(neighbor);
		}

		// Mark the current state explored
		explored.insert(unexplored.take(&current_state).unwrap());

		//TODO This is temp to only go through one iteration of Dijkstra's
		// break;
	}

}
