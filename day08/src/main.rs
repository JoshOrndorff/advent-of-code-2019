use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;


fn main() {
    // Read input
    // TODO get it from file
    let raw_data = fs::read_to_string("input.txt").unwrap();

    // Compute
    // let part_1_answer = part1(3, 2, "123456789012");
    let part_1_answer = part1(WIDTH, HEIGHT, &raw_data.trim_end());

    // Print results
    println!("part 1 result: {}", part_1_answer);
}

fn part1(width: usize, height: usize, data: &str) -> usize {
    // Parse into 3D grid
    let mut grid: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut chars = data.chars();

    let mut next_char: Option<char>;

    'outer: loop {
        let mut layer: Vec<Vec<u8>> = Vec::new();
        for _ in 0..height {
            let mut row: Vec<u8> = Vec::new();
            for _ in 0..width {
                next_char = chars.next();
                match next_char {
                    // Hack ascii code to get u8s
                    Some(c) => { row.push(c as u8 - 48); },
                    None => {break 'outer;},
                }
            }
            layer.push(row);
        }
        grid.push(layer);
    }

    // Find layer with fewest zeros
    let few_zero_layer = grid.iter().min_by_key(|l| count_digits(&l, 0)).unwrap();

    // Number of 1 digits multiplied by number of 2 digits
    count_digits(few_zero_layer, 1) * count_digits(few_zero_layer, 2)
}

fn count_digits(layer: &Vec<Vec<u8>>, target: u8) -> usize {
    let mut count = 0;
    for row in layer {
        for data in row {
            if data == &target {
                count += 1;
            }
        }
    }
    count
}
