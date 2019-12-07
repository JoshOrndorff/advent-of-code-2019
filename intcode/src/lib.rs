
#[derive(Eq, PartialEq)]
pub struct Intcode {
    memory: Vec<isize>,
    pointer: usize,
    halted: bool,
    input: Vec<isize>,
    output: Vec<isize>,
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
            let operand1 = self.memory[self.memory[self.pointer + 1] as usize];
            let operand2 = self.memory[self.memory[self.pointer + 2] as usize];
            let result_cell = self.memory[self.pointer + 3];

            self.memory[result_cell as usize] = if opcode == 1 {
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
    // pub fn new_with_input(s: &str, noun: usize, verb: usize) -> Self {
    //     let mut ic = Self::new(s);
    //
    //     ic.memory[1] = noun;
    //     ic.memory[2] = verb;
    //     ic
    // }

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
            input: vec![],
            output: vec![],
        }
    }

    /// Read an element of memory given an address
    pub fn read(&self, address: usize) -> isize {
        self.memory[address]
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
