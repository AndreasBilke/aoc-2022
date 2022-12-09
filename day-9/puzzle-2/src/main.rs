use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();
    let movements = create_movements(&lines);
    let mut rope = RopeState::new();

    for movement in &movements {
        rope.apply(movement);
    }

    println!("Tail moved to {} positions", rope.tail_trace.len());
}

fn create_movements(lines: &Vec<&str>) -> Vec<RopeMove> {
    let mut movements: Vec<RopeMove> = Vec::new();

    for line in lines {
        movements.push(RopeMove::from(&line));
    }

    movements
}

#[derive(Debug)]
enum RopeDirection {
    Right,
    Left,
    Up,
    Down
}

impl RopeDirection {
    fn from(direction: &str) -> Self {
        let direction = match direction {
            "R" => RopeDirection::Right,
            "L" => RopeDirection::Left,
            "U" => RopeDirection::Up,
            "D" => RopeDirection::Down,
            _ => panic!("Unknown direction")
        };

        direction
    }
}

#[derive(Debug)]
struct RopeMove {
    direction: RopeDirection,
    amount: i32
}

impl RopeMove {
    fn from(input: &str) -> Self {
        let mut parts = input.split(" ");
        let direction = parts.next().expect("Expected direction");
        let amount = parts.next().expect("Expected amount");
        let amount: i32 = amount.parse().expect("Amount is no number");

        RopeMove { direction: RopeDirection::from(direction), amount }
    }
}

#[derive(Debug)]
struct RopeState {
    ropes: [(i32, i32); 10],
    tail_trace: HashSet<(i32, i32)>
}

impl RopeState {
    fn new() -> Self {
        let mut trace: HashSet<(i32, i32)> = HashSet::new();
        trace.insert((0, 0)); // tail touches always the starting point

        let start_positions: [(i32, i32); 10] = [(0, 0); 10];
        RopeState { ropes: start_positions, tail_trace: trace }
    }

    fn apply(&mut self, movement: &RopeMove) {
        for _ in 1..=movement.amount {
            let change = match movement.direction {
                RopeDirection::Right => (1, 0),
                RopeDirection::Left => (-1, 0),
                RopeDirection::Up => (0, 1),
                RopeDirection::Down => (0, -1)
            };

            // update head
            let mut rope = self.ropes[0];
            rope.0 += change.0;
            rope.1 += change.1;
            self.ropes[0] = rope;

            for t_id in 1..self.ropes.len() {
                self.update_tail(t_id);
            }
        }
    }

    fn update_tail(&mut self, tail_id: usize) {
        let mut tail = self.ropes[tail_id];
        let successor_tail = self.ropes[tail_id - 1];

        let distance_horizontal = successor_tail.0 - tail.0;
        let distance_vertical = successor_tail.1 - tail.1;

        let point_distance = ((distance_horizontal.pow(2) + distance_vertical.pow(2)) as f32).sqrt();

        // points are close enough together
        if point_distance < 2.0 {
            return;
        }

        tail.0 += match distance_horizontal {
            0 => 0,
            x if x > 0 => 1,
            _ => -1
        };
        tail.1 += match distance_vertical {
            0 => 0,
            x if x > 0 => 1,
            _ => -1
        };

        self.ropes[tail_id] = tail;

        if tail_id < self.ropes.len() - 1 {
            return;
        }

        if !self.tail_trace.contains(&tail) {
            self.tail_trace.insert(tail);
        }
    }
}
