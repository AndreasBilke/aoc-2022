use regex::Regex;
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
    let (sensors, beacons) = read_sensors(&lines);
    let map = Map::new(sensors, beacons);
    let no_beacon_count = map.count_no_beacons(2000000);

    println!("In y=2000000 there are {} beacon free zones", no_beacon_count);
}

fn read_sensors(lines: &Vec<&str>) -> (Vec<Sensor>, Vec<Beacon>) {
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: Vec<Beacon> = Vec::new();

    let re = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

    for line in lines {
        let coordinates = re.captures(line).expect("No matching input");
        let sensor_x: i32 = coordinates[1].parse().expect("Not a number");
        let sensor_y: i32 = coordinates[2].parse().expect("Not a number");
        let beacon_x: i32 = coordinates[3].parse().expect("Not a number");
        let beacon_y: i32 = coordinates[4].parse().expect("Not a number");

        let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();

        sensors.push(Sensor { position: (sensor_x, sensor_y), distance});
        let beacon = Beacon { position: (beacon_x, beacon_y)};
        if !beacons.contains(&beacon) {
            beacons.push(beacon);
        }
    }

    (sensors, beacons)
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

impl Sensor {
    fn reachable(&self, other: (i32, i32)) -> bool {
        let dist = (self.position.0 - other.0).abs() + (self.position.1 - other.1).abs();

        dist <= self.distance
    }
}

struct Map {
    sensors: Vec<Sensor>,
    beacons: Vec<Beacon>,
    x_search_range: (i32, i32)
}

impl Map {
    fn new(sensors: Vec<Sensor>, beacons: Vec<Beacon>) -> Self {
        let min = sensors.iter().map(|s| {
           s.position.0 - s.distance
        }).min().expect("No min x found");

        let max = sensors.iter().map(|s| {
            s.position.0 + s.distance
        }).max().expect("No max x found");

        Map { sensors, beacons, x_search_range: (min, max) }
    }

    fn count_no_beacons(&self, row: i32) -> i32 {
        let mut no_beacons = 0;

        for x in self.x_search_range.0..=self.x_search_range.1 {
            let p = (x, row);
            let mut reachable = false;

            for s in &self.sensors {
                if s.reachable(p) {
                    reachable = true;

                    break;
                }
            }

            if reachable {
                no_beacons += 1;
            }
        }

        let num_sensor_row: i32 = self.sensors.iter().map(|s| {
            return if s.position.1 == row {
                1
            } else {
                0
            }
        }).sum();

        let num_beacons_row: i32 = self.beacons.iter().map(|b| {
            return if b.position.1 == row {
                1
            } else {
                0
            }
        }).sum();

        no_beacons - num_sensor_row - num_beacons_row
    }
}