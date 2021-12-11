use std::collections::HashSet;
use std::fmt;

fn main() {
    let mut world = get_input();
    let flashes = simulate_flashes(&mut world, 100);
    let synced = simulate_until_synchronized(&mut world) + 100;

    println!("task 1: flashed after 100 steps = {}", flashes);
    println!("task 2: flashes are synchronized after {} steps", synced);
}

fn simulate_flashes(world: &mut World, n: i32) -> i32 {
    let mut sum = 0;
    for _ in 0..n {
        sum += world.step();
    }
    sum
}

fn simulate_until_synchronized(world: &mut World) -> i32 {
    let mut steps = 0;
    loop {
        steps += 1;
        let flashes = world.step();
        if flashes == world.width * world.height {
            return steps;
        }
    }
}

struct World {
    cells: Vec<u32>,
    width: i32,
    height: i32,
}

impl World {
    fn parse(text: &str) -> World {
        let lines = text.trim().split("\n");
        let mut height = 0;
        let mut width = None;
        let mut cells = Vec::new();

        for line in lines {
            let mut line_cells = line
                .chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect::<Vec<_>>();
            cells.append(&mut line_cells);
            assert_eq!(width.get_or_insert(line.len()), &line.len());
            height += 1;
        }

        World {
            width: width.unwrap_or(0) as i32,
            height,
            cells,
        }
    }

    fn get(&self, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            0
        } else {
            self.cells[(y * self.width + x) as usize]
        }
    }

    fn inc(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.cells[(y * self.width + x) as usize] += 1;
        }
    }

    fn inc_all(&mut self) {
        for v in &mut self.cells {
            *v += 1;
        }
    }

    fn reset(&mut self) {
        for v in &mut self.cells {
            if *v > 9 {
                *v = 0;
            }
        }
    }

    fn step(&mut self) -> i32 {
        let mut flashes = 0;
        let mut has_flashed = HashSet::new();
        let mut to_flash = Vec::new();

        self.inc_all();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) > 9 {
                    to_flash.push((x, y));
                }
            }
        }

        while to_flash.len() > 0 {
            let (x, y) = to_flash.pop().unwrap();
            if !has_flashed.contains(&(x, y)) {
                for (dx, dy) in [
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, -1),
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                ]
                .iter()
                {
                    let xx = x + dx;
                    let yy = y + dy;
                    self.inc(xx, yy);

                    if self.get(xx, yy) > 9 {
                        to_flash.push((xx, yy));
                    }
                }

                flashes += 1;
                has_flashed.insert((x, y));
            }
        }

        self.reset();
        flashes
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn get_input() -> World {
    World::parse(include_str!("../../inputs/day11.txt"))
}

#[test]
fn test_simulate_steps() {
    let mut world = World::parse(
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
    );
    assert_eq!(simulate_flashes(&mut world, 10), 204);
}
