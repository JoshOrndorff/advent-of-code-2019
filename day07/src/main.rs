use std::collections::VecDeque;
use intcode::Intcode;
use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();

    // Compute Results

    println!("Part 1 result: {}", part1(&s));
}

fn part1(s: &str) -> isize {
    //TODO
    0
}

fn amp_chain(s: &str, phases: [u8; 5]) -> isize {
    let mut buffer: VecDeque<isize> = VecDeque::new();
    buffer.push_back(0);

    for i in 0..5 {
        buffer.push_back(phases[i] as isize);
        println!("{:?}", buffer);

        let mut amp = Intcode::new_with_input(&s, &buffer);
        amp.execute();

        buffer = amp.get_output();
    }
    println!("{:?}", buffer);
    buffer[0]
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_1() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(amp_chain(program, [4,3,2,1,0]), 43210);
    }

    #[test]
    fn ex1_2() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(amp_chain(program, [0,1,2,3,4]), 54321);
    }

    #[test]
    fn ex1_3() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        assert_eq!(amp_chain(program, [1,0,4,3,2]), 65210);
    }
}
