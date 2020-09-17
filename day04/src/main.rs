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
    let mut count1 = 0;
    let mut count2 = 0;
    for num in lower..=upper {
        let digits = get_digits(num);
        if part_1_checks(&digits) {
            count1 += 1;
        }
        if part_2_checks(&digits) {
            count2 += 1;
        }
    }

    // Print result
    println!("valid part 1 passwords: {}", count1);
    println!("valid part 2 passwords: {}", count2);
}

/// Given an integer, returns a vector of digits
fn get_digits(num: usize) -> Vec<usize> {
    let mut copy = num;
    let mut digits: Vec<usize> = Vec::new();
    while copy > 0 {
        digits.push(copy % 10);
        copy /= 10;
    }
    digits.reverse();
    digits
}

fn part_1_checks(digits: &Vec<usize>) -> bool {
    six_digits(&digits) && repeated_digit(&digits) && monotonic(&digits)
}

fn part_2_checks(digits: &Vec<usize>) -> bool {
    six_digits(&digits) && specific_repeated_digit(&digits) && monotonic(&digits)
}

fn specific_repeated_digit(digits: &Vec<usize>) -> bool {
    // With only six digits, it seems as wise to hard code all possibilities
    (digits[0] == digits[1] && digits[1] != digits[2])
        || (digits[0] != digits[1] && digits[1] == digits[2] && digits[2] != digits[3])
        || (digits[1] != digits[2] && digits[2] == digits[3] && digits[3] != digits[4])
        || (digits[2] != digits[3] && digits[3] == digits[4] && digits[4] != digits[5])
        || (digits[3] != digits[4] && digits[4] == digits[5])
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
        assert!(part_1_checks(&get_digits(111111)));
    }

    #[test]
    fn ex1_2() {
        assert!(!part_1_checks(&get_digits(223450)));
    }

    #[test]
    fn ex1_3() {
        assert!(!part_1_checks(&get_digits(123789)));
    }

    #[test]
    fn ex2_1() {
        assert!(part_1_checks(&get_digits(111111)));
    }

    #[test]
    fn ex2_2() {
        assert!(!part_1_checks(&get_digits(223450)));
    }

    #[test]
    fn ex2_3() {
        assert!(part_1_checks(&get_digits(123789)));
    }
}
