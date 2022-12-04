use std::env;
use std::fs;
use ranges;
use ranges::{GenericRange, OperationResult};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }

    let input = &args[1];
    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();

    let section_pairs = create_section_pairs(lines);

    let mut number_full_overlaps = 0;
    for section_pair in &section_pairs {
        if section_pair.full_overlap() {
            number_full_overlaps = number_full_overlaps + 1;
        }
    }

    println!("Number of full overlaps {}", number_full_overlaps);
}

fn create_section_pairs(lines: Vec<&str>) -> Vec<SectionPair> {
    let mut pairs: Vec<SectionPair> = Vec::new();

    for line in lines {
        let pair = SectionPair::new(line);
        pairs.push(pair);
    }

    pairs
}

#[derive(Debug)]
struct SectionPair {
    first_elf: GenericRange<i32>,
    second_elf: GenericRange<i32>
}

impl SectionPair {
    fn new(pair_item: &str) -> Self {
        let ranges: Vec<&str> = pair_item.split(",").collect();
        if ranges.len() != 2 {
            panic!("Expected to ranges for {}", pair_item);
        }

        let first_range: Vec<&str> = ranges[0].split("-").collect();
        let first_range_start: i32 = first_range[0].parse().unwrap();
        let first_range_end: i32 = first_range[1].parse().unwrap();

        let second_range: Vec<&str> = ranges[1].split("-").collect();
        let second_range_start: i32 = second_range[0].parse().unwrap();
        let second_range_end: i32 = second_range[1].parse().unwrap();

        let first_range = GenericRange::from(first_range_start..=first_range_end);
        let second_range = GenericRange::from(second_range_start..=second_range_end);

        SectionPair { first_elf: first_range, second_elf: second_range}
    }

    fn full_overlap(&self) -> bool {
        let intersection = self.first_elf & self.second_elf;
        let contains = match intersection {
            OperationResult::Empty => false,
            OperationResult::Single(x) => self.first_elf.is_equal(&x) || self.second_elf.is_equal(&x),
            _ => false
        };

        contains
    }
}