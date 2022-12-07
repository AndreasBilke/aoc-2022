use std::cell::RefCell;
use std::collections::VecDeque;
use std::env;
use std::fmt;
use std::fs;
use std::rc::Rc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();

    let root_directory = parse_input(lines);
    let free_space = 70000000 - root_directory.borrow().get_size();
    let mut directories_candiates: Vec<usize> = Vec::new();
    find_possible_dirs(root_directory, &mut directories_candiates, free_space);
    directories_candiates.sort();
    let smallest_candidate = directories_candiates.first().unwrap();
    println!("Smallest possible directory size is {}", smallest_candidate);
}

fn find_possible_dirs(start: Rc<RefCell<Directory>>, candidates: &mut Vec<usize>, free_space: usize) {
    let space_after_deletion = free_space + start.borrow().get_size();
    if space_after_deletion >= 30000000 {
        candidates.push(start.borrow().get_size());

        for dir in &start.borrow().dir_children {
            find_possible_dirs(Rc::clone(&dir), candidates, free_space);
        }
    }
}

fn handle_ls_command(remaining_lines: &mut VecDeque<&str>, cwd: Rc<RefCell<Directory>>) {
    while remaining_lines.len() > 0 && !remaining_lines.front().unwrap().starts_with("$ ") {
        let next_line = remaining_lines.pop_front().unwrap();
        let mut mut_cwd = cwd.borrow_mut();
        if next_line.starts_with("dir ") {
            let dir_name: Vec<&str> = next_line.split(" ").collect();
            let dir_name = dir_name.last().unwrap();

            mut_cwd.add_dir(Rc::new(RefCell::new(Directory::new(dir_name, Rc::clone(&cwd)))));
        } else { // this is a file
            let file: Vec<&str> = next_line.split(" ").collect();
            let size: usize = file.first().unwrap().parse().unwrap();

            mut_cwd.add_file(Rc::new(RefCell::new(File::new(size))));
        }
    }
}

fn parse_input(lines: Vec<&str>) -> Rc<RefCell<Directory>> {
    if !lines[0..=0].first().unwrap().starts_with("$ cd /") {
        panic!("Series of commands does not start with changing to root directory");
    }

    let mut lines_queue = VecDeque::from(lines);

    let root = Rc::new(RefCell::new(Directory::new_root("/")));
    let mut cwd = Rc::clone(&root);

    match lines_queue.pop_front() {
        None => panic!("No elements at all"),
        _ => ()
    }

    while !lines_queue.is_empty() {
        let line = lines_queue.pop_front().unwrap();
        if line.eq("$ ls") {
            handle_ls_command(&mut lines_queue, Rc::clone(&cwd));
        } else if line.starts_with("$ cd") {
            let command: Vec<&str> = line.split(" ").collect();
            let dir = command.last().unwrap();

            let dir = match *dir {
                ".." => match &cwd.borrow_mut().parent {
                    None => Rc::clone(&root),
                    Some(x) => Rc::clone(&x)
                }
                _ =>  cwd.borrow_mut().get_dir_by_name(dir).unwrap()
            };
            cwd = Rc::clone(&dir);
        }
    }

    root
}

struct Directory {
    parent: Option<Rc<RefCell<Directory>>>,
    dir_children: Vec<Rc<RefCell<Directory>>>,
    file_children: Vec<Rc<RefCell<File>>>,
    name: String
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Directory")
            .field("dir_children", &self.dir_children.len())
            .field("file_children", &self.file_children.len())
            .field("name", &self.name)
            .finish()
    }
}

impl Directory {
    fn new(name: &str, parent: Rc<RefCell<Directory>>) -> Self {
        Directory {
            parent: Some(parent),
            dir_children: Vec::new(),
            file_children: Vec::new(),
            name: String::from(name)
        }
    }

    fn new_root(name: &str) -> Self {
        Directory {
            parent: None,
            dir_children: Vec::new(),
            file_children: Vec::new(),
            name: String::from(name)
        }
    }

    fn add_file(&mut self, child: Rc<RefCell<File>>) {
        self.file_children.push(child);
    }

    fn add_dir(&mut self, child: Rc<RefCell<Directory>>) {
        self.dir_children.push(child);
    }

    fn get_dir_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Directory>>> {
        match self.dir_children.iter().find(|d| d.borrow().name == name) {
            None => None,
            Some(x) => Some(x.clone())
        }
    }

    fn get_size(&self) -> usize {
        let mut size: usize = 0;

        for child in &self.file_children {
            let child = child.borrow();
            size += child.get_size();
        }

        for child in &self.dir_children {
            let child = child.borrow();
            size += child.get_size();
        }

        size
    }
}

#[derive(Debug)]
struct File {
    size: usize
}

impl File {
    fn new(size: usize) -> Self {
        File { size }
    }

    fn get_size(&self) -> usize {
        self.size
    }
}