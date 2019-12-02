use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read data from file
    let f = BufReader::new(File::open("input.txt").unwrap());
    let masses: Vec<u32> = f
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect();

    // Calcualte total fuel
    let total_fuel = masses.iter().map(fuel).sum::<u32>();

    // Print result
    println!("Total fuel needed: {}", total_fuel);
}

fn more_fuel(mass: &u32) -> u32 {
    let initial = fuel(mass);

    initial + if initial > 0 { more_fuel(&initial) } else { 0 }
}

fn fuel(mass: &u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn no_negative_more_fuel() {
        assert_eq!(more_fuel(&0), 0);
        assert_eq!(more_fuel(&8), 0);
        assert_eq!(more_fuel(&9), 1);
    }

    #[test]
    fn more_fuel12() {
        assert_eq!(more_fuel(&14), 2);
    }

    #[test]
    fn more_fuel100756() {
        assert_eq!(more_fuel(&100756), 50346);
    }

    #[test]
    fn more_fuel1969() {
        assert_eq!(more_fuel(&1969), 966);
    }

    // Any mass that would require negative fuel should instead be treated as if it requires zero fuel
    #[test]
    fn no_negative_fuel() {
        assert_eq!(fuel(&0), 0);
        assert_eq!(fuel(&8), 0);
        assert_eq!(fuel(&9), 1);
    }

    // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    #[test]
    fn fuel12() {
        assert_eq!(fuel(&12), 2)
    }

    // For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    #[test]
    fn fuel14() {
        assert_eq!(fuel(&14), 2)
    }

    // For a mass of 1969, the fuel required is 654.
    #[test]
    fn fuel1969() {
        assert_eq!(fuel(&1969), 654)
    }

    // For a mass of 100756, the fuel required is 33583.
    #[test]
    fn fuel100756() {
        assert_eq!(fuel(&100756), 33583)
    }
}
