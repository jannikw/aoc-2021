use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let steps = get_input();
    let position_part_1 = run_part_1(&steps);
    let position_part_2 = run_part_2(&steps);

    println!(
        "part 1: submarine moved to {} x {} = {}",
        position_part_1.depth,
        position_part_1.horizontal,
        position_part_1.depth * position_part_1.horizontal
    );
    println!(
        "part 2: submarine moved to {} x {} = {}",
        position_part_2.depth,
        position_part_2.horizontal,
        position_part_2.depth * position_part_1.horizontal
    );
}

enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug, PartialEq)]
struct Position {
    depth: i32,
    horizontal: i32,
}

fn get_input() -> Vec<(Direction, i32)> {
    let file = File::open("../inputs/day02.txt").expect("input not found");
    let lines = BufReader::new(file).lines();
    let re = Regex::new(r"^(forward|up|down) (\d+)").unwrap();

    lines
        .map(|line| line.expect("failed to read line"))
        .map(|line| -> (Direction, i32) {
            let captures = re.captures(&line).expect("failed to parse line");
            let value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

            let direction = match captures.get(1).unwrap().as_str() {
                "up" => Direction::Up,
                "down" => Direction::Down,
                "forward" => Direction::Forward,
                _ => panic!("invalid direction"),
            };

            return (direction, value);
        })
        .collect()
}

fn run_part_1(steps: &Vec<(Direction, i32)>) -> Position {
    let mut depth = 0;
    let mut horizontal = 0;

    for (direction, value) in steps {
        match direction {
            Direction::Up => depth -= value,
            Direction::Down => depth += value,
            Direction::Forward => horizontal += value,
        }
    }

    return Position { depth, horizontal };
}

#[test]
fn test_part_1() {
    let position = run_part_1(&vec![
        (Direction::Forward, 5),
        (Direction::Down, 5),
        (Direction::Forward, 8),
        (Direction::Up, 3),
        (Direction::Down, 8),
        (Direction::Forward, 2),
    ]);
    assert_eq!(
        position,
        Position {
            depth: 10,
            horizontal: 15,
        }
    );
}

fn run_part_2(steps: &Vec<(Direction, i32)>) -> Position {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    for (direction, value) in steps {
        match direction {
            Direction::Up => aim -= value,
            Direction::Down => aim += value,
            Direction::Forward => {
                horizontal += value;
                depth += aim * value;
            }
        }
    }

    return Position { depth, horizontal };
}

#[test]
fn test_part_2() {
    let position = run_part_2(&vec![
        (Direction::Forward, 5),
        (Direction::Down, 5),
        (Direction::Forward, 8),
        (Direction::Up, 3),
        (Direction::Down, 8),
        (Direction::Forward, 2),
    ]);
    assert_eq!(
        position,
        Position {
            depth: 60,
            horizontal: 15,
        }
    );
}