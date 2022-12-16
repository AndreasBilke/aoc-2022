use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Index;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();

    let segments = create_stone_segments(lines);
    let mut cave = Cave::from(segments);

    loop {
        let sand_placed = cave.place_sand((500, 0));
        if !sand_placed {
            break;
        }
    }

    cave.draw();

    // sand flows freely now. Let's count the sand corns.
    println!("Number of sand corns {}", cave.count_sand());
}

fn create_stone_segments(lines: Vec<&str>) -> Vec<StoneSegment> {
    let mut segments: Vec<StoneSegment> = Vec::new();

    for line in lines {
        let positions: Vec<&str> = line.split(" -> ").collect();
        for i in 0..positions.len() - 1 { // skip last coordinate to allow forward index access
            let pos1 = positions.index(i);
            let pos2 = positions.index(i + 1);

            let segment = StoneSegment::from(pos1, pos2);

            segments.push(segment);
        }
    }

    segments
}

#[derive(Debug)]
enum Material {
    Air,
    Sand,
    Stone
}

#[derive(Debug)]
struct Cave {
    material: HashMap<(i32, i32), Material>,
    max_y: i32
}

impl Cave {
    fn from(stone_segments: Vec<StoneSegment>) -> Self {
        // find max stone segment to compute the floor
        let max_y = stone_segments.iter().map(|s| {
            let end = match s.alignment {
                StoneAlignment::Vertical => s.start.1 + s.amount,
                _ => s.start.1
            };
            s.start.1.max(end)
        }).max().expect("No maximum found");

        let mut material: HashMap<(i32, i32), Material> = HashMap::new();

        for segment in stone_segments {
            match segment.alignment {
                StoneAlignment::Horizontal => {
                    for x in segment.start.0.min(segment.start.0 + segment.amount)..=segment.start.0.max(segment.start.0 + segment.amount) {
                        material.insert((x, segment.start.1), Material::Stone);
                    }
                },
                StoneAlignment::Vertical => {
                    for y in segment.start.1.min(segment.start.1 + segment.amount)..=segment.start.1.max(segment.start.1 + segment.amount) {
                        material.insert((segment.start.0, y), Material::Stone);
                    }
                }
            }
        }

         Cave { material, max_y }
    }

    /// Try to place a sand corn at `position`. If is was successfully placed
    /// somewhere, this method returns true. Otherwise, it means that the sand corn is blocking
    /// the entry to the cave
    ///
    /// # Arguments
    ///
    /// * `position` - The coordinate where our sand corn should come to rest.
    ///                If this is not the final position, the next possible places will be
    ///                tried.
    fn place_sand(&mut self, position: (i32, i32)) -> bool {
        let next_positions = vec![
            (position.0, position.1 + 1),
            (position.0 - 1, position.1 + 1),
            (position.0 + 1, position.1 + 1)
        ];

        for next_position in next_positions {
            if !self.position_blocked(next_position) {
                return self.place_sand(next_position);
            }
        }

        // if we reached this point, the sand corn could not flow further,
        // didn't hit the ground and can come to rest here.
        self.material.insert(position, Material::Sand);

        // stop if the entry point is reached
        position != (500, 0)
    }

    fn position_blocked(&self, position: (i32, i32)) -> bool {
        let material = match self.material.get(&position) {
            Some(x) => x,
            None => {
                match position.1 == self.max_y + 2 {
                    true => &Material::Stone,
                    _ => &Material::Air
                }
            }
        };

        match material {
            Material::Air => false,
            _ => true
        }
    }

    fn count_sand(&self) -> usize {
        let mut sand: usize = 0;

        for (_, material) in &self.material {
            sand += match material {
                Material::Sand => 1,
                _ => 0
            }
        }

        sand
    }

    fn draw(&self) {
        let mut x_values: Vec<i32> = self.material.iter().map(|x| x.0.0 ).collect();
        x_values.sort();
        let min = x_values.first().expect("No min value").clone();
        let max = x_values.last().expect("No max value").clone();

        for y in 0..=self.max_y + 1 {
            for x in min - 5..=max + 5 {
                let material_symbol = match self.material.get(&(x, y)) {
                    Some(x) => {
                        match x {
                            Material::Air => '.',
                            Material::Stone => '#',
                            Material::Sand => 'o'
                        }
                    },
                    None => '.'
                };
                print!("{}", material_symbol);
            }
            println!();
        }
        // draw floor
        for _ in min - 5..=max + 5 {
            print!("#");
        }
        println!();
    }

}

#[derive(Debug)]
enum StoneAlignment {
    Horizontal,
    Vertical
}

#[derive(Debug)]
struct StoneSegment {
    start: (i32, i32),
    amount: i32,
    alignment: StoneAlignment
}

impl StoneSegment {
    fn from(p1:&str, p2: &str) -> Self {
        let mut p1 = p1.split(",");
        let p1x: i32 = p1.next().expect("No x coord").parse().expect("X coord is not a number");
        let p1y: i32 = p1.next().expect("No y coord").parse().expect("Y coord is not a number");

        let mut p2 = p2.split(",");
        let p2x: i32 = p2.next().expect("No x coord").parse().expect("X coord is not a number");
        let p2y: i32 = p2.next().expect("No y coord").parse().expect("Y coord is not a number");

        let alignment = match p1y == p2y {
            true => StoneAlignment::Horizontal,
            _ => StoneAlignment::Vertical
        };

        let amount = match alignment {
            StoneAlignment::Horizontal => p2x - p1x,
            _ => p2y - p1y
        };

        StoneSegment { start: (p1x, p1y), amount, alignment }
    }
}
