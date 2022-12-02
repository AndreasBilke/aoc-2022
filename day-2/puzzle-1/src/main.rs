use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();
    let plays = create_plays(lines);
    let mut play_sum = 0;
    for play in plays {
        play_sum = play_sum + play.value();
    }

    println!("Final score {play_sum}");
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissor
}

#[derive(Debug)]
struct Play {
    opponent: Move,
    me: Move
}

impl Play {
    fn new(move_a: &str, move_b: &str) -> Play {
        let opponent_move = match move_a {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissor,
            _   => panic!("Oh no, unknown move from opponent")
        };
        let own_move = match move_b {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissor,
            _   => panic!("Oh no, unknown move from me")
        };

        Play { opponent: opponent_move, me: own_move }
    }

    fn value(&self) -> i32 {
        let win_value = match self.me {
            Move::Rock => match self.opponent {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Scissor => 6
            },
            Move::Paper => match self.opponent {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Scissor => 0
            },
            Move::Scissor => match self.opponent {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Scissor => 3
            }
        };

        let move_value = match self.me {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3
        };

        return win_value + move_value;
    }
}

fn create_plays(plays_string: Vec<&str>) -> Vec<Play> {
    let mut plays: Vec<Play> = Vec::new();

    for play in plays_string {
        let mut play = play.split(" ");
        let play_opponent = match play.next() {
            Some(x) => x,
            None => panic!("Unexpected input")
        };
        let play_me = match play.next() {
            Some(x) => x,
            None => panic!("Unexpected input")
        };

        let play = Play::new(play_opponent, play_me);
        plays.push(play);
    }

    return plays;
}
