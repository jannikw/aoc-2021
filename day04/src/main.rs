use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let (drawn_numbers, boards) = get_input();
    let (sum, number) = find_first_winning_board(&drawn_numbers, boards.clone())
        .expect("failed to find first winning board");

    let (sum_2, number_2) = find_last_winning_board(&drawn_numbers, boards.clone())
        .expect("failed to find last winning board");

    println!(
        "part 1: sum of unmarked numbers = {}, last drawn number = {} -> {}",
        sum,
        number,
        sum * number
    );
    println!(
        "part 2: sum of unmarked numbers = {}, last drawn number = {} -> {}",
        sum_2,
        number_2,
        sum_2 * number_2
    );
}

fn find_first_winning_board(
    drawn_numbers: &Vec<i32>,
    mut boards: Vec<BingoBoard>,
) -> Option<(i32, i32)> {
    for &n in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark_number(n);

            if board.is_done() {
                let unmarked_numbers = board.get_unmarked_numbers();
                let sum = unmarked_numbers.iter().sum();

                return Some((sum, n));
            }
        }
    }

    return None;
}

fn find_last_winning_board(
    drawn_numbers: &Vec<i32>,
    mut boards: Vec<BingoBoard>,
) -> Option<(i32, i32)> {
    for &n in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark_number(n);
        }

        if boards.len() == 1 && boards[0].is_done() {
            let board = &boards[0];
            let unmarked_numbers = board.get_unmarked_numbers();
            let sum = unmarked_numbers.iter().sum();

            return Some((sum, n));
        } else {
            boards.retain(|b| !b.is_done());
        }
    }

    return None;
}

#[derive(Debug, PartialEq, Clone)]
struct BingoBoard {
    fields: [[(i32, bool); 5]; 5],
}

impl BingoBoard {
    pub fn new(fields: [[i32; 5]; 5]) -> Self {
        let mut marked_fields = [[(0, false); 5]; 5];

        for i in 0..5 {
            for h in 0..5 {
                marked_fields[i][h] = (fields[i][h], false);
            }
        }

        return BingoBoard {
            fields: marked_fields,
        };
    }

    pub fn parse(text: &str) -> Result<BingoBoard, Box<dyn std::error::Error>> {
        let re = Regex::new(concat!(
            r" ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)\n",
            r" ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)\n",
            r" ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)\n",
            r" ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)\n",
            r" ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)  ?(\d+)",
        ))?;
        let captures = re
            .captures(text)
            .ok_or(ParseError::new("failed to parse board"))?;
        let mut numbers = [[0; 5]; 5];

        for i in 0..5 {
            for h in 0..5 {
                numbers[i][h] = captures
                    .get(i * 5 + h + 1)
                    .ok_or(ParseError::new("failed to parse number on board"))?
                    .as_str()
                    .parse()?;
            }
        }

        return Ok(BingoBoard::new(numbers));
    }

    fn is_done(&self) -> bool {
        'outer_rows: for i in 0..5 {
            for h in 0..5 {
                let (_, marked) = self.fields[i][h];
                if !marked {
                    continue 'outer_rows;
                }
            }

            return true;
        }

        'outer_cols: for i in 0..5 {
            for h in 0..5 {
                let (_, marked) = self.fields[h][i];
                if !marked {
                    continue 'outer_cols;
                }
            }

            return true;
        }

        return false;
    }

    fn mark_number(&mut self, number: i32) {
        for i in 0..5 {
            for h in 0..5 {
                let (field_number, _) = self.fields[i][h];
                if number == field_number {
                    self.fields[i][h] = (number, true);
                }
            }
        }
    }

    fn get_unmarked_numbers(&self) -> Vec<i32> {
        let mut numbers = Vec::new();
        for i in 0..5 {
            for h in 0..5 {
                let (number, marked) = self.fields[i][h];
                if !marked {
                    numbers.push(number);
                }
            }
        }

        return numbers;
    }
}

#[derive(Debug, Clone)]
struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: &str) -> Self {
        return ParseError {
            message: message.to_owned(),
        };
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "failed to parse bingo board: {}", self.message)
    }
}

impl std::error::Error for ParseError {}

fn get_input() -> (Vec<i32>, Vec<BingoBoard>) {
    let file = File::open("../inputs/day04.txt").expect("failed to open input file");
    let lines = BufReader::new(file).lines();

    let mut drawn_numbers: Option<Vec<i32>> = None;
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut board_lines: Vec<String> = Vec::new();
    for line in lines {
        if drawn_numbers.is_none() {
            drawn_numbers = Some(
                line.expect("failed to read line")
                    .split(",")
                    .map(|n| n.parse().expect("failed to parse drawn number"))
                    .collect(),
            );
        } else {
            board_lines.push(line.expect("failed to read line"));

            if board_lines.len() == 6 {
                let board =
                    BingoBoard::parse(&board_lines.join("\n")).expect("failed to parse board");
                boards.push(board);
                board_lines.clear();
            }
        }
    }

    return (drawn_numbers.unwrap(), boards);
}

#[test]
fn test_parse() {
    let board_text = "64 19 39 69 90
41  5 59 37 42
75 95 58 89 92
20  3 85 48 71
31 94 11 18 70";
    let board = BingoBoard::parse(board_text).unwrap();

    assert_eq!(
        board,
        BingoBoard::new([
            [64, 19, 39, 69, 90],
            [41, 5, 59, 37, 42],
            [75, 95, 58, 89, 92],
            [20, 3, 85, 48, 71],
            [31, 94, 11, 18, 70]
        ])
    );
}
