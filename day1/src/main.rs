fn main() {
    println!("Hello, world!");
}

fn fuel(mass: u32) -> u32 {
    (mass / 3) - 2
}


#[cfg(test)]
mod test {
    use crate::*;
    // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    #[test]
    fn mass12() {
        assert_eq!(fuel(12), 2)
    }

    // For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    #[test]
    fn mass14() {
        assert_eq!(fuel(14), 2)
    }

    // For a mass of 1969, the fuel required is 654.
    #[test]
    fn mass1969() {
        assert_eq!(fuel(1969), 654)
    }

    // For a mass of 100756, the fuel required is 33583.
    #[test]
    fn mass100756() {
        assert_eq!(fuel(100756), 33583)
    }
}
