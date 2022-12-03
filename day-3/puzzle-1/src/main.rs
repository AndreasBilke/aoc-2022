use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();

    let rucksacks = create_rucksacks(lines);
    let mut sum_priorities = 0;
    for rucksack in rucksacks {
        sum_priorities = sum_priorities + rucksack.item_priority();
    }

    println!("Sum of priorities {sum_priorities}");
}

#[derive(Debug)]
struct Rucksack {
    compartment1: String,
    compartment2: String,
}

impl Rucksack {
    fn common_character(&self) -> char {
        for item1 in self.compartment1.chars() {
            if self.compartment2.contains(item1) {
                return item1;
            }
        }

        panic!("No common items found in {} and {}", self.compartment1, self.compartment2)
    }

    fn item_priority(&self) -> i32 {
        let item = self.common_character();
        match item {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            'A' => 27,
            'B' => 28,
            'C' => 29,
            'D' => 30,
            'E' => 31,
            'F' => 32,
            'G' => 33,
            'H' => 34,
            'I' => 35,
            'J' => 36,
            'K' => 37,
            'L' => 38,
            'M' => 39,
            'N' => 40,
            'O' => 41,
            'P' => 42,
            'Q' => 43,
            'R' => 44,
            'S' => 45,
            'T' => 46,
            'U' => 47,
            'V' => 48,
            'W' => 49,
            'X' => 50,
            'Y' => 51,
            'Z' => 52,
            _   => panic!("Unsupported character")
        }
    }
}

fn create_rucksacks(lines: Vec<&str>) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    for line in lines {
        let comp1 = String::from(&line[0..line.len()/2]);
        let comp2 = String::from(&line[line.len()/2..line.len()]);

        rucksacks.push(Rucksack { compartment1: comp1, compartment2: comp2 });
    }

    return rucksacks;
}