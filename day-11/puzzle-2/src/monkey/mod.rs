mod evaluation;
mod operation;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<i128>,
    inspections: i128,
    operation: operation::MonkeyOperation,
    test: evaluation::MonkeyTest
}

impl Monkey {
    pub fn from(lines: &mut VecDeque<&str>) -> Self {
        let id_line = lines.pop_front().expect("No monkey initiator line").to_string();
        if !id_line.starts_with("Monkey ") {
            panic!("Unexpected monkey initiator > '{}'", id_line);
        }

        let items_line = lines.pop_front().expect("Expect items line");
        let items_line = &items_line["  Starting items: ".len()..];
        let items = items_line.split(", ");
        let items: VecDeque<i128> = items.map(|x| {
            let item: i128 = x.parse().expect("No number in items line");
            item
        }).collect();

        let operation_line = lines.pop_front().expect("Expect operation line");
        let operation = operation::MonkeyOperation::from(operation_line);

        let divider_line = lines.pop_front().expect("Expect divider line");
        let true_line = lines.pop_front().expect("Expect true line");
        let false_line = lines.pop_front().expect("Expect false line");
        let test = evaluation::MonkeyTest::from(divider_line, true_line, false_line);

        Monkey { items, inspections: 0, operation, test }
    }

    pub fn play_round(&mut self, divider: i128) -> Vec<(usize, i128)> {
        let mut items: Vec<(usize, i128)> = Vec::new();

        while self.items.len() > 0 {
            self.inspections += 1;

            let item = self.items.pop_front().expect("Item expected");
            let mut item = self.operation.evaluate(item);
            item %= divider;

            let next_monkey = self.test.evaluate(item);

            items.push((next_monkey, item));
        }

        items
    }

    pub fn add_item(&mut self, item: i128) {
        self.items.push_back(item);
    }

    pub fn get_inspections(&self) -> i128 {
        self.inspections
    }

    pub fn get_test_dividor(&self) -> i128 {
        self.test.get_divider()
    }
}
