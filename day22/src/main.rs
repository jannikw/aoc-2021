#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    let toggles = get_input();
    let cores_on = count_on_after_initialization(&toggles);

    println!("task 1: number of cores on = {}", cores_on);
}



fn count_on_after_initialization(toggles: &Vec<CubeRangeToggle>) -> usize {
    let mut cores = Cube::new(CubeRange::new(-50, 50, -50, 50, -50, 50));

    for CubeRangeToggle(state, range) in toggles {
        cores.set_range(*state, &range);
    }

    cores.count_on()
}

struct Cube {
    dim: CubeRange,
    states: Vec<bool>,
}

impl Cube {
    fn new(dim: CubeRange) -> Self {
        let states = vec![
            false;
            (((dim.x_end - dim.x_start).abs() + 1)
                * ((dim.y_end - dim.y_start).abs() + 1)
                * ((dim.z_end - dim.z_start).abs() + 1)) as usize
        ];
        Cube { states, dim }
    }

    fn set(&mut self, state: bool, x: i64, y: i64, z: i64) {
        if x < self.dim.x_start
            || x > self.dim.x_end
            || y < self.dim.y_start
            || y > self.dim.y_end
            || z < self.dim.z_start
            || z > self.dim.z_end
        {
            return;
        }

        let xy_size = (((self.dim.x_end - self.dim.x_start).abs() + 1)
            * ((self.dim.y_end - self.dim.y_start).abs() + 1)) as usize;
        let x_size = ((self.dim.x_end - self.dim.x_start).abs() + 1) as usize;
        let z_off = (z - self.dim.z_start) as usize;
        let y_off = (y - self.dim.y_start) as usize;
        let x_off = (x - self.dim.x_start) as usize;
        self.states[z_off * xy_size + y_off * x_size + x_off] = state;
    }

    fn count_on(&self) -> usize {
        self.states.iter().filter(|s| **s).count()
    }

    fn set_range(&mut self, state: bool, range: &CubeRange) {
        for z in range.z_start.max(self.dim.z_start)..range.z_end.min(self.dim.z_end) + 1 {
            for y in range.y_start.max(self.dim.y_start)..range.y_end.min(self.dim.y_end) + 1 {
                for x in range.x_start.max(self.dim.x_start)..range.x_end.min(self.dim.x_end) + 1 {
                    self.set(state, x, y, z);
                }
            }
        }
    }
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
}

struct CubeRange {
    x_start: i64,
    x_end: i64,
    y_start: i64,
    y_end: i64,
    z_start: i64,
    z_end: i64,
}

impl CubeRange {
    fn new(x_start: i64, x_end: i64, y_start: i64, y_end: i64, z_start: i64, z_end: i64) -> Self {
        CubeRange {
            x_start,
            x_end,
            y_start,
            y_end,
            z_start,
            z_end,
        }
    }
}

struct CubeRangeToggle(bool, CubeRange);

impl CubeRangeToggle {
    fn parse(text: &str) -> Option<Self> {
        let captures = REGEX.captures(text)?;
        let toggle = captures.get(1)?.as_str() == "on";
        let x_start = captures.get(2)?.as_str().parse().ok()?;
        let x_end = captures.get(3)?.as_str().parse().ok()?;
        let y_start = captures.get(4)?.as_str().parse().ok()?;
        let y_end = captures.get(5)?.as_str().parse().ok()?;
        let z_start = captures.get(6)?.as_str().parse().ok()?;
        let z_end = captures.get(7)?.as_str().parse().ok()?;

        Some(CubeRangeToggle(
            toggle,
            CubeRange {
                x_start,
                x_end,
                y_start,
                y_end,
                z_start,
                z_end,
            },
        ))
    }
}

fn get_input() -> Vec<CubeRangeToggle> {
    include_str!("../../inputs/day22.txt")
        .trim()
        .split("\n")
        .map(|l| CubeRangeToggle::parse(l).unwrap())
        .collect()
}

#[test]
fn test_small_example() {
    let toggles: Vec<CubeRangeToggle> = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"
        .trim()
        .split("\n")
        .map(|l| CubeRangeToggle::parse(l).unwrap())
        .collect();
    assert_eq!(count_on_after_initialization(&toggles), 39);
}

#[test]
fn test_larger_example() {
    let toggles: Vec<CubeRangeToggle> = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"
        .trim()
        .split("\n")
        .map(|l| CubeRangeToggle::parse(l).unwrap())
        .collect();
    assert_eq!(count_on_after_initialization(&toggles), 590784);
}
