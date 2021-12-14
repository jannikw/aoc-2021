use std::collections::{BinaryHeap, HashSet};

fn main() {
    println!("Hello, world!");
    let (dots, folds) = get_input();

    let dots_after_first_fold = execute_fold(&dots, folds[0]);
    let dots_after_final_fold = folds.iter().fold(dots, |ds, &f| execute_fold(&ds, f));

    println!(
        "task 1: dots after first fold = {}",
        dots_after_first_fold.len()
    );
    println!("task 2: eight capital letters");
    print_dots(&dots_after_final_fold);
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Dot(i32, i32);

impl Ord for Dot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.1, other.0).cmp(&(self.1, self.0))
    }
}

impl PartialOrd for Dot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    AlongY(i32),
    AlongX(i32),
}

fn execute_fold(dots: &HashSet<Dot>, fold: Fold) -> HashSet<Dot> {
    let mut new_dots = HashSet::new();
    match fold {
        Fold::AlongX(fx) => {
            for &Dot(x, y) in dots {
                assert_ne!(x, fx);
                if x < fx {
                    new_dots.insert(Dot(x, y));
                } else {
                    new_dots.insert(Dot(fx + (fx - x), y));
                }
            }
        }
        Fold::AlongY(fy) => {
            for &Dot(x, y) in dots {
                assert_ne!(y, fy);
                if y < fy {
                    new_dots.insert(Dot(x, y));
                } else {
                    new_dots.insert(Dot(x, fy + (fy - y)));
                }
            }
        }
    }

    return new_dots;
}

fn print_dots(dots: &HashSet<Dot>) {
    let mut sorted: BinaryHeap<_> = dots.iter().collect();
    let mut last_y = 0;
    let mut last_x = 0;
    while let Some(&Dot(x, y)) = sorted.pop() {
        while y > last_y {
            println!("");
            last_y += 1;
            last_x = 0;
        }
        while x - 1 > last_x {
            print!(" ");
            last_x += 1;
        }
        print!("#");
        last_x += 1;
    }
    println!();
}

fn get_input() -> (HashSet<Dot>, Vec<Fold>) {
    use regex::Regex;
    let re = Regex::new(r"^(\d+),(\d+)|fold along x=(\d+)|fold along y=(\d+)$").unwrap();

    let mut dots = HashSet::new();
    let mut folds = Vec::new();

    for line in include_str!("../../inputs/day13.txt").trim().split("\n") {
        if let Some(captures) = re.captures(line) {
            if captures.get(1).is_some() {
                let x = captures.get(1).unwrap().as_str().parse().unwrap();
                let y = captures.get(2).unwrap().as_str().parse().unwrap();
                dots.insert(Dot(x, y));
            } else if let Some(x) = captures.get(3).and_then(|s| s.as_str().parse().ok()) {
                folds.push(Fold::AlongX(x));
            } else if let Some(y) = captures.get(4).and_then(|s| s.as_str().parse().ok()) {
                folds.push(Fold::AlongY(y));
            }
        } else {
            assert_eq!(line, "");
        }
    }

    return (dots, folds);
}
