use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let wires_steps: Vec<HashMap<Point, usize>> = f
        .lines()
        .map(|l| l.unwrap())
        .map(|x| parse_path(&x))
        .map(|x| path_to_steps(&x))
        .collect();

    let wires_points: Vec<HashSet<Point>> = wires_steps
        .iter()
        .map(|steps| steps.keys().map(|x| *x).collect::<HashSet<Point>>())
        .collect();
    let intersection_points: HashSet<Point> = wires_points[0]
        .intersection(&wires_points[1])
        .map(|x| *x)
        .collect();

    // Print result
    println!("smallest distance: {}", part1(&intersection_points));
    println!(
        "fewest combined steps: {}",
        part2(&intersection_points, &wires_steps)
    );
}

fn part2(points: &HashSet<Point>, step_maps: &Vec<HashMap<Point, usize>>) -> usize {
    points
        .iter()
        .map(|p| step_maps[0].get(&p).unwrap() + step_maps[1].get(&p).unwrap())
        .min()
        .unwrap()
}

/// Solve part 1 by calculating the Manhattan distance for each point in the set
/// and finding the shortest
fn part1(points: &HashSet<Point>) -> usize {
    points.iter().map(manhattan_distance).min().unwrap()
}

//TODO Maybe nom would be better for this stuff
fn parse_path(s: &str) -> Vec<Segment> {
    s.split(",").map(parse_segment).collect()
}

fn parse_segment(s: &str) -> Segment {
    let letter = s.chars().nth(0).unwrap();
    let direction = if letter == 'U' {
        Direction::U
    } else if letter == 'D' {
        Direction::D
    } else if letter == 'L' {
        Direction::L
    } else if letter == 'R' {
        Direction::R
    } else {
        panic!("Unrecognized direction")
    };

    let length = s.chars().skip(1).collect::<String>().parse().unwrap();

    Segment { direction, length }
}

/// Takes in a path reference and returns a HashMap mapping
/// each point traversed to the number of steps it took to get there.
fn path_to_steps(segments: &Vec<Segment>) -> HashMap<Point, usize> {
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut steps = 1;
    let mut points = HashMap::new();

    for segment in segments.iter() {
        for _ in 0..segment.length {
            match segment.direction {
                Direction::U => {
                    cur_y += 1;
                }
                Direction::D => {
                    cur_y -= 1;
                }
                Direction::L => {
                    cur_x -= 1;
                }
                Direction::R => {
                    cur_x += 1;
                }
            }
            points.insert(Point(cur_x, cur_y), steps);
            steps += 1;
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
        assert_eq!(
            part1(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
    }

    #[test]
    fn ex1_2() {
        assert_eq!(
            part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }
}
