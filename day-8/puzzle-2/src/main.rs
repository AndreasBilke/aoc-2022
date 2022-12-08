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
    let scenic_count = count_scenic_view(&trees);
    println!("Scenic count {}", scenic_count);
}

fn count_scenic_view(trees: &Vec<Vec<usize>>) -> usize {
    let mut scenic_views: Vec<usize> = Vec::new();

    for r in 0..trees.len() {
        for c in 0..trees.len() {
            let row = trees.get(r..=r).unwrap().first().unwrap();
            let height = row.get(c..=c).unwrap().first().unwrap().clone();

            let visible_trees = count_visible_trees(r, c, height, trees);
            let scenic_view = visible_trees.0 * visible_trees.1 * visible_trees.2 * visible_trees.3;
            scenic_views.push(scenic_view);
        }
    }

    scenic_views.sort();
    scenic_views.last().unwrap().clone()
}

fn count_visible_trees(r: usize, c: usize, height: usize, trees: &Vec<Vec<usize>>) -> (usize, usize, usize, usize) {
    let mut visible = (0, 0, 0, 0);

    // checking row visibility
    let row = trees.get(r..=r).unwrap().first().unwrap();
    for tree_height in row.get(0..c).unwrap().iter().rev() {
        visible.0 += 1;
        if *tree_height >= height {
            break;
        }
    }
    for tree_height in row.get(c+1..row.len()).unwrap().iter() {
        visible.1 += 1;
        if *tree_height >= height {
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
    for tree_height in column_values.get(0..r).unwrap().iter().rev() {
        visible.2 += 1;
        if *tree_height >= height {
            break;
        }
    }
    for tree_height in column_values.get(r+1..column_values.len()).unwrap().iter() {
        visible.3 += 1;
        if *tree_height >= height {
            break;
        }
    }

    visible
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
