fn main() {
    let start_pos = Pos(0, 0);
    let target_area = TargetArea {
        x_start: 144,
        x_end: 178,
        y_start: -100,
        y_end: -76,
    };
    let (hits, highpoint) = find_highpoint(&start_pos, &target_area);

    println!("task 1: highpoint is {}", highpoint.unwrap());
    println!("task 2: number of initial velocities = {}", hits);
}

struct TargetArea {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
}

#[derive(Copy, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn apply_velocity(&self, Velocity(vx, vy): &Velocity) -> Self {
        let Pos(x, y) = self;
        Pos(x + vx, y + vy)
    }

    fn is_in(&self, target: &TargetArea) -> bool {
        let &Pos(x, y) = self;
        target.x_start <= x && target.x_end >= x && target.y_start <= y && target.y_end >= y
    }
}

#[derive(Copy, Clone)]
struct Velocity(i32, i32);

impl Velocity {
    fn step(&self) -> Self {
        let &Velocity(vx, vy) = self;
        if vx == 0 {
            Velocity(vx, vy - 1)
        } else if vx > 0 {
            Velocity(vx - 1, vy - 1)
        } else {
            Velocity(vx + 1, vy - 1)
        }
    }
}

fn get_highpoint(
    start_velocity: &Velocity,
    start_pos: &Pos,
    target_area: &TargetArea,
) -> Option<i32> {
    let mut pos = *start_pos;
    let mut velocity = *start_velocity;
    let Pos(_, mut highpoint) = start_pos;

    loop {
        let Pos(x, y) = pos.apply_velocity(&velocity);
        let Velocity(vx, vy) = velocity.step();

        pos = Pos(x, y);
        velocity = Velocity(vx, vy);

        highpoint = i32::max(highpoint, y);

        if pos.is_in(target_area) {
            return Some(highpoint);
        }

        if vx == 0 && (x < target_area.x_start || x > target_area.x_end) {
            return None;
        }

        if vy < 0 && y < target_area.y_start {
            return None;
        }
    }
}

fn find_highpoint(start_pos: &Pos, target_area: &TargetArea) -> (usize, Option<i32>) {
    let mut highpoints = Vec::new();

    for vx in 0..target_area.x_end + 100 {
        for vy in target_area.y_start - 100..target_area.y_end + 300 {
            let velocity = Velocity(vx, vy);
            if let Some(highpoint) = get_highpoint(&velocity, start_pos, target_area) {
                highpoints.push(highpoint);
            }
        }
    }

    return (highpoints.len(), highpoints.iter().copied().max());
}

#[test]
fn test_example() {
    let (hits, high_point) = find_highpoint(
        &Pos(0, 0),
        &TargetArea {
            x_start: 20,
            x_end: 30,
            y_start: -10,
            y_end: -5,
        },
    );
    assert_eq!(hits, 112);
    assert_eq!(high_point, Some(45));
}
