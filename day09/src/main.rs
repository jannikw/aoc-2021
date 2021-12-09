fn main() {
    let map = get_input();
    let sum = find_sum_of_risk_levels(&map);
    let largest_basins = find_three_largest_basin_sizes(&map);

    println!("task 1: sum of risk levels = {}", sum);
    println!("task 2: sum of three largest basins = {}", largest_basins);
}

fn find_sum_of_risk_levels(map: &Map) -> i32 {
    return find_low_points(map)
        .iter()
        .map(|&(x, y)| map.get(x, y).unwrap() as i32 + 1)
        .sum();
}

fn find_three_largest_basin_sizes(map: &Map) -> i32 {
    use std::collections::BinaryHeap;

    let mut sizes: BinaryHeap<_> = find_low_points(map)
        .iter()
        .map(|&(x, y)| get_basin_size(map, x, y))
        .collect();

    // return sizes.into_iter_sorted().take(3).sum();
    return sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap();
}

fn find_low_points(map: &Map) -> Vec<(i32, i32)> {
    let mut low_points = Vec::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let height = map.get(x, y).unwrap();
            if height < map.get(x - 1, y).unwrap_or(u8::MAX)
                && height < map.get(x + 1, y).unwrap_or(u8::MAX)
                && height < map.get(x, y - 1).unwrap_or(u8::MAX)
                && height < map.get(x, y + 1).unwrap_or(u8::MAX)
            {
                low_points.push((x, y));
            }
        }
    }

    return low_points;
}

fn get_basin_size(map: &Map, x: i32, y: i32) -> i32 {
    use std::collections::HashSet;

    let mut size = 0;
    let mut checked = HashSet::new();
    let mut to_check = vec![(x, y)];
    while to_check.len() > 0 {
        let (x, y) = to_check.pop().unwrap();
        if map.get(x, y).unwrap_or(u8::MAX) < 9 && !checked.contains(&(x, y)) {
            size += 1;
            checked.insert((x, y));
            to_check.push((x + 1, y));
            to_check.push((x - 1, y));
            to_check.push((x, y + 1));
            to_check.push((x, y - 1));
        }
    }

    return size;
}

struct Map {
    width: i32,
    height: i32,
    cells: Vec<u8>,
}

impl Map {
    fn get(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            Some(self.cells[(y * self.width + x) as usize])
        }
    }
}

fn get_input() -> Map {
    let lines = include_str!("../../inputs/day09.txt").trim().split("\n");
    let mut height = 0;
    let mut width = None;
    let mut cells = Vec::new();

    for line in lines {
        let mut line_cells = line
            .chars()
            .into_iter()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
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
