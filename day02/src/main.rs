use std::fs;

const TARGET: usize = 19690720;

#[derive(Eq, PartialEq)]
struct Intcode {
    memory: Vec<usize>,
    pointer: usize,
    halted: bool,
}

impl Intcode {
    /// Create an new Intcode instance having executed one step
    #[allow(dead_code)]
    fn step(&self) -> Self {
        unimplemented!()
    }

    /// Execute this Intcode instance until it halts
    fn execute(&mut self) {

        // Loop until halt instruction
        while self.memory[self.pointer] != 99 {

            // println!("executing. pointer: {:?}", pointer);
            // println!("prestate: {:?}", tape);

            let opcode = self.memory[self.pointer];
            let operand1 = self.memory[self.memory[self.pointer + 1]];
            let operand2 = self.memory[self.memory[self.pointer + 2]];
            let result_cell = self.memory[self.pointer + 3];

            self.memory[result_cell] = if opcode == 1 {
                // Add instruction
                operand1 + operand2
            } else if opcode == 2 {
                // Multiply instruction
                operand1 * operand2
            } else {
                panic!("Invalid opcode: {}", opcode)
            };

            // println!("poststate: {:?}", self.memory);

            self.pointer += 4;
        }

        self.halted = true;
    }

    /// Create a new Intcode instance from the given string, and input noun and verb.
    fn new_with_input(s: &str, noun: usize, verb: usize) -> Self {

        let mut memory: Vec<usize> = s.trim_end()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

        memory[1] = noun;
        memory[2] = verb;

        Self {
            memory,
            pointer: 0,
            halted: false,
        }
    }

    /// Create a new Intcode instance directly from the given string
    fn new(s: &str) -> Self {
        //TODO too much duplicated code in `new` methods
        let memory: Vec<usize> = s.trim_end()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

        Self {
            memory,
            pointer: 0,
            halted: false,
        }
    }

    /// Read an element of memory given an address
    fn read(&self, address: usize) -> usize {
        self.memory[address]
    }

    /// Render memory as a string
    //TODO
    fn memory_string(&self) -> String {
        let mut s = String::new();

        for value in self.memory.iter() {
            s.push_str(&format!("{}", value));
            s.push_str(",");
        }

        s.pop();
        s
    }
}

fn main() {
    // Read input file to string
    let s = fs::read_to_string("input.txt").unwrap();

    // Compute Results
    let part_1_result = part_1(&s);
    let part_2_result = part_2(&s);

    // Print results
    println!("First cell in 1202 tape: {:?}", part_1_result);
    println!("Input that yields {}: {}", TARGET, part_2_result);
}

fn part_1(s: &str) -> usize {
    // New intcode instance with given input and code 1202
    let mut tape = Intcode::new_with_input(s, 12, 02);

    // Execute the program
    tape.execute();

    // Return value in cell 0
    tape.read(0)
}

fn part_2(s: &str) -> usize {
    // Search Algorithm: add a layer to the onion each time.
    // 0 1 4
    // 3 2 5
    // 8 7 6

    let mut active_tape: Intcode;
    let mut layer = 0;

    // Search loop through each layer
    loop {

        // Search the top
        for noun in 0..layer {
            active_tape = Intcode::new_with_input(&s, noun, layer);
            active_tape.execute();
            if active_tape.read(0) == TARGET {
                return 100 * noun + layer;
            }
        }

        // Search the right
        // TODO I think I'm double-checking the diagonal
        for verb in 0..layer {
            active_tape = Intcode::new_with_input(&s, layer, verb);
            active_tape.execute();
            if active_tape.read(0) == TARGET {
                return 100 * layer + verb;
            }

        }

        layer += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn long_example() {
        let mut experimental = Intcode::new("1,9,10,3,2,3,11,0,99,30,40,50");
        let expected = "3500,9,10,70,2,3,11,0,99,30,40,50";

        experimental.execute();
        assert_eq!(expected, experimental.memory_string());
    }

    #[test]
    fn ex1_1() {
        let mut experimental = Intcode::new("1,0,0,0,99");
        let expected = "2,0,0,0,99";

        experimental.execute();
        assert_eq!(expected, experimental.memory_string());
    }

    #[test]
    fn ex1_2() {
        let mut experimental = Intcode::new("2,3,0,3,99");
        let expected = "2,3,0,6,99";

        experimental.execute();
        assert_eq!(expected, experimental.memory_string());
    }

    #[test]
    fn ex1_3() {
        let mut experimental = Intcode::new("2,4,4,5,99,0");
        let expected = "2,4,4,5,99,9801";

        experimental.execute();
        assert_eq!(expected, experimental.memory_string());
    }

    #[test]
    fn ex1_4() {
        let mut experimental = Intcode::new("1,1,1,4,99,5,6,0,99");
        let expected = "30,1,1,4,2,5,6,0,99";

        experimental.execute();
        assert_eq!(expected, experimental.memory_string());
    }
}
