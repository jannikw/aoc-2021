use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("../inputs/day01.txt").expect("input not found");
    let lines = BufReader::new(file).lines();

    // Part 1
    let mut prev_depth: Option<i32> = None;
    let mut increased_count = 0;

    // Part 2
    let mut prev_depths: [i32; 3] = [0; 3];
    let mut increased_count_sw = 0;

    for (i, line) in lines.enumerate() {
        let depth: i32 = line
            .expect("failed to read line")
            .parse()
            .expect("failed to parse line");

        // Part 1
        match prev_depth {
            Some(d) if d < depth => increased_count += 1,
            _ => (),
        }
        prev_depth = Some(depth);

        // Part 2
        if i >= 3 {
            let prev_sw: i32 = prev_depths.iter().sum();
            let next_sw = prev_depths[(i - 2) % 3] + prev_depths[(i - 1) % 3] + depth;
            if next_sw > prev_sw {
                increased_count_sw += 1;
            }
        }
        prev_depths[i % 3] = depth
    }

    println!("part 1: depth increased {} times", increased_count);
    println!("part 2: depth increased {} times", increased_count_sw);
}
