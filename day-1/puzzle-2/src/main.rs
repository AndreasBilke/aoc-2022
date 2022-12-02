use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();
    let mut elf_calories = convert_array(lines);

    elf_calories.sort();
    let sum_elf = elf_calories[elf_calories.len() -1]
        + elf_calories[elf_calories.len() -2]
        + elf_calories[elf_calories.len() -3];
    println!("Sum of max is {sum_elf}");
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
