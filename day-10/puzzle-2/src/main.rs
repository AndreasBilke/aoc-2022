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

    let mut ops: Vec<Op> = Vec::new();
    for line in lines {
        ops.push(Op::from(line));
    }

    let mut cpu = Cpu::new();
    for op in &ops {
        cpu.execute(op);
    }
}

#[derive(Debug)]
enum Op {
    Add(i32),
    Noop
}

impl Op {
    fn from(command: &str) -> Self {
        let mut command= command.split(" ");
        let op = command.nth(0).expect("No command found");

        match op {
            "addx" => {
                let amount = command.nth(0).expect("No value passed");
                let amount: i32 = amount.parse().expect("No i32 value passed");

                Op::Add(amount)
            },
            "noop" => Op::Noop,
            _ => panic!("Unknown cpu command")
        }
    }
}

#[derive(Debug)]
struct Cpu {
    register_x: i32,
    clock_count: usize
}

impl Cpu {
    fn new() -> Self {
        Cpu { register_x: 1, clock_count: 0 }
    }

    fn execute(&mut self, op: &Op) {
        match op {
            Op::Noop => self.execute_noop(),
            Op::Add(x) => self.execute_add(*x)
        }
    }

    fn execute_add(&mut self, value: i32) {
        self.draw_crt();
        self.tick_clock();
        self.draw_crt();
        self.tick_clock();
        self.register_x += value;
    }

    fn execute_noop(&mut self) {
        self.draw_crt();
        self.tick_clock();
    }

    fn tick_clock(&mut self) {
        self.clock_count += 1;
    }

    fn draw_crt(&self) {
        if self.clock_count % 40 == 0 {
            println!();
        }

        self.draw_pixel();
    }

    fn draw_pixel(&self) {
        let mut character = '.';
        let count = (self.clock_count % 40) as i32;

        if count >= (self.register_x - 1) && count <= (self.register_x + 1) {
            character = '#';
        }

        print!("{character}");
    }
}