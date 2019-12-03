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
        .map(|l| l.unwrap())
        .collect();

    // Calcualte
    let shortest = part1(&paths[0], &paths[1]);

    // Print result
    println!("smallest distance: {:?}", shortest);
}

fn part1(path1: &str, path2: &str) -> usize {
    let points1 = path_to_points(&parse_path(path1));
    let points2 = path_to_points(&parse_path(path2));
    let intersection_points = points1.intersection(&points2);

    intersection_points.map(manhattan_distance).min().unwrap()
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

    #[test]
    fn long_example() {
        assert_eq!(part1("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
    }

    #[test]
    fn ex1_1() {
        assert_eq!(part1("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"), 159);
    }

    #[test]
    fn ex1_2() {
        assert_eq!(part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
    }
}
