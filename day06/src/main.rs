use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let timers = get_input();

    let table = LookupTable::init(256);

    let fish_count_80 = simulate_laternfishes(&table, &timers, 80);
    let fish_count_256 = simulate_laternfishes(&table, &timers, 256);

    println!("task 1: laternfishes after 80 days = {}", fish_count_80);
    println!("task 2: laternfishes after 256 days = {}", fish_count_256);
}

fn get_input() -> Vec<u64> {
    let file = File::open("../inputs/day06.txt").expect("failed to open input file");
    let mut line = String::new();
    BufReader::new(file)
        .read_line(&mut line)
        .expect("failed to read line");

    // Remove newline
    line.pop();

    return line
        .split(",")
        .map(|s| s.parse().expect("failed to parse input"))
        .collect();
}

fn simulate_laternfishes(table: &LookupTable, timers: &Vec<u64>, days: u64) -> u64 {
    return timers.iter().map(|&t| table.get(t, days)).sum();
}

#[test]
fn test_simulate_laternfishes() {
    let table = LookupTable::init(256);
    let timers = vec![3, 4, 3, 1, 2];
    assert_eq!(simulate_laternfishes(&table, &timers, 18), 26);
    assert_eq!(simulate_laternfishes(&table, &timers, 80), 5934);
    assert_eq!(simulate_laternfishes(&table, &timers, 256), 26984457539);
}

struct LookupTable {
    table: Vec<u64>,
}

impl LookupTable {
    fn init(days: u64) -> LookupTable {
        let mut table = vec![0u64; (days as usize + 1) * 9];

        fn get(table: &Vec<u64>, timer: u64, days: u64) -> u64 {
            return table[(days * 9 + timer) as usize];
        }

        for timer in 0..9 {
            table[timer] = 1;
        }

        for day in 1..days + 1 {
            for timer in 0..9 {
                if timer == 0 {
                    table[(day * 9 + timer) as usize] =
                        get(&table, 6, day - 1) + get(&table, 8, day - 1);
                } else {
                    table[(day * 9 + timer) as usize] = get(&table, timer - 1, day - 1)
                }
            }
        }

        return LookupTable { table };
    }

    fn get(&self, timer: u64, days: u64) -> u64 {
        return self.table[(days * 9 + timer) as usize];
    }
}
