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

impl Node {
    fn get_size(&self, nodes: &Vec<Node>) -> u32 {
        match self.item_type {
            ItemType::File(size) => size,
            ItemType::Directory => {
                let mut size = 0;
                for child in self.children.iter() {
                    size += nodes[child.index].get_size(nodes);
                }
                size
            }
        }
    }
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
        node_id
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", input);
    // println!("Part 2: {}", input);
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
            file_system.nodes[file_system.current_node.index]
                .children
                .push(node_id);
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
            let node_id = file_system.new_node(node_name, item_type);
            file_system.nodes[file_system.current_node.index]
                .children
                .push(node_id);
        }
    }
    file_system
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_get_size() {
        let nodes = vec![
            Node {
                name: String::from("/"),
                item_type: ItemType::Directory,
                children: vec![NodeId { index: 1 }],
                parent: None,
            },
            Node {
                name: String::from("a"),
                item_type: ItemType::Directory,
                children: vec![NodeId { index: 2 }],
                parent: Some(NodeId { index: 0 }),
            },
            Node {
                name: String::from("b"),
                item_type: ItemType::File(10),
                children: vec![],
                parent: Some(NodeId { index: 1 }),
            },
        ];
        assert_eq!(10, nodes[0].get_size(&nodes));
        assert_eq!(10, nodes[1].get_size(&nodes));
        assert_eq!(10, nodes[2].get_size(&nodes));
    }

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
}
