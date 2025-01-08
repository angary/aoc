use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

type DirectoryRef = Rc<RefCell<Directory>>;

#[derive(Clone)]
struct Directory {
    dirs: HashMap<String, DirectoryRef>,
    files: HashMap<String, u32>,
    parent: Option<DirectoryRef>,
}

impl Directory {
    pub fn new(parent: Option<DirectoryRef>) -> DirectoryRef {
        Rc::new(RefCell::new(Self {
            dirs: HashMap::new(),
            files: HashMap::new(),
            parent,
        }))
    }

    fn get_size(&self) -> u32 {
        self.files.values().sum::<u32>()
            + self
                .dirs
                .values()
                .map(|dir| dir.borrow().get_size())
                .sum::<u32>()
    }

    fn iter(&self) -> DirectoryIterator {
        DirectoryIterator {
            stack: vec![Rc::new(RefCell::new(self.clone()))],
        }
    }
}

struct DirectoryIterator {
    stack: Vec<DirectoryRef>,
}

impl Iterator for DirectoryIterator {
    type Item = DirectoryRef;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|dir| {
            let current = dir.borrow();
            self.stack.extend(current.dirs.values().cloned());
            dir.clone()
        })
    }
}

fn parse_directory(lines: &str) -> DirectoryRef {
    let root = Directory::new(None);
    let mut curr = Rc::clone(&root);
    for line in lines.lines().skip(1) {
        if line == "$ ls" {
            continue;
        } else if line.starts_with("$ cd") {
            curr = navigate_directory(&curr, line);
        } else {
            add_directory_or_file(&mut curr, line);
        }
    }
    root
}

fn navigate_directory(curr: &DirectoryRef, line: &str) -> DirectoryRef {
    let dir = line.split_whitespace().last().unwrap();
    let curr_clone = Rc::clone(curr);
    if dir == ".." {
        return curr_clone.borrow().parent.clone().unwrap();
    } else {
        return Rc::clone(curr_clone.borrow().dirs.get(dir).unwrap());
    }
}

fn add_directory_or_file(curr: &mut DirectoryRef, line: &str) {
    let (first, name) = line.split_once(' ').unwrap();
    if first == "dir" {
        let new_dir = Directory::new(Some(Rc::clone(curr)));
        curr.borrow_mut().dirs.insert(String::from(name), new_dir);
    } else {
        let file_size: u32 = first.parse().expect("failed to parse file size");
        curr.borrow_mut()
            .files
            .insert(String::from(name), file_size);
    }
}

fn task1(sizes: &[u32]) -> u32 {
    sizes.iter().filter(|&&size| size <= 100_000).sum()
}

fn task2(sizes: &[u32]) -> u32 {
    let used_bytes = sizes.iter().max().unwrap();
    let free_space_needed = used_bytes - 40_000_000;
    *sizes
        .iter()
        .filter(|&&size| size >= free_space_needed)
        .min()
        .unwrap()
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let directory = parse_directory(&input);
    let sizes: Vec<u32> = directory
        .borrow()
        .iter()
        .map(|d| d.borrow().get_size())
        .collect();
    println!("task 1: {}", task1(&sizes));
    println!("task 2: {}", task2(&sizes));
    Ok(())
}
