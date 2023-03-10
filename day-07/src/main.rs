// Copyright 2023 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::read_to_string;

const TOTAL_DISK_SPACE: u32 = 70000000;

#[derive(Debug, PartialEq)]
enum ItemType {
    File(u32),
    Directory,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct NodeId {
    index: usize,
}

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    item_type: ItemType,
    children: Vec<NodeId>,
    parent: Option<NodeId>,
}

#[derive(Debug, PartialEq)]
struct FileSystem {
    nodes: Vec<Node>,
    current_node: NodeId,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            nodes: vec![Node {
                name: String::from("/"),
                item_type: ItemType::Directory,
                children: vec![],
                parent: None,
            }],
            current_node: NodeId { index: 0 },
        }
    }

    fn new_node(&mut self, name: &str, item_type: ItemType) -> NodeId {
        let node = Node {
            name: String::from(name),
            item_type,
            children: vec![],
            parent: Some(self.current_node),
        };
        let node_id = NodeId {
            index: self.nodes.len(),
        };
        self.nodes.push(node);
        self.nodes[self.current_node.index].children.push(node_id);
        node_id
    }

    fn get_size(&self, node_id: NodeId) -> u32 {
        let node = &self.nodes[node_id.index];
        match node.item_type {
            ItemType::File(size) => size,
            ItemType::Directory => node
                .children
                .iter()
                .map(|child_id| self.get_size(*child_id))
                .sum(),
        }
    }

    fn find_size_of_directories_at_most(&self, size: u32) -> u32 {
        let mut sum = 0;
        for index in 0..self.nodes.len() {
            let item_size = self.get_size(NodeId { index });
            if ItemType::Directory == self.nodes[index].item_type && item_size <= size {
                sum += item_size
            }
        }
        sum
    }

    fn find_smallest_to_create_space(&self, size: u32) -> u32 {
        let currently_available = TOTAL_DISK_SPACE - self.get_size(NodeId { index: 0 });
        let needed_space = size - currently_available;
        let mut smallest_size = u32::MAX;
        for index in 0..self.nodes.len() {
            let item_size = self.get_size(NodeId { index });
            if ItemType::Directory == self.nodes[index].item_type
                && item_size >= needed_space
                && item_size < smallest_size
            {
                smallest_size = item_size
            }
        }
        smallest_size
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    let file_system = build_file_system(&input);
    println!(
        "Part 1: {}",
        file_system.find_size_of_directories_at_most(100000)
    );
    println!(
        "Part 2: {}",
        file_system.find_smallest_to_create_space(30000000)
    );
}

fn build_file_system(input: &str) -> FileSystem {
    let mut file_system = FileSystem::new();
    let lines: Vec<&str> = input.trim().lines().collect();
    for line in lines[1..].iter() {
        let line = line.trim();
        if "$ cd .." == line {
            file_system.current_node = file_system.nodes[file_system.current_node.index]
                .parent
                .unwrap();
        } else if line.starts_with("$ cd") {
            let line_parts: Vec<&str> = line.split_whitespace().collect();
            let node_name = line_parts[2];
            let node_id = file_system.new_node(node_name, ItemType::Directory);
            file_system.current_node = node_id;
        } else if line.starts_with("$") {
            continue;
        } else {
            let line_parts: Vec<&str> = line.split_whitespace().collect();
            let item_type = match line_parts[0] {
                "dir" => {
                    continue;
                }
                _ => ItemType::File(line_parts[0].parse().unwrap()),
            };
            let node_name = line_parts[1];
            file_system.new_node(node_name, item_type);
        }
    }
    file_system
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_file_system() {
        let expected = FileSystem {
            nodes: vec![Node {
                name: String::from("/"),
                item_type: ItemType::Directory,
                children: vec![],
                parent: None,
            }],
            current_node: NodeId { index: 0 },
        };
        assert_eq!(expected, FileSystem::new());
    }

    #[test]
    fn test_new_node() {
        let mut file_system = FileSystem::new();
        let expected = NodeId { index: 1 };
        assert_eq!(expected, file_system.new_node("a", ItemType::Directory));
        assert_eq!(2, file_system.nodes.len());
    }

    #[test]
    fn test_get_size() {
        let mut file_system = FileSystem::new();
        let node_id = file_system.new_node("a", ItemType::Directory);
        file_system.new_node("b", ItemType::File(10));
        file_system.new_node("c", ItemType::File(20));
        assert_eq!(0, file_system.get_size(node_id));
        assert_eq!(30, file_system.get_size(NodeId { index: 0 }));
    }

    #[test]
    fn test_build_file_system() {
        let input = "$ cd /
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
        let mut file_system = build_file_system(input);
        assert_eq!(14, file_system.nodes.len());
        file_system.current_node = NodeId { index: 0 };
        assert_eq!(
            4,
            file_system.nodes[file_system.current_node.index]
                .children
                .len()
        );
    }

    #[test]
    fn test_file_system_find_size_of_directories_at_most() {
        let input = "$ cd /
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
        let file_system = build_file_system(input);
        assert_eq!(95437, file_system.find_size_of_directories_at_most(100000));
    }

    #[test]
    fn test_file_system_find_smallest_to_create_space() {
        let input = "$ cd /
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
        let file_system = build_file_system(input);
        assert_eq!(
            24933642,
            file_system.find_smallest_to_create_space(30000000)
        );
    }
}
