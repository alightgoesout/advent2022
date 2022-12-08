use lazy_static::lazy_static;
use regex::Regex;
use std::iter::Peekable;

use crate::input::{read_lines, FilterNotEmpty};
use crate::Solution;

mod input;

const DEVICE_STORAGE: u32 = 70_000_000;
const UPDATE_SIZE: u32 = 30_000_000;

lazy_static! {
    static ref ROOT: Directory = Directory::parse(read_lines(input::INPUT).filter_not_empty());
}

pub struct Day7;

impl Solution for Day7 {
    fn day(&self) -> u8 {
        7
    }

    fn part_one(&self) -> String {
        format!(
            "Sum of the size of all directories under 100 000: {}",
            find_directories_with_size_under(100_000, &ROOT)
                .into_iter()
                .map(Directory::size)
                .sum::<u32>(),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Size of smallest directory to delete for update: {}",
            find_size_of_smallest_directory_to_delete_for_update(&ROOT).unwrap(),
        )
    }
}

fn find_directories_with_size_under(size: u32, root: &Directory) -> Vec<&Directory> {
    root.find_directories(|directory| directory.size() <= size)
}

fn find_size_of_smallest_directory_to_delete_for_update(root: &Directory) -> Option<u32> {
    let to_free = root.size() - (DEVICE_STORAGE - UPDATE_SIZE);
    find_directories_with_size_above(to_free, root)
        .iter()
        .map(|directory| directory.size())
        .min()
}

fn find_directories_with_size_above(size: u32, root: &Directory) -> Vec<&Directory> {
    root.find_directories(|directory| directory.size() >= size)
}

#[derive(Debug, Eq, PartialEq)]
struct Directory {
    name: String,
    items: Vec<FSItem>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn size(&self) -> u32 {
        self.items.iter().map(FSItem::size).sum()
    }

    pub fn find_directories<P: Fn(&Directory) -> bool>(&self, predicate: P) -> Vec<&Directory> {
        let mut directories = Vec::new();
        let mut to_visit = vec![self];

        while let Some(directory) = to_visit.pop() {
            for item in &directory.items {
                if let FSItem::Directory(directory) = item {
                    if predicate(directory) {
                        directories.push(directory)
                    }
                    to_visit.push(directory);
                }
            }
        }

        directories
    }

    pub fn add_file(&mut self, name: &str, size: u32) {
        self.items.push(FSItem::new_file(name, size));
    }

    pub fn add_directory(&mut self, name: &str) {
        self.items.push(FSItem::Directory(Self::new(name)));
    }

    pub fn get_directory_mut(&mut self, directory: &str) -> Option<&mut Self> {
        match self.items.iter_mut().find(|item| item.name() == directory) {
            Some(FSItem::Directory(directory)) => Some(directory),
            _ => None,
        }
    }

    pub fn parse<I: Iterator<Item = String>>(lines: I) -> Self {
        let mut root = Self::new("/");
        parse_fs(&mut root, &mut lines.peekable());
        root
    }
}

#[derive(Debug, Eq, PartialEq)]
enum FSItem {
    File { name: String, size: u32 },
    Directory(Directory),
}

impl FSItem {
    pub fn new_file(name: &str, size: u32) -> Self {
        Self::File {
            name: name.to_string(),
            size,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::File { name, .. } | Self::Directory(Directory { name, .. }) => name,
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Self::File { size, .. } => *size,
            Self::Directory(directory) => directory.size(),
        }
    }
}

lazy_static! {
    static ref CD_COMMAND: Regex = Regex::new(r"^\$ cd (\w+|\.\.)$").unwrap();
    static ref LS_COMMAND: Regex = Regex::new(r"^\$ ls$").unwrap();
    static ref FILE: Regex = Regex::new(r"^(\d+) ([\w.]+)$").unwrap();
    static ref DIRECTORY: Regex = Regex::new(r"^dir (\w+)$").unwrap();
}

fn parse_fs<I: Iterator<Item = String>>(
    current_directory: &mut Directory,
    lines: &mut Peekable<I>,
) {
    while let Some(line) = lines.next() {
        if let Some(captures) = CD_COMMAND.captures(&line) {
            let directory_name = captures.get(1).unwrap().as_str();
            if directory_name == ".." {
                return;
            } else if let Some(directory) = current_directory.get_directory_mut(directory_name) {
                parse_fs(directory, lines);
            }
        } else if LS_COMMAND.is_match(&line) {
            while let Some(line) = lines.peek() {
                if let Some(captures) = FILE.captures(line) {
                    let size = captures.get(1).unwrap().as_str().parse().unwrap();
                    let name = captures.get(2).unwrap().as_str();
                    current_directory.add_file(name, size);
                } else if let Some(captures) = DIRECTORY.captures(line) {
                    let name = captures.get(1).unwrap().as_str();
                    current_directory.add_directory(name);
                } else {
                    break;
                }
                lines.next();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{assert_eq, vec};

    const EXAMPLE: &[u8] = b"
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
7214296 k
";

    #[test]
    fn test_parse_empty() {
        assert_eq!(
            Directory::parse(std::iter::empty()),
            Directory {
                name: "/".to_string(),
                items: vec![]
            }
        )
    }

    #[test]
    fn test_parse_single_file() {
        assert_eq!(
            Directory::parse(read_lines(b"$ ls\n23 f".as_slice())),
            Directory {
                name: "/".to_string(),
                items: vec![FSItem::new_file("f", 23)]
            }
        )
    }

    #[test]
    fn parse_example() {
        let root = Directory::parse(read_lines(EXAMPLE).filter_not_empty());

        assert_eq!(
            root,
            Directory {
                name: "/".to_string(),
                items: vec![
                    FSItem::Directory(Directory {
                        name: "a".to_string(),
                        items: vec![
                            FSItem::Directory(Directory {
                                name: "e".to_string(),
                                items: vec![FSItem::new_file("i", 584)]
                            }),
                            FSItem::new_file("f", 29116),
                            FSItem::new_file("g", 2557),
                            FSItem::new_file("h.lst", 62596),
                        ],
                    }),
                    FSItem::new_file("b.txt", 14848514),
                    FSItem::new_file("c.dat", 8504156),
                    FSItem::Directory(Directory {
                        name: "d".to_string(),
                        items: vec![
                            FSItem::new_file("j", 4060174),
                            FSItem::new_file("d.log", 8033020),
                            FSItem::new_file("d.ext", 5626152),
                            FSItem::new_file("k", 7214296),
                        ]
                    })
                ],
            }
        );
    }

    #[test]
    fn part1_example() {
        let root = Directory::parse(read_lines(EXAMPLE).filter_not_empty());

        let result = find_directories_with_size_under(100_000, &root)
            .into_iter()
            .map(Directory::size)
            .sum::<u32>();

        assert_eq!(result, 95437);
    }

    #[test]
    fn part2_example() {
        let root = Directory::parse(read_lines(EXAMPLE).filter_not_empty());

        let result = find_size_of_smallest_directory_to_delete_for_update(&root);

        assert_eq!(result, Some(24933642));
    }
}
