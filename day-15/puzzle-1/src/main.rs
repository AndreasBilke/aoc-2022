use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

// für die Lösung reicht es aus in einem großen Suchradius (-40k < x < +40k) für jeden Punkt
// für jeden Sensor zu schauen ob er erreichbar wäre

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
    let mut map = Map::new(sensors, beacons);
    map.build_beacon_map();

    let no_beacon_count = map.get_no_beacons_in_row(2000000);
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
        beacons.push(Beacon { position: (beacon_x, beacon_y)});
    }

    (sensors, beacons)
}

struct Beacon {
    position: (i32, i32)
}

struct Sensor {
    position: (i32, i32),
    distance: i32
}

struct Map {
    sensors: Vec<Sensor>,
    beacons: Vec<Beacon>,
    no_beacons: HashSet<(i32, i32)>
}

impl Map {
    fn new(sensors: Vec<Sensor>, beacons: Vec<Beacon>) -> Self {
        Map { sensors, beacons, no_beacons: HashSet::new() }
    }

    fn build_beacon_map(&mut self) {
        // compute from sensor with distance all points
        for sensor in &self.sensors {
            for y in 0..=sensor.distance {
                for x in sensor.position.0 - sensor.distance + y..=sensor.position.0 + sensor.distance - y {
                    let no_beacon_pos1 = (x, sensor.position.1 + y);
                    let no_beacon_pos2 = (x, sensor.position.1 - y); // mirrored on y axis

                    let dist1 = (sensor.position.0 - no_beacon_pos1.0).abs() + (sensor.position.1 - no_beacon_pos1.1).abs();
                    let dist2 = (sensor.position.0 - no_beacon_pos2.0).abs() + (sensor.position.1 - no_beacon_pos2.1).abs();

                    assert!(dist1 <= sensor.distance);
                    assert!(dist2 <= sensor.distance);

                    self.no_beacons.insert(no_beacon_pos1);
                    self.no_beacons.insert(no_beacon_pos2);
                }
            }
        }

        for sensor in &self.sensors {
            self.no_beacons.remove(&sensor.position);
        }

        for beacon in &self.beacons {
            self.no_beacons.remove(&beacon.position);
        }
    }

    fn get_no_beacons_in_row(&self, row: i32) -> usize {
        self.no_beacons.iter().map(|pos|
            return if pos.1 == row {
                1
            } else {
                0
            }
        ).sum()
    }

    fn draw(&self) {
        let min_x = self.no_beacons.iter().map(|pos| pos.0 ).min().expect("No min found");
        let max_x = self.no_beacons.iter().map(|pos| pos.0 ).max().expect("No max found");

        let min_y = self.no_beacons.iter().map(|pos| pos.1 ).min().expect("No min found");
        let max_y = self.no_beacons.iter().map(|pos| pos.1 ).max().expect("No max found");

        let sensor_pos: Vec<(i32, i32)> = self.sensors.iter().map(|s| s.position).collect();
        let beacon_pos: Vec<(i32, i32)> = self.beacons.iter().map(|b| b.position).collect();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let mut symbol = '.';

                if self.no_beacons.contains(&(x, y)) {
                    symbol = '#';
                } else if sensor_pos.contains(&(x, y)) {
                    symbol = 'S';
                } else if beacon_pos.contains(&(x, y)) {
                    symbol = 'B';
                }

                print!("{}", symbol);
            }
            println!();
        }
    }
}