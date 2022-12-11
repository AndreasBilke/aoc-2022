mod monkey;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::rc::Rc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();

    let jungle = Jungle::from(lines);

    for _ in 1..=10000 {
        jungle.play_round();
    }

    let mut inspections: Vec<i128> = Vec::new();
    for monkey in &jungle.monkeys {
        inspections.push(monkey.borrow_mut().get_inspections());
    }

    inspections.sort();
    let last_number = inspections.pop().expect("Number expected");
    let second_to_last_number = inspections.pop().expect("Number expected");

    let result = last_number * second_to_last_number;

    println!("Monkey result is {result}");
}
#[derive(Debug)]
pub struct Jungle {
    monkeys: Vec<Rc<RefCell<monkey::Monkey>>>,
    divider_product: i128
}

impl Jungle {
    fn from(lines: Vec<&str>) -> Self {
        let mut lines = VecDeque::from(lines);
        let mut divider = 1 as i128;

        let mut monkeys: Vec<Rc<RefCell<monkey::Monkey>>> = Vec::new();
        while lines.len() > 0 {
            // remove empty lines between monkey blocks
            if lines.front().unwrap().len() == 0 {
                lines.pop_front();
            }

            let monkey = monkey::Monkey::from(&mut lines);
            divider *= monkey.get_test_dividor();
            monkeys.push(Rc::new(RefCell::new(monkey)));
        }

        Jungle { monkeys, divider_product: divider }
    }

    fn play_round(&self) {
        for monkey in &self.monkeys {
            let mut from_monkey = monkey.borrow_mut();
            let items = from_monkey.play_round(self.divider_product);

            for (next_monkey, item) in items {
                let mut next_monkey= self.monkeys.get(next_monkey)
                    .expect("Expect monkey")
                    .borrow_mut();

                next_monkey.add_item(item);
            }
        }
    }
}

