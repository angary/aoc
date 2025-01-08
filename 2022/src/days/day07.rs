use std::{collections::HashMap, error::Error};

#[derive(Clone, Debug)]
struct Directory {
    name: String,
    dirs: HashMap<String, Directory>,
    files: HashMap<String, usize>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

fn parse_directory(lines: &str) -> Directory {
    let mut root = Directory::new(String::from("/"));
    let mut prev = None;
    let mut curr = &mut root;

    for line in lines.lines() {
        if line.starts_with("$") {
            // Running a command
            if line.contains("cd") {
                let last_word = line.split_whitespace().last();
                let curr_reference: &mut Directory = &mut curr.clone();
                match last_word {
                    Some("..") => {
                        // Move back to the previous directory if possible
                        if let Some(parent_dir) = prev.take() {
                            curr = parent_dir; // Move to the parent directory
                        }
                    }
                    Some(dir_name) => {
                        // Navigate to a child directory
                        if let Some(next) = curr.dirs.get_mut(dir_name) {
                            prev = Some(curr); // Update previous directory
                            curr = next; // Move into the child directory
                        }
                    }
                    _ => {} // Should never happen
                };
                prev = Some(curr_reference);
            }
            // Ignore ls command
            continue;
        }
        // We are reading listed files / dirs
        let name = line
            .split_whitespace()
            .last()
            .expect("Should have a dir / file name");
        if line.starts_with("dir") {
            curr.dirs
                .insert(String::from(name), Directory::new(String::from(name)));
        } else {
            let file_size = line
                .split_whitespace()
                .next()
                .expect("Should have a file size");
            let file_size: usize = file_size.parse().expect("Failed to parse file size");
            curr.files.insert(String::from(name), file_size);
        }
    }
    root
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let directory = parse_directory(&input);
    Ok(())
}
