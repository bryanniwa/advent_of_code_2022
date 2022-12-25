pub fn part_one(lines: &[String]) {
    let mut file_system = Dir::default();
    let mut current = &file_system;

    for line in lines {
        if line.starts_with("$ cd") {
            let dir = line.split_whitespace().nth(2).unwrap();
            if let Some(mut node) = current.subdirs.iter().find(|x| x.name == dir) {
                current = node;
            } else {
                let mut new_dir = Dir::new(dir.to_string(), &current);
                current.as_mut().subdirs.push(new_dir);
                current = current.subdirs.last_mut().unwrap();
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
    parent: Option<&'a Box<Dir<'a>>>,
    size: usize,
    files: Vec<usize>,
    subdirs: Vec<Box<Dir<'a>>>,
}

impl Dir<'_> {
    fn default<'a>() -> Box<Dir<'a>> {
        Box::new(Dir {
            name: String::from("/"),
            parent: None,
            size: 0,
            files: Vec::new(),
            subdirs: Vec::new(),
        })
    }

    fn new<'a>(name: String, parent: &'a Box<Dir>) -> Box<Dir<'a>> {
        Box::new(Dir {
            name,
            parent: Some(parent),
            size: 0,
            files: Vec::new(),
            subdirs: Vec::new(),
        })
    }
}
