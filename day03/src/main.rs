use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point(isize, isize);

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Eq, PartialEq)]
struct Segment {
    direction: Direction,
    length: usize,
}

fn main() {
    // Read data from file
    let f = BufReader::new(File::open("input.txt").unwrap());
    let paths: Vec<_> = f
        .lines()
        .map(|l| parse_path(&l.unwrap()))
        .collect();

    // Calcualte
    let points1 = path_to_points(&paths[0]);
    let points2 = path_to_points(&paths[1]);
    let intersection_points = points1.intersection(&points2);

    let shortest = intersection_points.map(manhattan_distance).min();

    // Print result
    println!("smallest distance: {:?}", shortest);
}

//TODO Maybe nom would be better for this stuff
fn parse_path(s: &str) -> Vec<Segment> {
    s.split(",").map(parse_segment).collect()
}

fn parse_segment(s: &str) -> Segment {
    let letter = s.chars().nth(0).unwrap();
    let direction = if letter == 'U' { Direction::U }
               else if letter == 'D' { Direction::D }
               else if letter == 'L' { Direction::L }
               else if letter == 'R' { Direction::R }
               else {panic!("Unrecognized direction")};

    let length = s.chars().skip(1).collect::<String>().parse().unwrap();

    Segment{
        direction,
        length,
    }
}

fn path_to_points(segments: &Vec<Segment>) -> HashSet<Point> {
    let mut cur = Point(0,0);
    let mut points = HashSet::new();

    for segment in segments.iter() {
        for _ in 0..segment.length {
            match segment.direction {
                Direction::U => {
                    cur = Point(cur.0, cur.1 + 1);
                },
                Direction::D => {
                    cur = Point(cur.0, cur.1 - 1);
                },
                Direction::L => {
                    cur = Point(cur.0 - 1, cur.1);
                },
                Direction::R => {
                    cur = Point(cur.0 + 1, cur.1);
                },
            }
            points.insert(cur.clone());
        }
    }

    points
}

fn manhattan_distance(p: &Point) -> usize {
    (p.0.abs() + p.1.abs()).try_into().unwrap()
}

#[cfg(test)]
mod test {
    use crate::*;

    // #[test]
    // fn no_negative_more_fuel() {
    //     assert_eq!(more_fuel(&0), 0);
    //     assert_eq!(more_fuel(&8), 0);
    //     assert_eq!(more_fuel(&9), 1);
    // }
}
