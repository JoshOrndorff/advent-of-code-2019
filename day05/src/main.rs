use std::fs;
use intcode::Intcode;
use std::collections::VecDeque;

fn main() {
    // Read input file to string
    let s = fs::read_to_string("input.txt").unwrap();

    // Compute Results
    let mut input1 = VecDeque::new();
    input1.push_back(1);
    let mut machine1 = Intcode::new_with_input(&s, input1);
    machine1.execute();

    let mut input2 = VecDeque::new();
    input2.push_back(5);
    let mut machine2 = Intcode::new_with_input(&s, input2);
    machine2.execute();

    // Print results
    println!("TEST output: {:?}", machine1.get_output());
    println!("TEST output: {:?}", machine2.get_output());
}
