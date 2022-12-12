use priority_queue::PriorityQueue;

use std::cmp::Reverse;
use std::collections::HashMap;
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

    let map = PuzzleMap::from(lines);

    let mut shortest_path = PuzzleShortestPath::from(map);
    let path_length = shortest_path.compute();

    println!("Shortest path has {path_length} steps");
}

#[derive(Debug)]
struct PuzzleShortestPath {
    map: PuzzleMap
}

impl PuzzleShortestPath {
    fn from(map: PuzzleMap) -> Self {
        PuzzleShortestPath { map }
    }

    fn compute(&mut self) -> i32 {
        let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
        let mut predecessor: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

        self.init(&mut distances, self.map.start);

        // initialize priority queue
        let mut queue: PriorityQueue<(i32, i32), Reverse<i32>> = PriorityQueue::new();
        for (edge, _) in &self.map.edges {
            queue.push(edge.clone(), Reverse(distances.get(&edge)
                .expect("No distance data for edge").clone()));
        }

        while queue.len() > 0 {
            let (next_node, _) = queue.pop().expect("No element in queue");
            if next_node == self.map.dest {
                break;
            }

            for edge in self.map.edges.get(&next_node).expect("No edges found") {
                let new_path = PuzzleShortestPath::relax(next_node, edge.clone(), 1, &mut distances, &mut predecessor);
                if new_path {
                    let new_dist_edge = distances.get(edge).expect("No distance found").clone();
                    queue.change_priority(edge, Reverse(new_dist_edge));
                }
            }
        }

        PuzzleShortestPath::construct_path_length(&mut predecessor, self.map.start, self.map.dest)
    }

    fn construct_path_length(predecceor: &mut HashMap<(i32, i32), (i32, i32)>, start: (i32, i32), dest: (i32, i32)) -> i32 {
        let mut path_length = 0;

        let mut current_node = dest;
        while current_node != start {
            current_node = predecceor.get(&current_node).expect("No predeccor found").clone();
            path_length += 1;
        }

        path_length
    }

    fn relax(u: (i32, i32), v: (i32, i32), distance_from_to_to: i32,
             distances: &mut HashMap<(i32, i32), i32>,
             predecessor: &mut HashMap<(i32, i32), (i32, i32)>) -> bool {
        let new_distance= distances.get(&u).expect("No distance found").clone() as i64 + distance_from_to_to as i64;
        let old_distance = distances.get(&v).expect("No distance found").clone() as i64;

        if old_distance > new_distance {
            distances.insert(v, new_distance as i32);
            predecessor.insert(v, u);

            return true;
        }

        false
    }

    fn init(&self, distances: &mut HashMap<(i32, i32), i32>, start: (i32, i32)) {
        for (edge, _) in &self.map.edges {
            distances.insert(edge.clone(), i32::MAX);
        }
        distances.insert(start, 0);
    }
}

#[derive(Debug)]
struct PuzzleMap {
    edges: HashMap<(i32, i32), Vec<(i32, i32)>>,
    start: (i32, i32),
    dest: (i32, i32)
}

impl PuzzleMap {
    fn from(lines: Vec<&str>) -> Self {
        let number_of_rows = lines.len();
        let mut edges = PuzzleMap::initialise_edges(number_of_rows as i32);

        let mut start: (i32, i32) = (0, 0);
        let mut dest: (i32, i32) = (0, 0);

        let mut heights: HashMap<(i32, i32), i32> = HashMap::new();

        for (row, line) in lines.iter().enumerate() {
            for (column, height) in line.chars().into_iter().enumerate() {
                let height = match height {
                    'S' => {
                        start = (row as i32, column as i32);

                        PuzzleMap::elevation_from_char('a')
                    },
                    'E' =>  {
                        dest = (row as i32, column as i32);

                        PuzzleMap::elevation_from_char('z')
                    },
                    x => PuzzleMap::elevation_from_char(x)
                };

                heights.insert((row as i32, column as i32), height);
            }
        }

        for (node, from_height) in &heights {
            let mut edges_for_node: Vec<(i32, i32)> = Vec::new();
            let possible_neighbours = vec![
                (node.0 - 1, node.1),
                (node.0 + 1, node.1),
                (node.0, node.1 - 1),
                (node.0, node.1 + 1)
            ];

            for n in possible_neighbours {
                let neighbour_height = match heights.get(&n) {
                    None => -1,
                    Some(x) => x.clone()
                };
                // neighbour not existent
                if neighbour_height == -1 {
                    continue;
                }

                if neighbour_height - from_height <= 1 {
                    edges_for_node.push(n);
                }
            }
            edges.insert(node.clone(), edges_for_node);
        }

        PuzzleMap { edges, start, dest }
    }

    fn initialise_edges(rows: i32) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
        let mut edges: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for row in 0..rows {
            for col in 0..rows {
                edges.insert((row, col), Vec::new());
            }
        }

        edges
    }

    fn elevation_from_char(item: char) -> i32 {
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
            _ => panic!("Unsupported character in map")
        }
    }
}
