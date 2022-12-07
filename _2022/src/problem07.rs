use std::{collections::HashMap, fmt::Write, fs, path::PathBuf};

const MAX_COUNT: usize = 100_000;
const MAX_CAP: usize = 70000000;
const MIN_CAP: usize = 30000000;

fn main() {
    let data =
        fs::read_to_string("data/problem07.txt").expect("Could not find part one input file");
    let mut path = PathBuf::new();
    let mut dir = FileSystem::new();
    for cmd in data.split_terminator("$ ").skip(1) {
        if &cmd[..1] == "c" {
            match cmd[3..].trim() {
                ".." => {
                    path.pop();
                }
                s => {
                    path.push(s);
                }
            }
        } else if &cmd[..1] == "l" {
            for mut line in cmd[3..].lines().map(|l| l.split_terminator(" ")) {
                match line.next().unwrap() {
                    "dir" => {
                        let name = line.next().unwrap().to_string();
                        dir.add_entry(&mut path, FsEntry::Dir(0), name)
                    }
                    s => match s.parse::<usize>() {
                        Ok(num) => {
                            let name = line.next().unwrap().to_string();
                            dir.add_entry(&mut path, FsEntry::File(num), name)
                        }
                        Err(_) => {
                            unreachable!("Unknown entry type: {s:?}");
                        }
                    },
                }
            }
        } else {
            unreachable!("Unknown command: {cmd:?}");
        }
    }
    /* ------ Part 1 ------ */
    println!("Part 1) The count is: {}", dir.part_one_count());
    println!("Part 2) The count is: {}", dir.part_two_count());
}

#[derive(Debug, Clone)]
enum FsEntry {
    // Size of file
    File(usize),
    // The directory's name and its size (repeated data)
    Dir(usize),
}

#[derive(Debug)]
struct FileSystem {
    root: HashMap<String, FsEntry>,
}

impl FileSystem {
    fn new() -> Self {
        let mut root = HashMap::with_capacity(1);
        root.insert("/".into(), FsEntry::Dir(0));
        Self { root }
    }

    fn part_one_count(&self) -> usize {
        self.root
            .values()
            .filter_map(|entry| {
                if let FsEntry::Dir(count) = entry {
                    (*count <= MAX_COUNT).then_some(count)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part_two_count(&self) -> usize {
        let min_cap = MIN_CAP - (MAX_CAP - self.root.get("/").unwrap().count());
        self.root
            .values()
            .filter_map(|entry| {
                if let FsEntry::Dir(count) = entry {
                    (*count >= min_cap).then_some(*count)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }

    fn add_entry(&mut self, path: &mut PathBuf, entry: FsEntry, entry_name: String) {
        let count = match &entry {
            FsEntry::Dir(..) => 0,
            FsEntry::File(count) => *count,
        };
        path.push(entry_name);
        let mut name = String::new();
        for f in path.iter() {
            write!(name, "{}", f.to_str().unwrap()).unwrap();
            match self.root.get_mut(&name) {
                Some(FsEntry::Dir(ref mut size)) => {
                    *size += count;
                }
                None => {
                    self.root.insert(name, entry);
                    break;
                }
                Some(FsEntry::File(..)) => {
                    unreachable!("Can not overwrite file!!");
                }
            }
        }
        path.pop();
    }
}

impl FsEntry {
    fn count(&self) -> usize {
        match self {
            FsEntry::File(count) => *count,
            FsEntry::Dir(count) => *count,
        }
    }
}
