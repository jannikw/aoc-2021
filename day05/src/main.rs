use regex::Regex;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = get_input();
    let overlaps = determine_overlaps(&lines, false);
    let overlaps_diagonals = determine_overlaps(&lines, true);

    println!("task 1: number of overlaps = {}", overlaps);
    println!(
        "task 1: number of overlaps including diagonals = {}",
        overlaps_diagonals
    );
}

type Point = (i32, i32);

struct Line {
    from: Point,
    to: Point,
}

fn get_input() -> Vec<Line> {
    let file = File::open("../inputs/day05.txt").expect("failed to open input file");
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("failed to create regex");
    let mut lines = Vec::new();

    for line in BufReader::new(file)
        .lines()
        .map(|l| l.expect("failed to read line"))
    {
        let captures = re.captures(&line).expect("failed to parse line");
        let from = (
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
        );
        let to = (
            captures.get(3).unwrap().as_str().parse().unwrap(),
            captures.get(4).unwrap().as_str().parse().unwrap(),
        );
        lines.push(Line { from, to });
    }

    return lines;
}

fn determine_overlaps(lines: &Vec<Line>, include_diagonal: bool) -> usize {
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        let (x1, y1) = line.from;
        let (x2, y2) = line.to;

        max_x = max(max_x, max(x1, x2));
        max_y = max(max_y, max(y1, y2));
    }

    let mut world: Vec<i32> = vec![0; ((max_x + 1) * (max_y + 1)) as usize];

    for line in lines {
        let (x1, y1) = line.from;
        let (x2, y2) = line.to;

        if x1 == x2 {
            let start = min(y1, y2);
            let end = max(y1, y2);
            for y in start..end + 1 {
                world[(y * max_y + x1) as usize] += 1;
            }
        } else if y1 == y2 {
            let start = min(x1, x2);
            let end = max(x1, x2);
            for x in start..end + 1 {
                world[(y1 * max_y + x) as usize] += 1;
            }
        } else if include_diagonal {
            if x1 < x2 {
                let mut y = y1;
                for x in x1..x2 + 1 {
                    world[(y * max_y + x) as usize] += 1;
                    if y1 < y2 {
                        y += 1
                    } else {
                        y -= 1
                    }
                }
            } else {
                let mut y = y2;
                for x in x2..x1 + 1 {
                    world[(y * max_y + x) as usize] += 1;
                    if y2 < y1 {
                        y += 1
                    } else {
                        y -= 1
                    }
                }
            }
        }
    }

    return world.iter().filter(|&&n| n >= 2).count();
}

#[test]
fn test_determine_overlaps() {
    let overlaps = determine_overlaps(
        &vec![
            Line {
                from: (0, 9),
                to: (5, 9),
            },
            Line {
                from: (8, 0),
                to: (0, 8),
            },
            Line {
                from: (9, 4),
                to: (3, 4),
            },
            Line {
                from: (2, 2),
                to: (2, 1),
            },
            Line {
                from: (7, 0),
                to: (7, 4),
            },
            Line {
                from: (6, 4),
                to: (2, 0),
            },
            Line {
                from: (0, 9),
                to: (2, 9),
            },
            Line {
                from: (3, 4),
                to: (1, 4),
            },
            Line {
                from: (0, 0),
                to: (8, 8),
            },
            Line {
                from: (5, 5),
                to: (8, 2),
            },
        ],
        false,
    );
    assert_eq!(overlaps, 5);
}

#[test]
fn test_determine_overlaps_including_diagonals() {
    let overlaps = determine_overlaps(
        &vec![
            Line {
                from: (0, 9),
                to: (5, 9),
            },
            Line {
                from: (8, 0),
                to: (0, 8),
            },
            Line {
                from: (9, 4),
                to: (3, 4),
            },
            Line {
                from: (2, 2),
                to: (2, 1),
            },
            Line {
                from: (7, 0),
                to: (7, 4),
            },
            Line {
                from: (6, 4),
                to: (2, 0),
            },
            Line {
                from: (0, 9),
                to: (2, 9),
            },
            Line {
                from: (3, 4),
                to: (1, 4),
            },
            Line {
                from: (0, 0),
                to: (8, 8),
            },
            Line {
                from: (5, 5),
                to: (8, 2),
            },
        ],
        true,
    );
    assert_eq!(overlaps, 12);
}
