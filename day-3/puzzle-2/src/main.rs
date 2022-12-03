use array_tool::vec::Intersect;
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

    for i in 0..rucksacks.len() / 3 {
        let elf1 = &rucksacks[i * 3];
        let elf2 = &rucksacks[i * 3 + 1];
        let elf3 = &rucksacks[i * 3 + 2];

        let common_1_2 = elf1.common_items(elf2);
        let common_1_3 = elf1.common_items(elf3);
        let common_2_3 = elf2.common_items(elf3);

        let common_1_2_1_3 = common_1_2.intersect(common_1_3);
        let common_all = common_1_2_1_3.intersect(common_2_3);

        if common_all.len() != 1 {
            panic!("Found more than 1 common items for line {}", i * 3);
        }

        sum_priorities = sum_priorities + Rucksack::item_priority(common_all[0]);
    }

    println!("Sum of priorities {sum_priorities}");
}

#[derive(Debug)]
struct Rucksack {
    items: String
}

impl Rucksack {
    fn common_items(&self, other: &Rucksack) -> Vec<char> {
        let mut common_items: Vec<char> = Vec::new();
        for item in self.items.chars() {
            if other.items.contains(item) {
                common_items.push(item);
            }
        }

        return common_items;
    }

    fn item_priority(item: char) -> i32 {
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
        let items = String::from(line);

        rucksacks.push(Rucksack { items });
    }

    return rucksacks;
}