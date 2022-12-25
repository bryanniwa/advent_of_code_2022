use std::cell::RefCell;

pub fn part_one(lines: &[String]) {
    let mut file_system = Dir::default();
    let mut current = &file_system;

    for line in lines {
        if line.starts_with("$ cd") {
            let dir = line.split_whitespace().nth(2).unwrap();
            let subdirs = current.borrow().subdirs;
            if let Some(node) = subdirs.iter().find(|x| x.borrow().name == dir) {
                current = node;
            } else {
                let mut new_dir = Dir::new(dir.to_string(), &current);
                current.borrow_mut().subdirs.push(new_dir);
                current = current.borrow().subdirs.last_mut().unwrap();
            }
        }
    }
}

pub fn part_two(lines: &[String]) {
    println!("Not done yet");
}

#[derive(Debug)]
struct Dir<'a> {
    name: String,
    parent: Option<&'a RefCell<Dir<'a>>>,
    size: usize,
    files: Vec<usize>,
    subdirs: Vec<RefCell<Dir<'a>>>,
}

impl Dir<'_> {
    fn default<'a>() -> RefCell<Dir<'a>> {
        RefCell::new(Dir {
            name: String::from("/"),
            parent: None,
            size: 0,
            files: Vec::new(),
            subdirs: Vec::new(),
        })
    }

    fn new<'a>(name: String, parent: &'a RefCell<Dir>) -> RefCell<Dir<'a>> {
        RefCell::new(Dir {
            name,
            parent: Some(parent),
            size: 0,
            files: Vec::new(),
            subdirs: Vec::new(),
        })
    }
}
