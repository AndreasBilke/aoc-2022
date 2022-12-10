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

    println!("Signal strength sum {}", cpu.sum_of_signal_strengths());
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
    clock_count: usize,
    signal_strengths: Vec<i32>
}

impl Cpu {
    fn new() -> Self {
        Cpu { register_x: 1, clock_count: 1, signal_strengths: Vec::new() }
    }

    fn execute(&mut self, op: &Op) {
        match op {
            Op::Noop => self.execute_noop(),
            Op::Add(x) => self.execute_add(*x)
        }
    }

    fn execute_add(&mut self, value: i32) {
        self.track_strength();
        self.tick_clock();
        self.track_strength();
        self.tick_clock();
        self.register_x += value;
    }

    fn execute_noop(&mut self) {
        self.track_strength();
        self.tick_clock();
    }

    fn tick_clock(&mut self) {
        self.clock_count += 1;
    }

    fn track_strength(&mut self) {
        if self.clock_count < 20 {
            return;
        }

        if self.clock_count == 20 || (self.clock_count - 20) % 40 == 0 {
            self.signal_strengths.push(self.clock_count as i32 * self.register_x);
        }
    }

    fn sum_of_signal_strengths(&self) -> i32 {
        let mut sum = 0;

        for strength in &self.signal_strengths {
            sum += strength;
        }

        sum
    }
}