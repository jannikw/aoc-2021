fn main() {
    let positions = get_input();

    let (pos, costs) = find_cheapest_alignment(&positions, false);
    let (pos2, costs2) = find_cheapest_alignment(&positions, true);

    println!("task 1: cheapest alignment at {} for {} fuel", pos, costs);
    println!("task 2: cheapest alignment at {} for {} fuel", pos2, costs2);
}

fn get_input() -> Vec<i32> {
    return include_str!("../../inputs/day07.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
}

fn find_cheapest_alignment(positions: &Vec<i32>, part2: bool) -> (i32, i32) {
    let &max = positions.iter().max().unwrap_or(&1);
    let mut cheapest_pos = 0;
    let mut cheapest_costs = None;
    for i in 0..max + 1 {
        let costs: i32 = positions
            .iter()
            .map(|p| i32::abs(i - p))
            .map(|c| -> i32 {
                if part2 {
                    ((c + 1) as f32 / 2.0 * (c as f32)) as i32
                } else {
                    c
                }
            })
            .sum();
        if cheapest_costs.is_none() || cheapest_costs.unwrap() > costs {
            cheapest_pos = i;
            cheapest_costs = Some(costs);
        }
    }

    return (cheapest_pos, cheapest_costs.unwrap());
}

#[test]
fn test_find_cheapest_alignment() {
    assert_eq!(
        find_cheapest_alignment(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], false),
        (2, 37)
    );
    assert_eq!(
        find_cheapest_alignment(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], true),
        (5, 168)
    );
}
