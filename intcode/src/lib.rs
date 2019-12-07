use std::collections::VecDeque;

// hardcoded lengths of opcodes.
// opcodes 1 and 2 have length four (see day 2)
// opcodes 3 and 4 have length two (see day 5)
const lengths: [usize; 5] = [0, 4, 4, 2, 2];

#[derive(Eq, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Eq, PartialEq)]
struct Operation {
    opcode: usize,
    modes: Vec<Mode>,
}

#[derive(Eq, PartialEq)]
pub struct Intcode {
    memory: Vec<isize>,
    pointer: usize,
    halted: bool,
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

        // Loop until halt instruction
        while self.memory[self.pointer] != 99 {

            // println!("executing. pointer: {:?}", pointer);
            // println!("prestate: {:?}", tape);

            let opcode = self.memory[self.pointer];
            let operation = parse_operation(opcode);

            if operation.opcode == 1 {
                // Add instruction
                let operand1 = self.memory[
                    match operation.modes[0] {
                        Mode::Position => self.memory[self.pointer + 1] as usize,
                        Mode::Immediate => self.pointer + 1,
                    }
                ];
                let operand2 = self.memory[
                    match operation.modes[1] {
                        Mode::Position => self.memory[self.pointer + 2] as usize,
                        Mode::Immediate => self.pointer + 2,
                    }
                ];
                // I'm not even looking at the mode here. It only makes sense
                // for the result cell to be location mode
                let result_cell = self.memory[self.pointer + 3] as usize;

                self.memory[result_cell] = operand1 + operand2;

            } else if operation.opcode == 2 {
                // Multiply instruction
                //TODO lots of duplicated code with addition
                let operand1 = self.memory[
                    match operation.modes[0] {
                        Mode::Position => self.memory[self.pointer + 1] as usize,
                        Mode::Immediate => self.pointer + 1,
                    }
                ];
                let operand2 = self.memory[
                    match operation.modes[1] {
                        Mode::Position => self.memory[self.pointer + 2] as usize,
                        Mode::Immediate => self.pointer + 2,
                    }
                ];
                // I'm not even looking at the mode here. It only makes sense
                // for the result cell to be location mode
                let result_cell = self.memory[self.pointer + 3] as usize;

                self.memory[result_cell] = operand1 * operand2;

            } else if operation.opcode == 3 {
                // Input instruction
                let input_value = self.input.pop_front().unwrap();
                // I'm not even looking at the mode here. It only makes sense
                // for the result cell to be location mode
                let result_cell = self.memory[self.pointer + 1] as usize;
                self.memory[result_cell] = input_value;
            } else if operation.opcode == 4 {
                // Output instruction
                // I'm not even looking at the mode here. It does make sense to
                // output an immediate, but the instructions don't describe that happening
                let output_value = self.memory[self.pointer + 1];
                self.output.push_back(output_value);
            } else {
                panic!("Invalid opcode: {}", opcode)
            };

            // println!("poststate: {:?}", self.memory);

            self.pointer += lengths[operation.opcode];
        }

        self.halted = true;
    }

    /// Create a new Intcode instance from the given string, and input.
    pub fn new_with_input(s: &str, input: VecDeque<isize>) -> Self {
        let mut ic = Self::new(s);

        ic.input = input;
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
            halted: false,
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
}

/// Given a modes string and an expected number of modes, returns a vector of Mode variants
fn get_modes(digits: usize, num:  usize) -> Vec<Mode> {
    let mut modes = Vec::new();
    let mut modes_digits = digits;
    for _ in 1..num {

        if modes_digits % 10 == 1 {
            modes.push(Mode::Immediate);
        } else {
            modes.push(Mode::Position);
        }
        modes_digits /= 10;
    }
    modes
}


fn parse_operation(n: isize) -> Operation {
    let opcode: usize = n as usize % 100;
    let modes_digits: usize = n as usize / 100;

    let modes = get_modes(modes_digits, lengths[opcode]);

    Operation {
        opcode,
        modes,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
