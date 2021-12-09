fn main() {
    let entries = get_input();

    let count_digits_1478 = entries
        .iter()
        .flat_map(|e| e.output.into_iter())
        .map(|s| count_signals(&s))
        .filter(|&count| count == 2 || count == 3 || count == 4 || count == 7)
        .count();

    println!(
        "task 1: digits 1, 4, 7 and 8 appear {} times in output",
        count_digits_1478
    );

    let sum: i32 = entries.iter().map(|e| decode_output(e)).sum();

    println!("task 2: sum of output numbers = {}", sum);
}

type Signals = (bool, bool, bool, bool, bool, bool, bool);

fn count_signals(signals: &Signals) -> i32 {
    let &(a, b, c, d, e, f, g) = signals;

    let mut count = 0;
    if a {
        count += 1;
    }
    if b {
        count += 1;
    }
    if c {
        count += 1;
    }
    if d {
        count += 1;
    }
    if e {
        count += 1;
    }
    if f {
        count += 1;
    }
    if g {
        count += 1;
    }

    return count;
}

fn decode_output(entry: &Entry) -> i32 {
    use std::collections::HashMap;

    let mut codes = HashMap::new();

    // find codes by number of signals (see task 1)
    let code_1 = entry
        .patterns
        .into_iter()
        .filter(|s| count_signals(s) == 2)
        .next()
        .unwrap();
    let code_4 = entry
        .patterns
        .into_iter()
        .filter(|s| count_signals(s) == 4)
        .next()
        .unwrap();
    let code_7 = entry
        .patterns
        .into_iter()
        .filter(|s| count_signals(s) == 3)
        .next()
        .unwrap();
    let code_8 = entry
        .patterns
        .into_iter()
        .filter(|s| count_signals(s) == 7)
        .next()
        .unwrap();

    let code_9 = entry
        .patterns
        .into_iter()
        .filter(|s| count_signals(s) == 6)
        .filter(|(a, b, c, d, e, f, g)| -> bool {
            let (a4, b4, c4, d4, e4, f4, g4) = code_4;

            return (!a && !a4)
                || (!b && !b4)
                || (!c && !c4)
                || (!d && !d4)
                || (!e && !e4)
                || (!f && !f4)
                || (!g && !g4);
        })
        .next()
        .unwrap();

    let code_6 = entry
        .patterns
        .into_iter()
        .filter(|s| count_signals(s) == 6)
        .filter(|(a, b, c, d, e, f, g)| -> bool {
            let (a1, b1, c1, d1, e1, f1, g1) = code_1;

            return (!a && a1)
                || (!b && b1)
                || (!c && c1)
                || (!d && d1)
                || (!e && e1)
                || (!f && f1)
                || (!g && g1);
        })
        .next()
        .unwrap();

    let code_0 = entry
        .patterns
        .into_iter()
        .filter(|&s| count_signals(&s) == 6 && s != code_6 && s != code_9)
        .next()
        .unwrap();

    let code_5 = entry
        .patterns
        .into_iter()
        .filter(|s| count_signals(s) == 5)
        .filter(|(a, b, c, d, e, f, g)| -> bool {
            let (a6, b6, c6, d6, e6, f6, g6) = code_6;

            return (!a || a6)
                && (!b || b6)
                && (!c || c6)
                && (!d || d6)
                && (!e || e6)
                && (!f || f6)
                && (!g || g6);
        })
        .next()
        .unwrap();
    let code_3 = entry
        .patterns
        .into_iter()
        .filter(|&s| count_signals(&s) == 5 && s != code_5)
        .filter(|(a, b, c, d, e, f, g)| -> bool {
            let (a9, b9, c9, d9, e9, f9, g9) = code_9;

            return (!a || a9)
                && (!b || b9)
                && (!c || c9)
                && (!d || d9)
                && (!e || e9)
                && (!f || f9)
                && (!g || g9);
        })
        .next()
        .unwrap();

    // Last remaining with 5 1s
    let code_2 = entry
        .patterns
        .into_iter()
        .filter(|&s| count_signals(&s) == 5 && s != code_3 && s != code_5)
        .next()
        .unwrap();

    codes.insert(code_0, 0);
    codes.insert(code_1, 1);
    codes.insert(code_2, 2);
    codes.insert(code_3, 3);
    codes.insert(code_4, 4);
    codes.insert(code_5, 5);
    codes.insert(code_6, 6);
    codes.insert(code_7, 7);
    codes.insert(code_8, 8);
    codes.insert(code_9, 9);

    let mut output = 0;
    for signals in entry.output.into_iter() {
        output = output * 10 + codes.get(&signals).unwrap();
    }

    output
}

struct Entry {
    patterns: [Signals; 10],
    output: [Signals; 4],
}

fn get_input() -> Vec<Entry> {
    fn parse_signals(text: &str) -> Signals {
        let a = text.contains("a");
        let b = text.contains("b");
        let c = text.contains("c");
        let d = text.contains("d");
        let e = text.contains("e");
        let f = text.contains("f");
        let g = text.contains("g");

        return (a, b, c, d, e, f, g);
    }

    let lines = include_str!("../../inputs/day08.txt").trim().split("\n");

    lines
        .map(|l| l.split(" | "))
        .map(|mut split| Entry {
            patterns: split
                .next()
                .unwrap()
                .split(" ")
                .map(parse_signals)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap(),
            output: split
                .next()
                .unwrap()
                .split(" ")
                .map(parse_signals)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap(),
        })
        .collect()
}
