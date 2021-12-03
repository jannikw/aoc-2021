use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let report = get_input();

    let result_task_1 = calc_power_consumption(&report);
    let (oxygen_generator_rating, c02_scrubber_rating) = calc_life_support_rating(&report);
    let life_support_rating = oxygen_generator_rating * c02_scrubber_rating;

    println!(
        "task 1: gamma = {}, epsilon = {} -> {}",
        result_task_1.gamma,
        result_task_1.epsilon,
        result_task_1.gamma * result_task_1.epsilon
    );

    println!("task 2: life support rating = {}", life_support_rating);
}

fn get_input() -> Vec<u32> {
    let file = File::open("../inputs/day03.txt").expect("failed to open input file");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("failed to read line"))
        .map(|s| u32::from_str_radix(&s, 2).expect("failed to parse number"))
        .collect()
}

#[derive(Debug, PartialEq)]
struct Result {
    epsilon: u32,
    gamma: u32,
}

fn calc_power_consumption(report: &Vec<u32>) -> Result {
    let mut zero_bits = [0; 32];
    let mut one_bits = [0; 32];

    for value in report {
        for i in 0..32 {
            let check_bit = value & 1 << i;
            if check_bit > 0 {
                one_bits[i] += 1;
            } else {
                zero_bits[i] += 1;
            }
        }
    }

    let mut epsilon = 0u32;
    let mut gamma = 0u32;

    for i in 0..32 {
        // Assume that one bit is never all zeroes/all ones
        if one_bits[i] > 0 && zero_bits[i] > 0 {
            if one_bits[i] > zero_bits[i] {
                gamma = gamma | 1 << i;
            }
            if one_bits[i] < zero_bits[i] {
                epsilon = epsilon | 1 << i;
            }
        }
    }

    return Result { epsilon, gamma };
}

fn calc_life_support_rating(report: &Vec<u32>) -> (u32, u32) {
    let mut values_1 = report.to_owned();
    let mut values_2 = report.to_owned();

    for i in (0..32).rev() {
        if values_1.len() > 1 {
            let (ones, zeroes): (Vec<u32>, Vec<u32>) =
                values_1.iter().partition(|&v| (v & 1 << i) > 0);
            if zeroes.len() > ones.len() {
                values_1 = zeroes;
            } else {
                values_1 = ones;
            }
        }

        if values_2.len() > 1 {
            let (ones, zeroes): (Vec<u32>, Vec<u32>) =
                values_2.iter().partition(|&v| (v & 1 << i) > 0);
            if ones.len() > 0 && ones.len() < zeroes.len() {
                values_2 = ones;
            } else {
                values_2 = zeroes;
            }
        }
    }

    assert_eq!(values_1.len(), 1);
    assert_eq!(values_2.len(), 1);

    return (values_1[0], values_2[0]);
}

#[test]
fn test_calc_power_consumption() {
    let result = calc_power_consumption(&vec![
        0b00100u32, 0b11110u32, 0b10110u32, 0b10111u32, 0b10101u32, 0b01111u32, 0b00111u32,
        0b11100u32, 0b10000u32, 0b11001u32, 0b00010u32, 0b01010u32,
    ]);

    assert_eq!(
        result,
        Result {
            epsilon: 9,
            gamma: 22,
        }
    );
}

#[test]
fn test_calc_life_support_rating() {
    let (oxygen_generator_rating, c02_scrubber_rating) = calc_life_support_rating(&vec![
        0b00100u32, 0b11110u32, 0b10110u32, 0b10111u32, 0b10101u32, 0b01111u32, 0b00111u32,
        0b11100u32, 0b10000u32, 0b11001u32, 0b00010u32, 0b01010u32,
    ]);

    assert_eq!(oxygen_generator_rating, 23);
    assert_eq!(c02_scrubber_rating, 10);
}
