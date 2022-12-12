use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
enum FileSystemItem {
    Dir { children: Vec<String> },
    File { size: usize },
}

impl FileSystemItem {
    fn get_size(&self, filesystem: &HashMap<String, FileSystemItem>) -> usize {
        match self {
            FileSystemItem::File { size } => *size,
            FileSystemItem::Dir { children } => {
                let mut total = 0;
                for child in children {
                    let child_item = filesystem.get(&child[..]).unwrap();
                    total += child_item.get_size(filesystem);
                }
                total
            }
        }
    }
}

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let filesystem = parse_contents(&contents);
    let size_of_dirs = get_size_of_dirs(&filesystem);
    println!(
        "The total size of directories less than 100,000: {}",
        size_of_dirs
    );
    let minimum_size = get_dir_to_delete(&filesystem);
    println!("Minimum directory size: {}", minimum_size);
    Ok(())
}

fn parse_contents(contents: &str) -> HashMap<String, FileSystemItem> {
    let mut filesystem: HashMap<String, FileSystemItem> = HashMap::new();
    let mut cwd = String::new();
    for line in contents.lines() {
        match line.split_whitespace().next().unwrap() {
            "$" => match line.split_whitespace().skip(1).collect::<Vec<_>>()[..] {
                ["cd", ".."] => {
                    cwd = String::from(cwd.rsplit_once('>').unwrap().0);
                }
                ["cd", "/"] => {
                    cwd = String::from("/");
                    if !filesystem.contains_key(&cwd[..]) {
                        filesystem.insert(
                            cwd.clone(),
                            FileSystemItem::Dir {
                                children: Vec::new(),
                            },
                        );
                    }
                }
                ["cd", dir_name] => {
                    let key = format!("{}>{}", cwd, dir_name);
                    cwd = key.clone();
                    if !filesystem.contains_key(&key[..]) {
                        filesystem.insert(
                            key.clone(),
                            FileSystemItem::Dir {
                                children: Vec::new(),
                            },
                        );
                    }
                }
                _ => {}
            },
            "dir" => {
                let d_name = line.split_whitespace().nth(1).unwrap();
                let key = format!("{}>{}", cwd, d_name);
                if !filesystem.contains_key(&key[..]) {
                    filesystem.insert(
                        key.clone(),
                        FileSystemItem::Dir {
                            children: Vec::new(),
                        },
                    );
                }
                add_child(&cwd, d_name, &mut filesystem);
            }
            _ => {
                let f_info = line.split_whitespace().collect::<Vec<_>>();
                let f_size = f_info[0];
                let f_name = f_info[1];
                let key = format!("{}>{}", cwd, f_name);
                if !filesystem.contains_key(&key[..]) {
                    filesystem.insert(
                        key.clone(),
                        FileSystemItem::File {
                            size: f_size.parse::<usize>().unwrap(),
                        },
                    );
                }
                add_child(&cwd, f_name, &mut filesystem);
            }
        }
    }
    filesystem
}

fn add_child<'a>(cwd: &str, child_name: &'a str, filesystem: &mut HashMap<String, FileSystemItem>) {
    let item = filesystem.get_mut(cwd).unwrap();
    let key = format!("{}>{}", cwd, child_name);
    if let FileSystemItem::Dir { children, .. } = item {
        children.push(key);
    };
}

fn get_size_of_dirs(filesystem: &HashMap<String, FileSystemItem>) -> usize {
    let mut total_size = 0;
    for key in filesystem.keys() {
        let item = filesystem.get(key).unwrap();
        let dir_size = if let FileSystemItem::Dir { children: _ } = item {
            item.get_size(filesystem)
        } else {
            0
        };
        if dir_size <= 100000 {
            total_size += dir_size;
        }
    }
    total_size
}

fn get_dir_to_delete(filesystem: &HashMap<String, FileSystemItem>) -> usize {
    const TOTAL_DISK_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;

    let space_of_root = filesystem.get("/").unwrap().get_size(filesystem);
    let free_space = TOTAL_DISK_SPACE - space_of_root;
    let required_to_delete = REQUIRED_SPACE - free_space;
    let mut min_size = space_of_root;
    for value in filesystem.values() {
        match value {
            FileSystemItem::Dir { children: _ } => {
                let dir_size = value.get_size(filesystem);
                if dir_size > required_to_delete && dir_size < min_size {
                    min_size = dir_size;
                }
            }
            FileSystemItem::File { size: _ } => {}
        }
    }
    min_size
}
