fn main() {
    let lines = get_input();
    let error_score = get_error_score(&lines);
    let auto_complete_scores = get_auto_complete_scores(&lines);

    println!("task 1: total syntax error score = {}", error_score);
    println!(
        "task 2: median auto complete score = {}",
        median(&auto_complete_scores)
    );
}

fn get_error_score<T>(lines: &[T]) -> i32
where
    T: AsRef<str>,
{
    lines
        .iter()
        .map(|l| check_line(l.as_ref()))
        .map(|e| match e {
            Some(SyntaxError::IllegalCharacter(')')) => 3,
            Some(SyntaxError::IllegalCharacter(']')) => 57,
            Some(SyntaxError::IllegalCharacter('}')) => 1197,
            Some(SyntaxError::IllegalCharacter('>')) => 25137,
            None | Some(SyntaxError::MissingCharacters(_)) => 0,
            _ => panic!("invalid character"),
        })
        .sum()
}

fn get_auto_complete_scores<T>(lines: &[T]) -> Vec<u64>
where
    T: AsRef<str>,
{
    lines
        .iter()
        .map(|l| check_line(l.as_ref()))
        .map(|e| match e {
            Some(SyntaxError::MissingCharacters(cs)) => cs.iter().fold(0, |sum, c| {
                sum * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("invalid character"),
                    }
            }),
            None | Some(SyntaxError::IllegalCharacter(_)) => 0,
        })
        .filter(|&s| s > 0)
        .collect()
}

fn median(scores: &[u64]) -> u64 {
    use std::collections::BinaryHeap;

    let mut heap = scores.iter().collect::<BinaryHeap<_>>();
    for _ in 0..scores.len() / 2 {
        heap.pop();
    }

    return **heap.peek().unwrap();
}

enum SyntaxError {
    IllegalCharacter(char),
    MissingCharacters(Vec<char>),
}

fn check_line(line: &str) -> Option<SyntaxError> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' if stack.last().map(|&l| c == l).unwrap_or(false) => {
                stack.pop();
            }
            _ => return Some(SyntaxError::IllegalCharacter(c)),
        }
    }

    if stack.len() > 0 {
        stack.reverse();
        Some(SyntaxError::MissingCharacters(stack))
    } else {
        None
    }
}

fn get_input() -> Vec<String> {
    include_str!("../../inputs/day10.txt")
        .trim()
        .split("\n")
        .map(|s| s.to_owned())
        .collect()
}

#[test]
fn test_get_error_score() {
    assert_eq!(
        get_error_score(&vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]"
        ]),
        26397
    );
}

#[test]
fn test_get_auto_complete_scores() {
    assert_eq!(
        get_auto_complete_scores(&vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]"
        ]),
        vec![288957, 5566, 1480781, 995444, 294,]
    );
}
