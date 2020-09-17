use intcode::Intcode;
use std::fs;

const TARGET: isize = 19690720;

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

fn intcode_with_custom_inputs(s: &str, noun: isize, verb: isize) -> Intcode {
    let mut ic = Intcode::new(s);
    ic.mutate_memory(1, noun);
    ic.mutate_memory(2, verb);
    ic
}

fn part_1(s: &str) -> isize {
    // New intcode instance with given input and code 1202
    let mut tape = intcode_with_custom_inputs(s, 12, 02);

    // Execute the program
    tape.execute();

    // Return value in cell 0
    tape.read(0)
}

fn part_2(s: &str) -> isize {
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
            active_tape = intcode_with_custom_inputs(&s, noun, layer);
            active_tape.execute();
            if active_tape.read(0) == TARGET {
                return 100 * noun + layer;
            }
        }

        // Search the right
        // TODO I think I'm double-checking the diagonal
        for verb in 0..layer {
            active_tape = intcode_with_custom_inputs(&s, layer, verb);
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
