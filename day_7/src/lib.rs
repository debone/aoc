use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Commands {
    ChangeDirectory,
    ListDirectory,
    File,
    Directory,
}

pub fn first() {
    let contents = fs::read_to_string("day_7/input.txt").unwrap();
    let mut currentDirectory = vec![];
    let mut directories = HashMap::new();
    directories.insert('/'.to_string(), 0);

    for line in contents.lines() {
        print!("");
        let command = match line {
            i if i.starts_with("$ cd") => {
                match line[5..line.len()].to_string() {
                    dir if dir == ".." => {
                        currentDirectory.pop();
                    }
                    dir @ _ => currentDirectory.push(dir),
                }
                Commands::ChangeDirectory
            }
            i if i.starts_with("$ ls") => Commands::ListDirectory,
            i if i.starts_with("dir") => {
                let dir_name = line[4..line.len()].to_string();
                directories.insert(format!("{}/{}", currentDirectory.join("/"), dir_name), 0);
                Commands::Directory
            }
            _ => {
                let (file_size, file_name) = line.split_once(' ').unwrap();
                let mut dirs = currentDirectory.clone();

                loop {
                    let dir = dirs.join("/");
                    let file_size: i32 = file_size.parse().unwrap();
                    let curr_size = directories.get(&dir).unwrap();
                    directories.insert(dir.to_string(), curr_size + file_size);
                    match dirs.pop() {
                        None | _ if dirs.len() == 0 => break,
                        _ => (),
                    };
                }
                Commands::File
            }
        };
    }

    let mut total = 0;

    for (dir_name, dir_size) in directories.iter() {
        if (dir_size <= &100000) {
            println!("adding dir {dir_name}");
            total += dir_size;
        }
    }

    // 1127800

    println!("{total}")
}

const TOTAL_SPACE: i32 = 70000000;
const UPGRADE_SIZE: i32 = 30000000;

pub fn second() {
    let contents = fs::read_to_string("day_7/input.txt").unwrap();
    let mut currentDirectory = vec![];
    let mut directories = HashMap::new();
    directories.insert('/'.to_string(), 0);

    for line in contents.lines() {
        print!("");
        let command = match line {
            i if i.starts_with("$ cd") => {
                match line[5..line.len()].to_string() {
                    dir if dir == ".." => {
                        currentDirectory.pop();
                    }
                    dir @ _ => currentDirectory.push(dir),
                }
                Commands::ChangeDirectory
            }
            i if i.starts_with("$ ls") => Commands::ListDirectory,
            i if i.starts_with("dir") => {
                let dir_name = line[4..line.len()].to_string();
                directories.insert(format!("{}/{}", currentDirectory.join("/"), dir_name), 0);
                Commands::Directory
            }
            _ => {
                let (file_size, file_name) = line.split_once(' ').unwrap();
                let mut dirs = currentDirectory.clone();

                loop {
                    let dir = dirs.join("/");
                    let file_size: i32 = file_size.parse().unwrap();
                    let curr_size = directories.get(&dir).unwrap();
                    directories.insert(dir.to_string(), curr_size + file_size);
                    match dirs.pop() {
                        None | _ if dirs.len() == 0 => break,
                        _ => (),
                    };
                }
                Commands::File
            }
        };
    }

    let used_space = TOTAL_SPACE - directories.get("/").unwrap();
    let size_needed = -1 * (used_space - UPGRADE_SIZE);

    let mut dir = TOTAL_SPACE;

    for (dir_name, dir_size) in directories.iter() {
        if (dir_size > &size_needed && dir_size <= &dir) {
            println!("smallest dir {dir_name}");
            dir = *dir_size;
        }
    }

    // 1127800

    println!("{dir}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
