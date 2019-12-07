// 224 is too low

use std::fs;
use intcode::Intcode;
use std::collections::VecDeque;

fn main() {
    // Read input file to string
    let s = fs::read_to_string("input.txt").unwrap();

    // Compute Results
    let mut input = VecDeque::new();
    input.push_back(1);
    let mut machine = Intcode::new_with_input(&s, input);
    machine.execute();

    // Print results
    println!("TEST output: {:?}", machine.get_output());
}
