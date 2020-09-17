use super::*;

// Is this how you take a sting slice?
// TODO actually do some parsing
pub fn parse_reactions(_s: &str) -> Vec<Reaction> {
	vec![
		Reaction {
			inputs: vec![
				Reagent {
					name: "ORE".into(),
					quantity: 10,
				},
			],
			outputs: vec![
				Reagent {
					name: "A".into(),
					quantity: 10,
				},
			]
		},
		Reaction {
			inputs: vec![
				Reagent {
					name: "ORE".into(),
					quantity: 1,
				},
			],
			outputs: vec![
				Reagent {
					name: "B".into(),
					quantity: 1,
				},
			]
		},
		Reaction {
			inputs: vec![
				Reagent {
					name: "A".into(),
					quantity: 7,
				},
				Reagent {
					name: "B".into(),
					quantity: 1,
				},
			],
			outputs: vec![
				Reagent {
					name: "C".into(),
					quantity: 1,
				},
			]
		},
		Reaction {
			inputs: vec![
				Reagent {
					name: "A".into(),
					quantity: 7,
				},
				Reagent {
					name: "C".into(),
					quantity: 1,
				},
			],
			outputs: vec![
				Reagent {
					name: "D".into(),
					quantity: 1,
				},
			]
		},
		Reaction {
			inputs: vec![
				Reagent {
					name: "A".into(),
					quantity: 7,
				},
				Reagent {
					name: "D".into(),
					quantity: 1,
				},
			],
			outputs: vec![
				Reagent {
					name: "E".into(),
					quantity: 1,
				},
			]
		},
		Reaction {
			inputs: vec![
				Reagent {
					name: "A".into(),
					quantity: 7,
				},
				Reagent {
					name: "E".into(),
					quantity: 1,
				},
			],
			outputs: vec![
				Reagent {
					name: "FUEL".into(),
					quantity: 1,
				},
			]
		},
		// The special ore collection "reaction"
		Reaction {
			inputs: vec![],
			outputs: vec![
				Reagent {
					name: "ORE".into(),
					quantity: 1,
				},
			]
		}
	]
}
