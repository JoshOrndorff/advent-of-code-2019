// 600 (just a guess) is too low
// 579 is too low, but is correct for someone else.

use std::fs;

fn main() {
    // Read data from file
    let bounds = fs::read_to_string("input.txt")
        .unwrap()
        .split("-")
        .map(|s| s.trim_end().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let lower = bounds[0];
    let upper = bounds[1];
    println!("Final bounds: {}-{}", lower, upper);

    // Search Loop
    let mut count = 0;
    for i in lower..=upper {
        if passes_checks(i) {
            count += 1;
        }
    }

    // Print result
    println!("count: {}", count);
}

fn passes_checks(candidate: usize) -> bool {
    let mut copy = candidate;
    let mut digits: Vec<usize> = Vec::new();
    while copy > 0 {
        digits.push(copy % 10);
        copy /= 10;
    }

    six_digits(&digits) && repeated_digit(&digits) && monotonic(&digits)
}

fn six_digits(digits: &Vec<usize>) -> bool {
    digits.len() == 6
}

fn repeated_digit(digits: &Vec<usize>) -> bool {
    // Bound is correct. Panics with out of range when removing -1
    for i in 0..digits.len() - 1 {
        if digits[i] == digits[i + 1] {
            return true;
        }
    }

    false
}

fn monotonic(digits: &Vec<usize>) -> bool {
    let mut prev = 0;
    for digit in digits {
        if digit >= &prev {
            prev = *digit;
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1_1() {
        assert!(passes_checks(111111));
    }

    #[test]
    fn ex1_2() {
        assert!(!passes_checks(223450));
    }

    #[test]
    fn ex1_3() {
        assert!(!passes_checks(123789));
    }
}
