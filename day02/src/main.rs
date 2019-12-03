use std::fs;

fn main() {
    // Read input file to string
    let s = fs::read_to_string("input.txt").unwrap();

    // Parse into list of numbers
    let mut tape1 = string_to_tape(&s);
    let mut tape2 = tape1.clone();
    println!("Tape 1: {:?}", tape1);
    println!("Tape 2: {:?}", tape2);

    // Compute part 1 result
    let part_1_result = part_1(&mut tape1);

    //TODO Compute part 2 result

    // Print results
    println!("{:?}", part_1_result);
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

    println!("Debug: part 1 starting tape: {:?}", tape);

    // Execute the program
    execute(tape);

    // Return value in cell 0
    tape[0]
}

/// Given a mutable tape, mutates it according to intcode rules
fn execute(tape: &mut Vec<usize>) {
    // Initialize pointer
    let mut pointer = 0;
    // Execution loop
    while tape[pointer] != 99 {

        println!("executing. pointer: {:?}", pointer);
        println!("prestate: {:?}", tape);

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

        println!("poststate: {:?}", tape);

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
}
