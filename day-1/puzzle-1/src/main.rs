use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();
    let elf_calories = convert_array(lines);

    let max_calories = elf_calories.iter().max();
    match max_calories {
        Some (max) => println!("Max value is {max}"),
        None => println!("List is empty")
    }
}

fn convert_array(initial_array: Vec<&str>) -> Vec<i32> {
    let mut converted_array: Vec<i32> = Vec::new();

    let mut current_elf = 0;
    for item in initial_array {
        if item.eq("") {
            converted_array.push(current_elf);
            current_elf = 0;
            continue;
        }

        let number: i32 = match item.parse() {
            Ok(value) => value,
            Err(_) => panic!("Ups. Cannot convert {item}")
        };
        current_elf = current_elf + number;
    }

    return converted_array;
}
