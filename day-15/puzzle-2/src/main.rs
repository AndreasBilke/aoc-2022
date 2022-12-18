use std::collections::Bound;
use regex::Regex;
use std::env;
use std::fs;
use range_collections::{AbstractRangeSet, RangeSet, RangeSet2};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();
    let sensors = read_sensors(&lines);
    let map = Map::new(sensors);
    let tuning_frequency = map.smart_tuning_frequency();

    println!("Tuning frequency is {}", tuning_frequency);
}

fn read_sensors(lines: &Vec<&str>) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = Vec::new();

    let re = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

    for line in lines {
        let coordinates = re.captures(line).expect("No matching input");
        let sensor_x: i32 = coordinates[1].parse().expect("Not a number");
        let sensor_y: i32 = coordinates[2].parse().expect("Not a number");
        let beacon_x: i32 = coordinates[3].parse().expect("Not a number");
        let beacon_y: i32 = coordinates[4].parse().expect("Not a number");

        let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();

        sensors.push(Sensor { position: (sensor_x, sensor_y), distance});
    }

    sensors
}

#[derive(PartialEq)]
struct Beacon {
    position: (i32, i32)
}

#[derive(Debug)]
struct Sensor {
    position: (i32, i32),
    distance: i32
}

struct Map {
    sensors: Vec<Sensor>
}

impl Map {
    fn new(sensors: Vec<Sensor>) -> Self {
        Map { sensors }
    }

    fn smart_tuning_frequency(&self) -> i128 {
        let mut pos = (0, 0);
        'row_loop: for row in 0..=4000000 {
            let mut remaining_row: RangeSet2<i32> = RangeSet::from(0..4000001);
            for s in &self.sensors {
                let vertical_diff = s.position.1.abs_diff(row) as i32;
                if vertical_diff > s.distance {
                    continue;
                }
                let remaining_diff = s.distance - vertical_diff;
                let sensor_range: RangeSet2<i32> = RangeSet::from(s.position.0 - remaining_diff..s.position.0 + remaining_diff + 1);

                remaining_row = remaining_row.difference(&sensor_range);
            }

            if !remaining_row.is_empty() {
                for (x1, _) in remaining_row.iter() {
                    let x1 = match x1 {
                        Bound::Included(x) => x.clone(),
                        _ => panic!("Should not happen")
                    };

                    pos = (x1, row);
                    break 'row_loop;
                }
            }
        }

        println!("Whoooosh. I found {:?}", pos);
        pos.0 as i128 * 4000000 + pos.1 as i128
    }
}