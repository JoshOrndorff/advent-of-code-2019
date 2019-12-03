use std::fs;

const TARGET: usize = 19690720;

fn main() {
    // Read input file to string
    let s = fs::read_to_string("input.txt").unwrap();

    // Parse into list of numbers
    let initial_tape = string_to_tape(&s);

    // Compute part 1 result
    let part_1_result = part_1(&mut initial_tape.clone());

    // Compute part 2 result
    let part_2_result = part_2(&mut initial_tape.clone());

    // Print results
    println!("First cell in 1202 tape: {:?}", part_1_result);
    println!("Input that yields {}: {}", TARGET, part_2_result);
}

fn string_to_tape(s: &str) -> Vec<usize> {
    s.trim_end()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part_1(tape: &mut Vec<usize>) -> usize {
    // Make tape 1202 state
    tape[1] = 12;
    tape[2] = 02;

    // Execute the program
    execute(tape);

    // Return value in cell 0
    tape[0]
}

fn part_2(initialized_tape: &Vec<usize>) -> usize {
    // Naive idea: choose a square size and brute force it.
    // Better idea: add a layer to the onion each time.
    // 0 1 4
    // 3 2 5
    // 8 7 6

    let mut active_tape = initialized_tape.clone();
    let mut layer = 0;

    // Search loop through each layer
    loop {

        // Search the top
        for noun in 0..layer {
            active_tape = initialized_tape.clone();
            println!("On top of layer {}. Checking {}{}", layer, noun, layer);
            active_tape[1] = noun;
            active_tape[2] = layer;
            execute(&mut active_tape);
            if active_tape[0] == TARGET {
                return 100 * noun * layer;
            }
        }

        // Search the right
        // TODO I think I'm double-checking the diagonal
        for verb in 0..layer {
            active_tape = initialized_tape.clone();
            println!("On right of layer {}. Checking {}{}", layer, layer, verb);
            active_tape[1] = layer;
            active_tape[2] = verb;
            execute(&mut active_tape);
            if active_tape[0] == TARGET {
                return 100 * layer * verb;
            }

        }

        layer += 1;
    }
}

/// Given a mutable tape, mutates it according to intcode rules
fn execute(tape: &mut Vec<usize>) {
    // Initialize pointer
    let mut pointer = 0;
    // Execution loop
    while tape[pointer] != 99 {

        // println!("executing. pointer: {:?}", pointer);
        // println!("prestate: {:?}", tape);

        let opcode = tape[pointer];
        let operand1 = tape[tape[pointer + 1]];
        let operand2 = tape[tape[pointer + 2]];
        let result_cell = tape[pointer + 3];

        tape[result_cell] = if opcode == 1 {
            // Add instruction
            operand1 + operand2
        } else if opcode == 2 {
            // Multiply instruction
            operand1 * operand2
        } else {
            panic!("Invalid opcode: {}", opcode)
        };

        // println!("poststate: {:?}", tape);

        pointer += 4;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn long_example() {
        let mut experimental = string_to_tape("1,9,10,3,2,3,11,0,99,30,40,50");
        let expected = string_to_tape("3500,9,10,70,2,3,11,0,99,30,40,50");

        execute(&mut experimental);
        assert_eq!(expected, experimental);
    }

    #[test]
    fn ex1_1() {
        let mut experimental = string_to_tape("1,0,0,0,99");
        let expected = string_to_tape("2,0,0,0,99");

        execute(&mut experimental);
        assert_eq!(expected, experimental);
    }

    #[test]
    fn ex1_2() {
        let mut experimental = string_to_tape("2,3,0,3,99");
        let expected = string_to_tape("2,3,0,6,99");

        execute(&mut experimental);
        assert_eq!(expected, experimental);
    }

    #[test]
    fn ex1_3() {
        let mut experimental = string_to_tape("2,4,4,5,99,0");
        let expected = string_to_tape("2,4,4,5,99,9801");

        execute(&mut experimental);
        assert_eq!(expected, experimental);
    }

    #[test]
    fn ex1_4() {
        let mut experimental = string_to_tape("1,1,1,4,99,5,6,0,99");
        let expected = string_to_tape("30,1,1,4,2,5,6,0,99");

        execute(&mut experimental);
        assert_eq!(expected, experimental);
    }

    // #[test]
    // fn ex??() {
    //     let mut experimental = string_to_tape("");
    //     let expected = string_to_tape("");
    //
    //     execute(&mut experimental);
    //     assert_eq!(expected, experimental);
    // }
}
