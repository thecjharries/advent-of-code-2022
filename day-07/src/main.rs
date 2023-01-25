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
    // let mut file_system = FileSystem::new();
    // let mut lines: Vec<&str> = input.trim().lines().collect();
    // for line in lines[1..].iter() {
    //     let line = line.trim();
    //     if "$ cd .." == line {
    //         file_system.current_node = file_system.nodes[file_system.current_node.index].parent.unwrap();
    //     } else if line.starts_with("$ cd") {
    //         let line_parts: Vec<&str> = line.split_whitespace().collect();
    //         let node_name = line_parts[2];

    //     }
    // }
    todo!()
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
        let file_system = build_file_system(input);
        assert_eq!(13, file_system.nodes.len());
    }
}
