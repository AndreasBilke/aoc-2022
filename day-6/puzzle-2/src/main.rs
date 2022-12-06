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
    let line = lines[0];

    let start = match find_start_sequence(line) {
        None => panic!("No start sequence found"),
        Some(x) => x
    };
    println!("Start sequence ends at character {}", start + 1);
}

fn find_start_sequence(input: &str) -> Option<usize> {
    for i in 3..input.len() {
        let characters: Vec<u8> = Vec::from(&input[i..=i+14]);

        let mut found_duplicate = false;
        let mut unique_content: HashSet<u8> = HashSet::new();
        for content in &characters {
            if unique_content.contains(&content) {
                found_duplicate = true;

                break;
            } else {
                unique_content.insert(*content);
            }
        }

        if !found_duplicate {
            return Some(i + 13);
        }
    }

    None
}
