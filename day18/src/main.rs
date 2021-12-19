use std::fmt;

fn main() {
    let numbers = get_input();
    let sum = add_number_list(&numbers);
    let max_magnitude = find_largest_sum_magnitude(&numbers);

    println!("task 1: magnitude of sum = {}", sum.magnitude());
    println!(
        "task 2: largest magnitude of added pairs = {}",
        max_magnitude
    );
}

#[derive(Clone, Eq, PartialEq)]
enum Number {
    Regular(i64),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn parse(text: &str) -> Self {
        let mut stack: Vec<Number> = Vec::new();
        let mut num = String::new();
        for c in text.chars() {
            if c.is_digit(10) {
                num.push(c);
            } else if num.len() > 0 {
                stack.push(Number::Regular(num.parse().unwrap()));
                num.clear();
            }

            if c == ']' {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Number::Pair(Box::new(left), Box::new(right)));
            }
            // else if c.is_digit(10) {
            //     let value = c.to_digit(10).unwrap();
            //     stack.push(Number::Regular(value as i64));
            // }
            else {
                assert!(c == '[' || c == ',' || c.is_digit(10));
            }
        }
        // println!("size {}", stack.len());
        // for x in &stack {
        //     println!("{:?}", x);
        // }
        assert_eq!(stack.len(), 1);
        return stack.pop().unwrap();
    }

    fn parse_list<'a, I>(it: I) -> Vec<Number>
    where
        I: IntoIterator<Item = &'a str>,
    {
        it.into_iter().map(|n| Number::parse(n)).collect()
    }

    fn pair(left: Number, right: Number) -> Self {
        Number::Pair(Box::new(left), Box::new(right))
    }

    fn magnitude(&self) -> i64 {
        match self {
            Number::Regular(value) => *value,
            Number::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Regular(value) => write!(f, "{}", value),
            Number::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Regular(value) => write!(f, "{}", value),
            Number::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

fn add_numbers(left: &Number, right: &Number) -> Number {
    reduce(&Number::pair(left.clone(), right.clone()))
}

fn add_number_list(numbers: &Vec<Number>) -> Number {
    let mut sum = numbers[0].clone();
    for n in &numbers[1..] {
        sum = add_numbers(&sum, n);
    }
    return sum;
}

fn explode(number: &Number, nested: u32) -> (Number, bool, i64, i64) {
    match number {
        Number::Regular(value) => (Number::Regular(*value), false, 0, 0),
        Number::Pair(left, right) if nested >= 4 => match (left.as_ref(), right.as_ref()) {
            (Number::Regular(l), Number::Regular(r)) => (Number::Regular(0), true, *l, *r),
            _ => panic!("unexpected pair"),
        },
        Number::Pair(left, right) => {
            let (new_number_left, reduced, add_left, add_right) = explode(left, nested + 1);
            if reduced {
                (
                    Number::pair(new_number_left, add_left_most(right, add_right)),
                    true,
                    add_left,
                    0,
                )
            } else {
                let (new_number_right, reduced2, add_left, add_right) = explode(right, nested + 1);
                (
                    Number::pair(add_right_most(left, add_left), new_number_right),
                    reduced2,
                    0,
                    add_right,
                )
            }
        }
    }
}

fn split(number: &Number) -> (Number, bool) {
    match number {
        Number::Regular(value) if *value >= 10 => (
            Number::Pair(
                Box::new(Number::Regular(*value / 2)),
                Box::new(Number::Regular(*value / 2 + *value % 2)),
            ),
            true,
        ),
        Number::Regular(value) => (Number::Regular(*value), false),
        Number::Pair(left, right) => {
            let (new_number_left, reduced) = split(left);
            if reduced {
                (Number::pair(new_number_left, right.as_ref().clone()), true)
            } else {
                let (new_number_right, reduced2) = split(right);
                (
                    Number::pair(left.as_ref().clone(), new_number_right),
                    reduced2,
                )
            }
        }
    }
}

fn add_left_most(number: &Number, add: i64) -> Number {
    match number {
        Number::Regular(value) => Number::Regular(value + add),
        Number::Pair(left, right) => Number::pair(add_left_most(left, add), right.as_ref().clone()),
    }
}

fn add_right_most(number: &Number, add: i64) -> Number {
    match number {
        Number::Regular(value) => Number::Regular(value + add),
        Number::Pair(left, right) => {
            Number::pair(left.as_ref().clone(), add_right_most(right, add))
        }
    }
}

fn reduce(number: &Number) -> Number {
    let mut current_number = number.clone();
    loop {
        let (exploded_number, reduced, _, _) = explode(&current_number, 0);
        if reduced {
            current_number = exploded_number;
            continue;
        }

        let (split_number, reduced) = split(&current_number);
        if reduced {
            current_number = split_number;
            continue;
        }

        return split_number;
    }
}

fn find_largest_sum_magnitude(numbers: &Vec<Number>) -> i64 {
    let mut max_magnitude = 0;
    for (ix, x) in numbers.iter().enumerate() {
        for (iy, y) in numbers.iter().enumerate() {
            if ix != iy {
                max_magnitude = max_magnitude.max(add_numbers(x, y).magnitude());
                max_magnitude = max_magnitude.max(add_numbers(y, x).magnitude());
            }
        }
    }

    max_magnitude
}

fn get_input() -> Vec<Number> {
    Number::parse_list(include_str!("../../inputs/day18.txt").trim().lines())
}

#[test]
fn test_parse() {
    assert_eq!(
        Number::parse("[[1,9],[8,5]]"),
        Number::pair(
            Number::pair(Number::Regular(1), Number::Regular(9)),
            Number::pair(Number::Regular(8), Number::Regular(5))
        )
    );
}

#[test]
fn test_reduce() {
    assert_eq!(
        reduce(&Number::parse("[[[[[9,8],1],2],3],4]")),
        Number::parse("[[[[0,9],2],3],4]")
    );
    assert_eq!(
        reduce(&Number::parse("[7,[6,[5,[4,[3,2]]]]]")),
        Number::parse("[7,[6,[5,[7,0]]]]")
    );
    assert_eq!(
        reduce(&Number::parse("[[6,[5,[4,[3,2]]]],1]")),
        Number::parse("[[6,[5,[7,0]]],3]")
    );
    assert_eq!(
        reduce(&Number::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
        Number::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    );
}

#[test]
fn test_addition() {
    let sum = add_numbers(
        &Number::parse("[[[[4,3],4],4],[7,[[8,4],9]]]"),
        &Number::parse("[1,1]"),
    );
    assert_eq!(sum, Number::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    assert_eq!(
        add_numbers(
            &Number::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            &Number::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
        ),
        Number::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
    );
}

#[test]
fn test_add_number_list() {
    assert_eq!(
        add_number_list(&Number::parse_list(vec![
            "[1,1]", "[2,2]", "[3,3]", "[4,4]"
        ])),
        Number::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")
    );
    assert_eq!(
        add_number_list(&Number::parse_list(vec![
            "[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"
        ])),
        Number::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")
    );
    assert_eq!(
        add_number_list(&Number::parse_list(vec![
            "[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"
        ])),
        Number::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")
    );
    assert_eq!(
        add_number_list(&Number::parse_list(vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ])),
        Number::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
    );
}

#[test]
fn test_example_homework() {
    let numbers = Number::parse_list(vec![
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ]);
    let sum = add_number_list(&numbers);
    assert_eq!(
        sum,
        Number::parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
    );
    assert_eq!(sum.magnitude(), 4140);
}
