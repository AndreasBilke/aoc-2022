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
    let line = lines[0];

    let start = match find_start_sequence(line) {
        None => panic!("No start sequence found"),
        Some(x) => x
    };
    println!("Start sequence ends at character {}", start + 1);
}

fn find_start_sequence(input: &str) -> Option<usize> {
    for i in 3..input.len() {
        let first = input.get(i - 3..=i - 3).unwrap();
        let second = input.get(i - 2..=i - 2).unwrap();
        let third = input.get(i - 1..=i - 1).unwrap();
        let fourth = input.get(i..=i).unwrap();

        if first != second && second != third && third != fourth && first != third && first != fourth && second != fourth {
            return Some(i);
        }
    }

    None
}
