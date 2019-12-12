use std::collections::VecDeque;

// hardcoded lengths of opcodes.
// opcode 0 is not valid
// opcodes 1 and 2 have length four (see day 2)
// opcodes 3 and 4 have length two (see day 5 part 1)
// opcodes 5 and 6 have length 3 (see day 5 part 2)
// opcodes 7 and 8 have length 4 (see day 5 part 2)
const LENGTHS: [usize; 9] = [0, 4, 4, 2, 2, 3, 3, 4, 4];

#[derive(Eq, PartialEq, Debug)]
struct Operation {
    opcode: usize,
    operand_locations: Vec<usize>,
}

#[derive(Eq, PartialEq)]
pub struct Intcode {
    memory: Vec<isize>,
    pointer: usize,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
}

impl Intcode {
    /// Create an new Intcode instance having executed one step
    #[allow(dead_code)]
    pub fn step(&self) -> Self {
        unimplemented!()
    }

    /// Execute this Intcode instance until it halts
    pub fn execute(&mut self) {
        let mut jumped = false;

        // Loop until halt instruction
        while self.memory[self.pointer] != 99 {

            let operation = self.parse_operation();
            // println!("\n\npointer: {:?}", self.pointer);
            // println!("operation: {:?}", operation);
            // println!("prestate: {:?}", self.memory);

            if operation.opcode == 1 {
                // Add instruction
                let op0 = self.memory[operation.operand_locations[0]];
                let op1 = self.memory[operation.operand_locations[1]];
                self.memory[operation.operand_locations[2]] = op0 + op1;

            } else if operation.opcode == 2 {
                // Multiply instruction
                let op0 = self.memory[operation.operand_locations[0]];
                let op1 = self.memory[operation.operand_locations[1]];
                self.memory[operation.operand_locations[2]] = op0 * op1;

            } else if operation.opcode == 3 {
                // Input instruction
                let input_value = self.input.pop_front().unwrap();
                self.memory[operation.operand_locations[0]] = input_value;

            } else if operation.opcode == 4 {
                // Output instruction
                let output_value = self.memory[operation.operand_locations[0]];
                self.output.push_back(output_value);

            } else if operation.opcode == 5 {
                // Jump if true
                let op0 = self.memory[operation.operand_locations[0]];
                let op1 = self.memory[operation.operand_locations[1]];
                if op0 != 0 {
                    self.pointer = op1 as usize;
                    jumped = true;
                }

            } else if operation.opcode == 6 {
                // Jump if false
                let op0 = self.memory[operation.operand_locations[0]];
                let op1 = self.memory[operation.operand_locations[1]];
                if op0 == 0 {
                    self.pointer = op1 as usize;
                    jumped = true;
                }

            } else if operation.opcode == 7 {
                // Less than
                let op0 = self.memory[operation.operand_locations[0]];
                let op1 = self.memory[operation.operand_locations[1]];
                self.memory[operation.operand_locations[2]] = if op0 < op1 {
                    1
                } else {
                    0
                };
            } else if operation.opcode == 8 {
                // Equals
                let op0 = self.memory[operation.operand_locations[0]];
                let op1 = self.memory[operation.operand_locations[1]];
                self.memory[operation.operand_locations[2]] = if op0 == op1 {
                    1
                } else {
                    0
                };
            } else {
                panic!("Invalid opcode: {}", operation.opcode)
            };

            // Adjust the pointer unless a jump instruction occurred
            if !jumped {
                self.pointer += LENGTHS[operation.opcode];
            }
            jumped = false;
        }
    }

    /// Create a new Intcode instance from the given string, and input.
    pub fn new_with_input(s: &str, input: &VecDeque<isize>) -> Self {
        let mut ic = Self::new(s);

        ic.input = input.clone();
        ic
    }

    /// Create a new Intcode instance directly from the given string
    pub fn new(s: &str) -> Self {
        let memory: Vec<_> = s.trim_end()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

        Self {
            memory,
            pointer: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    /// Read an element of memory given an address
    pub fn read(&self, address: usize) -> isize {
        self.memory[address]
    }

    /// Get the output tape from the machine
    pub fn get_output(&self) -> VecDeque<isize> {
        self.output.clone()
    }

    /// Mutates the given memory in the memory tape to the given value
    /// Used specifically for the weird input technique in day 2
    pub fn mutate_memory(&mut self, location: usize, value: isize) {
        self.memory[location] = value;
    }

    /// Render memory as a string
    //TODO
    pub fn memory_string(&self) -> String {
        let mut s = String::new();

        for value in self.memory.iter() {
            s.push_str(&format!("{}", value));
            s.push_str(",");
        }

        s.pop();
        s
    }

    /// arses the operation at the current pointer location
    /// Panics if the value at that cell is not a valid operation
    fn parse_operation(&self) -> Operation {
        let op_digits = self.memory[self.pointer] as usize;
        let opcode: usize = op_digits % 100;
        let mut modes_digits: usize = op_digits / 100;

        // Expected number of operands for this opcode. Knowing this value is
        // necessary because leading zeros may be omitted
        let num_operands = LENGTHS[opcode];

        // Loop through looking up the operands
        let mut operand_locations: Vec<usize> = Vec::new();
        for offset in 1..num_operands {

            if modes_digits % 10 == 1 {
                // Immediate
                operand_locations.push(self.pointer + offset);
            } else {
                // Position
                operand_locations.push(self.memory[self.pointer + offset] as usize);
            }
            modes_digits /= 10;
        }

        Operation {
            opcode,
            operand_locations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_io_1() {
        let mut input = VecDeque::new();
        input.push_back(1);
        let mut machine = Intcode::new_with_input(&"3,0,4,0,99", input);
        machine.execute();
        let mut expected = VecDeque::new();
        expected.push_back(1);
        assert_eq!(machine.get_output(), expected);
    }
}
