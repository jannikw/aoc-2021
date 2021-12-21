fn main() {
    let (player_a, player_b) = play_game(10 - 1, 1 - 1);

    let losing_score = if player_a.has_won() {
        player_b.score
    } else {
        player_a.score
    };
    let rolls = player_a.rolls + player_b.rolls;

    println!(
        "task 1: losing score and total dice rolls -> {} * {} = {}",
        losing_score,
        rolls,
        losing_score * rolls
    );

    let (wins_a, wins_b) = play_game_quantum(PlayerQ::new(10 - 1), PlayerQ::new(1 - 1));
    println!(
        "task 2: most wins with quantum die = {}",
        wins_a.max(wins_b)
    );
}

struct DeterministicDice {
    current: i32,
}

impl DeterministicDice {
    fn start(start: i32) -> DeterministicDice {
        assert!(start >= 1 && start <= 100);
        DeterministicDice { current: start - 1 }
    }
}

impl Iterator for DeterministicDice {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let result = self.current + 1;
        self.current = (self.current + 1) % 100;
        Some(result)
    }
}

struct Player {
    position: i32,
    score: i32,
    rolls: i32,
}

impl Player {
    fn new(position: i32) -> Self {
        Player {
            position,
            score: 0,
            rolls: 0,
        }
    }

    fn has_won(&self) -> bool {
        self.score >= 1000
    }

    fn turn(&mut self, dice: &mut DeterministicDice) {
        self.position =
            (self.position + dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap())
                % 10;
        self.score += self.position + 1;
        self.rolls += 3;
    }
}

fn play_game(player_a_start: i32, player_b_start: i32) -> (Player, Player) {
    let mut player_a = Player::new(player_a_start);
    let mut player_b = Player::new(player_b_start);
    let mut dice = DeterministicDice::start(1);

    while !player_a.has_won() && !player_b.has_won() {
        player_a.turn(&mut dice);
        if !player_a.has_won() {
            player_b.turn(&mut dice);
        }
    }

    return (player_a, player_b);
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct PlayerQ {
    pos: u8,
    score: u8,
}
impl PlayerQ {
    fn new(starting_pos: u8) -> Self {
        PlayerQ {
            pos: starting_pos,
            score: 0,
        }
    }

    fn turn(&self, roll: u8) -> Self {
        let new_pos = (self.pos + roll) % 10;
        PlayerQ {
            pos: new_pos,
            score: self.score + new_pos + 1,
        }
    }

    fn has_won(&self) -> bool {
        self.score >= 21
    }
}

static ROLLS_QUANTITIES: &'static [(u8, u64)] =
    &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play_game_quantum(player_a: PlayerQ, player_b: PlayerQ) -> (u64, u64) {
    use std::collections::HashMap;

    type WinMap = HashMap<(PlayerQ, PlayerQ), (u64, u64)>;

    let mut win_map: WinMap = HashMap::new();

    fn get(mut win_map: &mut WinMap, player_a: PlayerQ, player_b: PlayerQ) -> (u64, u64) {
        if player_a.has_won() || player_b.has_won() {
            panic!();
        }

        match win_map.get(&(player_a, player_b)) {
            Some(&(wins_a, wins_b)) => (wins_a, wins_b),
            None => {
                let result = calc(&mut win_map, player_a, player_b);
                win_map.insert((player_a, player_b), result);
                return result;
            }
        }
    }

    fn calc(win_map: &mut WinMap, player_a: PlayerQ, player_b: PlayerQ) -> (u64, u64) {
        let mut wins_a = 0;
        let mut wins_b = 0;
        for &(roll_a, qty_a) in ROLLS_QUANTITIES {
            let player_a_new = player_a.turn(roll_a);
            if player_a_new.has_won() {
                wins_a += qty_a;
                continue;
            }

            for &(roll_b, qty_b) in ROLLS_QUANTITIES {
                let player_b_new = player_b.turn(roll_b);
                if player_b_new.has_won() {
                    wins_b += qty_a * qty_b;
                    continue;
                }

                let (rec_wins_a, rec_wins_b) = get(win_map, player_a_new, player_b_new);
                wins_a += qty_a * qty_b * rec_wins_a;
                wins_b += qty_a * qty_b * rec_wins_b;
            }
        }

        return (wins_a, wins_b);
    }

    return get(&mut win_map, player_a, player_b);
}

#[test]
fn test_game() {
    let (player_a, player_b) = play_game(4 - 1, 8 - 1);
    assert_eq!(player_a.has_won(), true);
    assert_eq!(player_b.score, 745);
    assert_eq!(player_a.rolls + player_b.rolls, 993);
}

#[test]
fn test_game_quantum() {
    let (wins_a, wins_b) = play_game_quantum(PlayerQ::new(4 - 1), PlayerQ::new(8 - 1));
    assert_eq!(wins_a, 444356092776315);
    assert_eq!(wins_b, 341960390180808);
}
