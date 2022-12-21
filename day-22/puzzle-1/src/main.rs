use regex::Regex;
use std::collections::HashMap;
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
    let monkeys = parse_monkey_lines(lines);

    let result = value_for_monkey(&"root".to_string(), &monkeys);
    println!("Result for root is {}", result);
}

fn parse_monkey_lines(lines: Vec<&str>) -> HashMap<String, Monkey> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    let re = Regex::new(r"^(\w+): ((\w| |\+|-|\*|/)+)$").unwrap();

    for line in lines {
        let parts = re.captures(line).expect("Nothing matched");
        let monkey_name = &parts[1];
        let monkey = match &parts[2].parse::<i128>() {
            Ok(number) => Monkey::from(*number),
            Err(_) => Monkey::from_str(&parts[2])
        };

        monkeys.insert(monkey_name.to_string(), monkey);
    }

    monkeys
}

fn value_for_monkey(for_name: &String, monkeys: &HashMap<String, Monkey>) -> i128 {
    let m = monkeys.get(for_name).expect("Unknown monkey name");

    match &m.operation {
        MonkeyOperation::Number(x) => x.clone(),
        MonkeyOperation::Binary(m1, op, m2) => {
            let v1 = value_for_monkey(m1, monkeys);
            let v2 = value_for_monkey(m2, monkeys);

            match op {
                MonkeyCalculation::Plus => v1 + v2,
                MonkeyCalculation::Minus => v1 - v2,
                MonkeyCalculation::Multiply => v1 * v2,
                MonkeyCalculation::Divide => v1 / v2
            }
        }
    }
}

struct Monkey {
    operation: MonkeyOperation
}

impl Monkey {
    fn from(number: i128) -> Self {
        Monkey { operation: MonkeyOperation::Number(number) }
    }

    fn from_str(compute: &str) -> Self {
        let mut parts = compute.split(" ");
        let monkey1 = parts.next().expect("No first monkey in computation");
        let operation = match parts.next().expect("No monkey operation") {
            "+" => MonkeyCalculation::Plus,
            "-" => MonkeyCalculation::Minus,
            "*" => MonkeyCalculation::Multiply,
            "/" => MonkeyCalculation::Divide,
            _ => panic!("Unknown operation")
        };
        let monkey2 = parts.next().expect("No second monkey in computation");

        Monkey { operation: MonkeyOperation::Binary(monkey1.to_string(), operation, monkey2.to_string()) }
    }
}

enum MonkeyOperation {
    Number(i128),
    Binary(String, MonkeyCalculation, String)
}

enum MonkeyCalculation {
    Plus,
    Minus,
    Multiply,
    Divide
}
