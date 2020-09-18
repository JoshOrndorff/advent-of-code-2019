use super::{Reaction, Reagent};
use std::collections::{BTreeMap, HashSet};
use std::convert::TryInto;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct State {
    reagents: BTreeMap<String, i64>,
    ore_consumed: u64,
}

impl State {
    fn new() -> Self {
        Self {
            reagents: BTreeMap::new(),
            ore_consumed: 0,
        }
    }

    fn new_one_fuel() -> Self {
        let mut s = Self {
            reagents: BTreeMap::new(),
            ore_consumed: 0,
        };
        s.reagents.insert("FUEL".into(), 1);
        s
    }

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
        // so update the output quantity too
        *neighbor
            .reagents
            .entry(reaction.output.name.clone())
            .or_insert(0) += reaction.output.quantity;

        Some(neighbor)
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

pub fn dijkstra_solution(reactions: &Vec<Reaction>) -> u64 {
    // We'll solve the problem using Dijkstra's algorithm to find a path from no resources
    // to one fules
    let mut unexplored = HashSet::<State>::new();
    let mut explored = HashSet::<State>::new();

    // We'll find a path from the
    // starting state (empty) to any valid target state (at least one fuel).
    unexplored.insert(State::new());

    let mut current_state = State::new();

    while !current_state.has_fuel() {
        // Get an owned instance of the next state to explore
        current_state = unexplored
            .iter()
            .min_by(|x, y| x.ore_consumed.cmp(&y.ore_consumed))
            .expect("Unexplored set should not be empty; min_by will return Some(_); qed;")
            .clone();

        // Calculate any states we can transition to by applying a reaction
        let neighbors = reactions
            .iter()
            .filter_map(|r| current_state.get_next_state(r))
            .collect::<Vec<_>>();

        // Mark the current state explored
        explored.insert(unexplored.take(&current_state).unwrap());

        // We also want to explore gathering more ore because some reactions
        // need more than 1 ore.
        unexplored.insert(current_state.gather_ore());

        // Mark each neighbor as unexplored
        for neighbor in neighbors {
            unexplored.insert(neighbor);
        }
    }

    current_state.ore_consumed
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

    // This one is small so Djkstra can actually solve it
    assert_eq!(dijkstra_solution(&reactions), 31);
}
