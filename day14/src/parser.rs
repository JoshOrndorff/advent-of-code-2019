use super::*;
use std::str::FromStr;

pub fn parse_reactions(s: &str) -> Vec<Reaction> {
    s.trim().split('\n').map(parse_reaction).collect()
}

// Is this how you take a sting slice?
fn parse_reaction(s: &str) -> Reaction {
    let arrow_index = s.find("=>").expect("Arrow symbol is in every live");
    let input_str = &s[..arrow_index].trim();
    let output_str = &s[(arrow_index + 2)..].trim();

    Reaction {
        inputs: parse_reagents(input_str),
        outputs: parse_reagents(output_str),
    }
}

fn parse_reagents(s: &str) -> Vec<Reagent> {
    s.split(",").map(|s| s.trim()).map(parse_reagent).collect()
}

fn parse_reagent(s: &str) -> Reagent {
    let space_index = s
        .find(" ")
        .expect("every reagent has a space between the quantity and the name");
    let quantity_str = s[..space_index].trim();
    let name_str = s[space_index..].trim();

    Reagent {
        name: name_str.into(),
        quantity: FromStr::from_str(quantity_str).unwrap(),
    }
}

#[test]
fn first_example_parses() {
    let input = "\
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    let expected = vec![
        Reaction {
            inputs: vec![Reagent {
                name: "ORE".into(),
                quantity: 10,
            }],
            outputs: vec![Reagent {
                name: "A".into(),
                quantity: 10,
            }],
        },
        Reaction {
            inputs: vec![Reagent {
                name: "ORE".into(),
                quantity: 1,
            }],
            outputs: vec![Reagent {
                name: "B".into(),
                quantity: 1,
            }],
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
            outputs: vec![Reagent {
                name: "C".into(),
                quantity: 1,
            }],
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
            outputs: vec![Reagent {
                name: "D".into(),
                quantity: 1,
            }],
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
            outputs: vec![Reagent {
                name: "E".into(),
                quantity: 1,
            }],
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
            outputs: vec![Reagent {
                name: "FUEL".into(),
                quantity: 1,
            }],
        },
    ];
    assert_eq!(parse_reactions(input), expected);
}
