#[derive(Debug)]
pub struct MonkeyTest {
    divider: i32,
    true_monkey_id: usize,
    false_monkey_id: usize
}

impl MonkeyTest {
    pub fn from(divider_line: &str, true_line: &str, false_line: &str) -> Self {
        if !divider_line.starts_with("  Test: divisible by ") {
            panic!("Unexpected divider line");
        }

        let divider_line: Vec<&str> = divider_line.split(" ").collect();
        let divider: i32 = divider_line.last().expect("No divider found").parse().expect("Divider is not a number");

        if !true_line.starts_with("    If true: throw to monkey ") {
            panic!("Unexpected true line");
        }

        let true_line: Vec<&str> = true_line.split(" ").collect();
        let true_monkey_id: usize = true_line.last().expect("No monkey id found").parse().expect("Monkey id is not a number");

        if !false_line.starts_with("    If false: throw to monkey ") {
            panic!("Unexpected true line");
        }

        let false_line: Vec<&str> = false_line.split(" ").collect();
        let false_monkey_id: usize = false_line.last().expect("No monkey id found").parse().expect("Monkey id is not a number");

        MonkeyTest { divider, true_monkey_id, false_monkey_id }
    }

    pub fn evaluate(&self, number: i32) -> usize {
        match number % self.divider {
            0 => self.true_monkey_id,
            _ => self.false_monkey_id
        }
    }
}