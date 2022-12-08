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

    let trees: Vec<Vec<usize>> = create_tree_map(&lines);
    let visible_trees = count_visible_trees(&trees);
    println!("Visible trees {}", visible_trees);
}

fn count_visible_trees(trees: &Vec<Vec<usize>>) -> usize {
    let mut visible = trees.len() * 4 - 4; // map is squared, all edges are visible (don't count corners twice)

    for r in 1..trees.len() - 1 {
        for c in 1..trees.len() - 1 {
            let row = trees.get(r..=r).unwrap().first().unwrap();
            let height = row.get(c..=c).unwrap().first().unwrap().clone();

            if tree_is_visible(r, c, height, trees) {
                visible += 1;
            }
        }
    }

    visible
}

fn tree_is_visible(r: usize, c: usize, height: usize, trees: &Vec<Vec<usize>>) -> bool {
    let mut visible = vec![true, true, true, true];

    // checking row visibility
    let row = trees.get(r..=r).unwrap().first().unwrap();
    for tree_height in row.get(0..c).unwrap() {
        if *tree_height >= height {
            visible[0] = false;
            break;
        }
    }
    for tree_height in row.get(c+1..row.len()).unwrap() {
        if *tree_height >= height {
            visible[1] = false;
            break;
        }
    }

    // checking column visibility
    // If I would have an 2D Array, and not a vec of vec, accessing the column values would be much easier :(
    let mut column_values: Vec<usize> = Vec::new();
    for row_id in 0..trees.len() {
        let row = trees.get(row_id..=row_id).unwrap().first().unwrap();
        let column_value = row.get(c..=c).unwrap().first().unwrap().clone();
        column_values.push(column_value);
    }

    for tree_height in column_values.get(0..r).unwrap() {
        if *tree_height >= height {
            visible[2] = false;
            break;
        }
    }
    for tree_height in column_values.get(r+1..column_values.len()).unwrap() {
        if *tree_height >= height {
            visible[3] = false;
            break;
        }
    }

    visible[0] || visible[1] || visible[2] || visible[3]
}

fn create_tree_map(input: &Vec<&str>) -> Vec<Vec<usize>> {
    let mut map: Vec<Vec<usize>> = Vec::new();

    for line in input {
        if line.len() == 0 {
            continue;
        }

        let mut row: Vec<usize> = Vec::new();
        for char in line.split("") {
            if char == "" {
                continue;
            }

            let height: usize = char.parse().unwrap();
            row.push(height);
        }
        map.push(row);
    }

    map
}
