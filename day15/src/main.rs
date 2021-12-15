use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

fn main() {
    let map = get_input();
    let goal = (map.width - 1, map.height - 1);
    let result = find_path(&map, goal).unwrap();
    println!("task 1: found path with total risk = {}", result);

    let ex_map = map.expand(5);
    let new_goal = (ex_map.width - 1, ex_map.height - 1);
    let ex_result = find_path(&ex_map, new_goal).unwrap();
    println!(
        "task 2: found path in expanded map with total risk = {}",
        ex_result
    );
}

fn find_path(map: &Map, goal: (i32, i32)) -> Option<i32> {
    // A* algorithm for finding the optimal path
    let mut open_list = PriorityQueue::new();
    let mut closed_list = HashSet::new();
    let mut came_from = HashMap::new();
    let (gx, gy) = goal;

    open_list.push((0, 0), Reverse(0));
    came_from.insert((0, 0), 0);

    while let Some(((x, y), _)) = open_list.pop() {
        if (x, y) == goal {
            return Some(*came_from.get(&goal).unwrap());
        }
        let &cost = came_from.get(&(x, y)).unwrap();

        closed_list.insert((x, y));

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let xx = x + dx;
            let yy = y + dy;
            if !closed_list.contains(&(xx, yy)) {
                if let Some(risk) = map.get(xx, yy) {
                    // Calculate new cost with heuristic for remaining path to goal
                    let new_cost = cost + risk + i32::abs(gx - xx) + i32::abs(gy - yy);
                    if let Some(&Reverse(old_cost)) = open_list.get_priority(&(xx, yy)) {
                        if old_cost > new_cost {
                            open_list.change_priority(&(xx, yy), Reverse(new_cost));
                            *came_from.get_mut(&(xx, yy)).unwrap() = cost + risk;
                        }
                    } else {
                        open_list.push((xx, yy), Reverse(new_cost));
                        came_from.insert((xx, yy), cost + risk);
                    }
                }
            }
        }
    }

    return None;
}

struct Map {
    width: i32,
    height: i32,
    cells: Vec<i32>,
}

impl Map {
    fn get(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            Some(self.cells[(y * self.width + x) as usize])
        }
    }

    fn parse(text: &str) -> Map {
        let lines = text.trim().split("\n");
        let mut height = 0;
        let mut width = None;
        let mut cells = Vec::new();

        for line in lines {
            let mut line_cells = line
                .chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>();
            cells.append(&mut line_cells);
            assert_eq!(width.get_or_insert(line.len()), &line.len());
            height += 1;
        }

        Map {
            width: width.unwrap_or(0) as i32,
            height,
            cells,
        }
    }

    fn expand(&self, factor: i32) -> Map {
        let new_width = self.width * factor;
        let new_height = self.height * factor;
        let mut cells = vec![0; (new_width * new_height) as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                for i in 0..factor {
                    for h in 0..factor {
                        cells[((y + h * self.height) * new_width + x + self.width * i) as usize] =
                            (self.cells[(y * self.width + x) as usize] - 1 + (i + h)) % 9 + 1;
                    }
                }
            }
        }

        Map {
            width: new_width,
            height: new_height,
            cells,
        }
    }
}

fn get_input() -> Map {
    Map::parse(include_str!("../../inputs/day15.txt"))
}

#[test]
fn test_find_path() {
    let map = Map::parse(
        "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
    );
    let goal = (map.width - 1, map.height - 1);
    let result = find_path(&map, goal);
    assert_eq!(result, Some(40));
}

#[test]
fn test_find_path_expanded() {
    let map = Map::parse(
        "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
    )
    .expand(5);
    let goal = (map.width - 1, map.height - 1);
    let result = find_path(&map, goal);
    assert_eq!(result, Some(315));
}
