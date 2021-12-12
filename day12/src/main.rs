use std::collections::{HashMap, HashSet};

fn main() {
    let cave_system = get_input();

    let path_count_1 = find_paths(&cave_system, false);
    let path_count_2 = find_paths(&cave_system, true);

    println!("task 1: number of paths = {}", path_count_1);
    println!(
        "task 2: number of paths (allowing small caves twice) = {}",
        path_count_2
    );
}

type CaveSystem = HashMap<String, Vec<String>>;

fn find_paths(system: &CaveSystem, allow_small_twice: bool) -> usize {
    let mut paths_found = 0;
    let mut initial = HashSet::new();
    initial.insert("start");
    let mut paths_todo = vec![(false, "start", initial)];
    let empty = Vec::new();

    while paths_todo.len() > 0 {
        let (has_small_twice, current, path) = paths_todo.pop().unwrap();
        for next in system.get(current).unwrap_or(&empty) {
            if next == "end" {
                // Reached the end, increment path counter
                paths_found += 1;
            } else if next.chars().next().unwrap().is_ascii_lowercase() {
                // Next could be a small cave
                if !path.contains(&next.as_str()) {
                    // Next has not been visited yet
                    let mut new_path = path.clone();
                    new_path.insert(next);
                    paths_todo.push((has_small_twice, next, new_path));
                } else if next != "start" && allow_small_twice && !has_small_twice {
                    // Next has been visited already, but no other small cave was visited twice yet
                    paths_todo.push((true, next, path.clone()));
                }
            } else {
                // Next is a big cave
                let mut new_path = path.clone();
                new_path.insert(next);
                paths_todo.push((has_small_twice, next, new_path));
            }
        }
    }

    return paths_found;
}

fn get_input() -> CaveSystem {
    let lines = include_str!("../../inputs/day12.txt").trim().split("\n");
    let mut map = HashMap::new();

    for line in lines {
        if let [from, to] = &line.split("-").collect::<Vec<&str>>()[..] {
            map.entry(from.to_string())
                .or_insert_with(|| Vec::new())
                .push(to.to_string());
            map.entry(to.to_string())
                .or_insert_with(|| Vec::new())
                .push(from.to_string());
        } else {
            panic!("invalid input line");
        }
    }

    map
}
