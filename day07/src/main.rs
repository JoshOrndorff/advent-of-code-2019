use std::collections::{VecDeque, HashSet};
use intcode::Intcode;
use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();

    // Find all possible orderings
    let remaining: HashSet<u8> = vec![0, 1, 2, 3, 4].iter().map(|x| *x as u8).collect();
    let mut orderings = Vec::new();
    get_all_orderings(&Vec::new(), &remaining, &mut orderings);
    // println!("{:?}", orderings);

    // Try all orderings
    let max_thrust = orderings.iter().map(|o| amp_chain(&s, o)).max().unwrap();

    println!("Part 1 result: {}", max_thrust);
}

fn get_all_orderings(used: &Vec<u8>, remaining: &HashSet<u8>, mut results: &mut Vec<Vec<u8>>) {

    if remaining.len() == 0 {
        results.push(used.clone());
    }

    for item in remaining {
        let mut new_used = used.clone();
        new_used.push(*item);
        let mut new_remaining = remaining.clone();
        new_remaining.remove(item);
        get_all_orderings(&new_used, &new_remaining, &mut results);
    }
}

fn part1(s: &str) -> isize {

    //TODO
    1
}

fn amp_chain(s: &str, phases: &Vec<u8>) -> isize {
    let mut buffer: VecDeque<isize> = VecDeque::new();
    buffer.push_back(0);

    for i in 0..5 {
        buffer.push_front(phases[i] as isize);

        let mut amp = Intcode::new_with_input(&s, &buffer);
        amp.execute();

        buffer = amp.get_output();
    }
    buffer[0]
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_1() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(amp_chain(program, &vec![4,3,2,1,0]), 43210);
    }

    #[test]
    fn ex1_2() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(amp_chain(program, &vec![0,1,2,3,4]), 54321);
    }

    #[test]
    fn ex1_3() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        assert_eq!(amp_chain(program, &vec![1,0,4,3,2]), 65210);
    }
}
