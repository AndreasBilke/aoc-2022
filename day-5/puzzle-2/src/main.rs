use itertools::Itertools;
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

    let mut stacks: HashMap<i32, Vec<String>> = create_stacks(get_number_stacks(&lines));
    fill_initial_stacks(&lines, &mut stacks);

    let commands = create_move_commands(&lines);
    for command in commands {
        command.execute(&mut stacks);
    }

    for (_, v) in stacks.iter().sorted_by_key(|x| x.0) {
        print!("{}", v.last().unwrap());
    }
    println!();
}

fn fill_initial_stacks(lines: &Vec<&str>, stacks: &mut HashMap<i32, Vec<String>>) {
    for line in lines {
        for (index, character) in line.chars().enumerate() {
            if character == '[' {
                let item = line.chars().nth(index + 1).expect("Whoops. Invalid array access");
                let stack_number = (index as i32 + 1) / 4 + 1;

                let stack = stacks.get_mut(&stack_number).expect("Stack with ID {stack_number} should be here");
                stack.push(item.to_string());
            }
        }
    }

    for (_stack_id, stack) in stacks {
        stack.reverse();
    }
}

fn create_stacks(number: i32) -> HashMap<i32, Vec<String>> {
    let mut stacks: HashMap<i32, Vec<String>> = HashMap::new();

    for i in 1..=number {
        let stack: Vec<String> = Vec::new();

        stacks.insert(i, stack);
    }

    stacks
}

fn get_number_stacks(lines: &Vec<&str>) -> i32 {
    for line in lines {
        if line.starts_with(" 1 ") {
            let stack_ids: Vec<&str> = line.split_whitespace().collect();
            let highest_id: i32 = stack_ids[stack_ids.len() - 1].parse().unwrap();

            return highest_id
        }
    }

    0
}

fn create_move_commands(lines: &Vec<&str>) -> Vec<MoveCommand> {
    let mut commands: Vec<MoveCommand> = Vec::new();

    for line in lines {
        if line.starts_with("move") {
            commands.push(MoveCommand::new(line));
        }
    }

    commands
}

#[derive(Debug)]
struct MoveCommand {
    number_creates: i32,
    from_stack: i32,
    to_stack: i32
}

impl MoveCommand {
    fn new(input: &str) -> Self {
        let input: Vec<&str> = input.split(" ").collect();
        let number: i32 = input[1].parse().unwrap();
        let from: i32 = input[3].parse().unwrap();
        let to: i32 = input[5].parse().unwrap();

        MoveCommand { number_creates: number, from_stack: from, to_stack: to }
    }

    fn execute(&self, stacks: &mut HashMap<i32, Vec<String>>) {
        let mut from_values: Vec<String> = Vec::new();

        for _ in 1..=self.number_creates {
            from_values.push(stacks.get_mut(&self.from_stack).unwrap().pop().unwrap());
        }

        from_values.reverse();
        stacks.get_mut(&self.to_stack).unwrap().extend_from_slice(&from_values);
    }
}
