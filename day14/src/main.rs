use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let (input, element_map) = get_input();

    // task 1: work with slower algorithm
    let mut step = input.chars().collect();
    for _ in 0..10 {
        step = perform_insertions(&element_map, &step);
    }
    let (most_common, least_common) = count_elements(&step);
    println!(
        "task 1: most common - least common element after 10 steps = {}",
        most_common - least_common
    );

    // task 2: work with faster algorithm, avoiding memory exhaustion
    let mut pairs = count_pairs(&input.chars().collect());
    for _ in 0..40 {
        pairs = perform_insertions_fast(&element_map, &pairs);
    }
    let (most_common_40, least_common_40) = count_elements_from_pairs(&pairs);
    println!(
        "task 2: most common - least common element after 40 steps = {}",
        most_common_40 - least_common_40
    );
}

type ElementMap = HashMap<(char, char), char>;
type ElementPairs = Vec<(char, char, u64)>;

fn perform_insertions(element_map: &ElementMap, input: &Vec<char>) -> Vec<char> {
    let mut result = Vec::new();
    for i in 0..input.len() - 1 {
        let a = input[i];
        let b = input[i + 1];
        result.push(a);
        result.push(*element_map.get(&(a, b)).unwrap());
    }
    result.push(*input.last().unwrap());

    assert_eq!(result.len(), input.len() * 2 - 1);

    result
}

fn perform_insertions_fast(element_map: &ElementMap, input_pairs: &ElementPairs) -> ElementPairs {
    let mut result = HashMap::new();
    for &(a, b, count) in input_pairs {
        let &c = element_map.get(&(a, b)).unwrap();
        *result.entry((a, c)).or_insert(0) += count;
        *result.entry((c, b)).or_insert(0) += count;
    }

    result
        .iter()
        .map(|(&(a, b), &count)| (a, b, count))
        .collect()
}

fn count_pairs(input: &Vec<char>) -> ElementPairs {
    let mut result = HashMap::new();
    for i in 0..input.len() - 1 {
        let a = input[i];
        let b = input[i + 1];
        *result.entry((a, b)).or_insert(0) += 1;
    }

    result
        .iter()
        .map(|(&(a, b), &count)| (a, b, count))
        .collect()
}

fn count_elements(input: &Vec<char>) -> (u64, u64) {
    let mut counts = vec![0; 26];
    for &c in input {
        counts[c as usize - 'A' as usize] += 1;
    }

    return (
        *counts.iter().max().unwrap(),
        *counts.iter().filter(|&&c| c > 0).min().unwrap(),
    );
}

fn count_elements_from_pairs(input: &ElementPairs) -> (u64, u64) {
    let mut counts = vec![0; 26];
    for &(a, b, count) in input {
        counts[a as usize - 'A' as usize] += count;
        counts[b as usize - 'A' as usize] += count;
    }

    for count in &mut counts {
        // if count is uneven, it is either the first or last letter in original input
        if *count % 2 == 1 {
            *count += 1;
        }
        *count /= 2;
    }

    return (
        *counts.iter().max().unwrap(),
        *counts.iter().filter(|&&c| c > 0).min().unwrap(),
    );
}

fn get_input() -> (String, ElementMap) {
    let mut element_map = ElementMap::new();
    let mut start = None;
    let re = Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();

    for line in include_str!("../../inputs/day14.txt").trim().split("\n") {
        if start.is_none() {
            start.replace(line);
        } else if let Some((from, to)) =
            re.captures(line)
                .and_then(|captures| -> Option<((char, char), char)> {
                    let from = (
                        captures.get(1)?.as_str().chars().next()?,
                        captures.get(2)?.as_str().chars().next()?,
                    );
                    let to = captures.get(3)?.as_str().chars().next()?;
                    Some((from, to))
                })
        {
            element_map.insert(from, to);
        } else {
            assert_eq!(line, "");
        }
    }

    return (start.unwrap().to_string(), element_map);
}

#[test]
fn test_slow_and_fast() {
    let (input, element_map) = get_input();
    let mut step = input.chars().collect();
    let mut pairs = count_pairs(&input.chars().collect());
    for _ in 0..10 {
        step = perform_insertions(&element_map, &step);
        pairs = perform_insertions_fast(&element_map, &pairs);
        // Both implementations should come to the same result
        assert_eq!(count_elements(&step), count_elements_from_pairs(&pairs));
    }
}
