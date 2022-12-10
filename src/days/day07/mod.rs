use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
struct Directory {
    sub_directories: HashMap<String, Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
    total_size: u32,
}

impl Directory {
    /// Adds a given file size to the directory's total size and updates all parent directories.
    fn add_file_size(&mut self, size: u32) {
        self.total_size += size;

        if let Some(parent) = &mut self.parent {
            parent.borrow_mut().add_file_size(size);
        }
    }
    fn empty_with_parent(parent: Option<Rc<RefCell<Directory>>>) -> Self {
        Directory {
            sub_directories: HashMap::new(),
            parent,
            total_size: 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    const MAX_DIRECTORY_SIZE: u32 = 100_000;

    let file_system = construct_file_system(input);
    let dirs = dirs_with_size_constraint(file_system, |size| size <= MAX_DIRECTORY_SIZE);

    Some(dirs.iter().map(|dir| dir.borrow().total_size).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    const DISK_SPACE: u32 = 70_000_000;
    const REQUIRED_UNUSED_SPACE: u32 = 30000000;

    let file_system = construct_file_system(input);
    let remaining = DISK_SPACE - file_system.borrow().total_size;
    let required_dir_size = REQUIRED_UNUSED_SPACE - remaining;
    let dirs = dirs_with_size_constraint(file_system, |size| size >= required_dir_size);

    Some(dirs.iter().map(|d| d.borrow().total_size).min().unwrap())
}

fn construct_file_system(input: &str) -> Rc<RefCell<Directory>> {
    // The root '/'
    let file_system = Rc::new(RefCell::new(Directory::empty_with_parent(None)));

    // The current directory
    let mut curr_dir = file_system.clone();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "$" {
            let command = parts[1];

            if command == "cd" {
                let path = parts[2];
                let old_dir = curr_dir.clone();

                curr_dir = if path == "/" {
                    file_system.clone()
                } else if path == ".." {
                    old_dir.borrow().parent.as_ref().unwrap().clone()
                } else {
                    old_dir.borrow().sub_directories.get(path).unwrap().clone()
                };
            }
        } else if parts[0] == "dir" {
            let name = parts[1].to_string();
            let old_dir = curr_dir.clone();
            let new_dir = Rc::new(RefCell::new(Directory::empty_with_parent(Some(old_dir))));
            curr_dir.borrow_mut().sub_directories.insert(name, new_dir);
        } else {
            let file_size = parts[0].parse::<u32>().unwrap();
            curr_dir.borrow_mut().add_file_size(file_size);
        }
    }

    file_system
}

fn dirs_with_size_constraint<F: FnOnce(u32) -> bool + Clone + Copy>(
    dir: Rc<RefCell<Directory>>,
    constraint: F,
) -> Vec<Rc<RefCell<Directory>>> {
    let mut dirs = vec![];
    for sub_dir in dir.borrow().sub_directories.values() {
        if constraint(sub_dir.borrow().total_size) {
            dirs.push(sub_dir.clone());
        }
        dirs.extend(dirs_with_size_constraint(sub_dir.clone(), constraint));
    }
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), Some(95437));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), Some(24933642));
    }
}
